use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use backoff::ExponentialBackoff;
use futures_util::future::select;
use srs::{
    ClientDisconnectMessage, Message, Modulation, Packet, RadioUpdateMessage,
    ServerSettingsMessage, StreamError, SyncMessage, UpdateMessage,
};
use stubs::common::v0::Unit;
use stubs::mission::v0::StreamEventsResponse;
use stubs::mission::v0::stream_events_response::{Event, SrsConnectEvent, SrsDisconnectEvent};
use tokio::sync::RwLock;
use tonic::{Code, Status};

use crate::fps::event_time;
use crate::rpc::MissionRpc;
use crate::shutdown::ShutdownHandle;

#[derive(Clone, Default)]
pub struct SrsClients {
    pub clients: Arc<RwLock<HashMap<String, srs::message::Client>>>,
}

pub async fn run_in_background(
    rpc: MissionRpc,
    clients: SrsClients,
    config: crate::config::SrsConfig,
    shutdown_handle: ShutdownHandle,
) {
    let client = srs::Client::new(
        "DCS-gRPC",
        256_000_000, // freq doesn't really matter here
        srs::Coalition::Spectator,
    );

    let addr = config
        .addr
        .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5002));

    let backoff = ExponentialBackoff {
        initial_interval: Duration::from_secs(30),
        max_interval: Duration::from_secs(5 * 60),
        max_elapsed_time: None, // never stop trying
        ..Default::default()
    };

    select(
        Box::pin(backoff::future::retry_notify(
            backoff,
            // on each try, run the program and consider every error as transient (ie. worth
            // retrying)
            || async {
                run(
                    rpc.clone(),
                    clients.clone(),
                    addr,
                    client.clone(),
                    shutdown_handle.clone(),
                )
                .await
                .map_err(backoff::Error::transient)
            },
            // error hook:
            |err, backoff: Duration| {
                log::debug!(
                    "retrying with backoff {:.2}s after error: {err}",
                    backoff.as_secs_f64(),
                );
            },
        )),
        shutdown_handle.signal(),
    )
    .await;
}

fn aggregate_frequencies(clients: &HashMap<String, srs::message::Client>) -> HashMap<u32, HashSet<u64>> {
    let mut map: HashMap<u32, HashSet<u64>> = HashMap::new();
    for c in clients.values() {
        if let Some(radio) = &c.radio_info {
            if radio.unit_id != 0 && radio.unit != "CA" {
                let freqs = map.entry(radio.unit_id).or_default();
                for r in &radio.radios {
                    if matches!(r.modulation, Modulation::Am | Modulation::Fm) {
                        freqs.insert(r.freq as u64);
                    }
                }
            }
        }
    }
    map
}

async fn run(
    rpc: MissionRpc,
    clients: SrsClients,
    addr: SocketAddr,
    client: srs::Client,
    shutdown_handle: ShutdownHandle,
) -> Result<(), StreamError> {
    let (_, mut rx) = client.start(addr, shutdown_handle.signal()).await?;
    while let Some(p) = rx.recv().await {
        let Packet::Control(msg) = p? else {
            continue;
        };
        log::info!("srs msg: {msg:#?}");

        let mut clients_guard = clients.clients.write().await;
        let before = aggregate_frequencies(&clients_guard);
        let mut changed = false;

        match msg {
            Message::Sync(SyncMessage { clients: sync_clients, .. }) => {
                clients_guard.clear();
                for c in sync_clients {
                    clients_guard.insert(c.client_guid.clone(), c);
                }
                changed = true;
            }
            Message::Update(UpdateMessage { client, .. })
            | Message::RadioUpdate(RadioUpdateMessage { client, .. }) => {
                clients_guard.insert(client.client_guid.clone(), client);
                changed = true;
            }
            Message::ClientDisconnect(ClientDisconnectMessage { client, .. }) => {
                clients_guard.remove(&client.client_guid);
                changed = true;
            }
            Message::ServerSettings(ServerSettingsMessage { server_settings, .. }) => {
                if server_settings.get("SHOW_TUNED_COUNT").is_none_or(|s| s != "True") {
                    log::warn!(
                        "`Show Tuned/Client Count` is disabled on your SRS server. \
                         Enable it if you want to receive the frequencies your SRS clients are on."
                    )
                }
            }
            Message::Ping(_) | Message::VersionMismatch(_) => {}
        }

        if changed {
            let after = aggregate_frequencies(&clients_guard);
            drop(clients_guard);

            for (unit_id, after_freqs) in &after {
                let before_freqs = before.get(unit_id);
                let mut unit_cache = None;
                for freq in after_freqs {
                    if before_freqs.is_none() || !before_freqs.unwrap().contains(freq) {
                        unit_cache = connected(&rpc, unit_cache, *unit_id, *freq).await;
                    }
                }
            }
            for (unit_id, before_freqs) in &before {
                let after_freqs = after.get(unit_id);
                let mut unit_cache = None;
                for freq in before_freqs {
                    if after_freqs.is_none() || !after_freqs.unwrap().contains(freq) {
                        unit_cache = disconnected(&rpc, unit_cache, *unit_id, *freq).await;
                    }
                }
            }
        }
    }

    Ok(())
}

async fn connected(
    rpc: &MissionRpc,
    unit: Option<Unit>,
    unit_id: u32,
    frequency: u64,
) -> Option<Unit> {
    let unit = if let Some(unit) = unit {
        unit
    } else {
        match get_unit_by_id(rpc, unit_id).await {
            Ok(unit) => unit,
            Err(err) => {
                if err.code() != Code::NotFound {
                    log::error!("failed to get unit by id for srs connect event: {}", err);
                }
                return None;
            }
        }
    };

    rpc.event(StreamEventsResponse {
        time: event_time(),
        event: Some(Event::SrsConnect(SrsConnectEvent {
            unit: Some(unit.clone()),
            frequency,
        })),
    })
    .await;

    Some(unit)
}

async fn disconnected(
    rpc: &MissionRpc,
    unit: Option<Unit>,
    unit_id: u32,
    frequency: u64,
) -> Option<Unit> {
    let unit = if let Some(unit) = unit {
        unit
    } else {
        match get_unit_by_id(rpc, unit_id).await {
            Ok(unit) => unit,
            Err(err) => {
                if err.code() != Code::NotFound {
                    log::error!("failed to get unit by id for srs disconnect event: {}", err);
                }
                return None;
            }
        }
    };

    rpc.event(StreamEventsResponse {
        time: event_time(),
        event: Some(Event::SrsDisconnect(SrsDisconnectEvent {
            unit: Some(unit.clone()),
            frequency,
        })),
    })
    .await;

    Some(unit)
}

async fn get_unit_by_id(rpc: &MissionRpc, id: u32) -> Result<Unit, Status> {
    #[derive(serde::Serialize)]
    struct GetUnitByIdRequest {
        id: u32,
    }
    #[derive(Debug, serde::Deserialize)]
    struct GetUnitByIdResponse {
        unit: Unit,
    }

    let res: GetUnitByIdResponse = rpc
        .request(
            "getUnitById",
            tonic::Request::new(GetUnitByIdRequest { id }),
        )
        .await?;
    Ok(res.unit)
}
