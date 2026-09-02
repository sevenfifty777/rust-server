use stubs::recovery;
use stubs::recovery::v0::recovery_service_server::RecoveryService;
use tonic::{Request, Response, Status};

use super::MissionRpc;
use crate::authentication::ClientIdentity;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StartTelemetryParams {
    owner: String,
    recovery_handle: String,
    aircraft_name: String,
    aircraft_id: u32,
    carrier_name: String,
    carrier_id: u32,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ReadTelemetryParams {
    owner: String,
    recovery_handle: String,
    expected_source_epoch: String,
    after_sequence: u64,
    limit: u32,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StopTelemetryParams {
    owner: String,
    recovery_handle: String,
    expected_source_epoch: String,
}

#[allow(clippy::result_large_err)] // Preserve tonic's established Status return type.
fn owner<T>(request: &Request<T>) -> Result<String, Status> {
    request
        .extensions()
        .get::<ClientIdentity>()
        .map(|identity| identity.0.clone())
        .ok_or_else(|| Status::unauthenticated("missing authenticated client identity"))
}

#[tonic::async_trait]
impl RecoveryService for MissionRpc {
    async fn get_recovery_snapshot(
        &self,
        request: Request<recovery::v0::GetRecoverySnapshotRequest>,
    ) -> Result<Response<recovery::v0::GetRecoverySnapshotResponse>, Status> {
        let res = self.request("getRecoverySnapshot", request).await?;
        Ok(Response::new(res))
    }

    async fn start_recovery_telemetry(
        &self,
        request: Request<recovery::v0::StartRecoveryTelemetryRequest>,
    ) -> Result<Response<recovery::v0::StartRecoveryTelemetryResponse>, Status> {
        let owner = owner(&request)?;
        let input = request.into_inner();
        let params = StartTelemetryParams {
            owner,
            recovery_handle: input.recovery_handle,
            aircraft_name: input.aircraft_name,
            aircraft_id: input.aircraft_id,
            carrier_name: input.carrier_name,
            carrier_id: input.carrier_id,
        };
        let res = self
            .request("startRecoveryTelemetry", Request::new(params))
            .await?;
        Ok(Response::new(res))
    }

    async fn read_recovery_telemetry(
        &self,
        request: Request<recovery::v0::ReadRecoveryTelemetryRequest>,
    ) -> Result<Response<recovery::v0::ReadRecoveryTelemetryResponse>, Status> {
        let owner = owner(&request)?;
        self.check_recovery_read_quota(&owner)?;
        let input = request.into_inner();
        let params = ReadTelemetryParams {
            owner,
            recovery_handle: input.recovery_handle,
            expected_source_epoch: input.expected_source_epoch,
            after_sequence: input.after_sequence,
            limit: input.limit,
        };
        let res = self
            .request("readRecoveryTelemetry", Request::new(params))
            .await?;
        Ok(Response::new(res))
    }

    async fn stop_recovery_telemetry(
        &self,
        request: Request<recovery::v0::StopRecoveryTelemetryRequest>,
    ) -> Result<Response<recovery::v0::StopRecoveryTelemetryResponse>, Status> {
        let owner = owner(&request)?;
        let input = request.into_inner();
        let params = StopTelemetryParams {
            owner,
            recovery_handle: input.recovery_handle,
            expected_source_epoch: input.expected_source_epoch,
        };
        let res = self
            .request("stopRecoveryTelemetry", Request::new(params))
            .await?;
        Ok(Response::new(res))
    }
}
