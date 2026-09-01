use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use mlua::{Lua, LuaSerdeExt, Result as LuaResult, SerializeOptions, Value};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio_stream::Stream;
use tokio_stream::wrappers::ReceiverStream;

const DEFAULT_QUEUE_CAPACITY: usize = 4_096;

type Queue = Arc<Mutex<VecDeque<Box<dyn Request + Send + Sync>>>>;
type Subscriptions<E> = Arc<Mutex<Vec<mpsc::Sender<E>>>>;

pub struct PendingRequest<P, R>
where
    P: Serialize,
    for<'de> R: Deserialize<'de>,
{
    id: u64,
    method: String,
    params: Option<P>,
    enqueued_at: Instant,
    queue_depth_at_enqueue: usize,
    queue_depth_at_dequeue: usize,
    tx: Option<oneshot::Sender<Response<R>>>,
}

pub trait Request {
    fn id(&self) -> u64;
    fn method(&self) -> &str;
    fn queue_wait(&self) -> Duration;
    fn queue_depth_at_enqueue(&self) -> usize;
    fn queue_depth_at_dequeue(&self) -> usize;
    fn set_queue_depth_at_dequeue(&mut self, depth: usize);
    fn is_cancelled(&self) -> bool;
    fn params(&self, lua: &Lua) -> Result<Option<Value>, mlua::Error>;
    fn success(&mut self, lua: &Lua, value: &Value) -> Result<(), mlua::Error>;
    fn error(&mut self, error: String, kind: Option<String>);
}

pub struct IPC<E> {
    queue: Queue,
    queue_capacity: usize,
    next_request_id: Arc<AtomicU64>,
    discarded_cancelled: Arc<AtomicU64>,
    subscriptions: Subscriptions<E>,
}

impl<E> IPC<E> {
    pub fn with_queue_capacity(queue_capacity: usize) -> Self {
        assert!(queue_capacity > 0, "IPC queue capacity must be nonzero");
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            queue_capacity,
            next_request_id: Arc::new(AtomicU64::new(1)),
            discarded_cancelled: Arc::new(AtomicU64::new(0)),
            subscriptions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn try_next(&self) -> Option<Box<dyn Request + Send + Sync>> {
        let Ok(mut queue) = self.queue.try_lock() else {
            return None;
        };
        while let Some(mut request) = queue.pop_front() {
            if request.is_cancelled() {
                self.discarded_cancelled.fetch_add(1, Ordering::Relaxed);
                continue;
            }
            request.set_queue_depth_at_dequeue(queue.len());
            return Some(request);
        }
        None
    }

    pub fn queue_len(&self) -> Option<usize> {
        self.queue.try_lock().ok().map(|queue| queue.len())
    }

    pub fn take_discarded_cancelled(&self) -> u64 {
        self.discarded_cancelled.swap(0, Ordering::Relaxed)
    }

    pub async fn event(&self, event: E)
    where
        E: Clone + std::fmt::Debug,
    {
        let mut clients = self.subscriptions.lock().await;
        clients.retain_mut(move |tx| match tx.try_send(event.clone()) {
            Ok(_) => true,
            Err(TrySendError::Full(_)) => {
                log::error!(
                    "IPC event channel is full and cannot receive any more events right now"
                );
                true
            }
            Err(TrySendError::Closed(_)) => false,
        });
    }

    pub async fn request<P, R>(&self, method: &str, params: Option<P>) -> Result<R, Error>
    where
        P: serde::Serialize + Send + Sync + 'static,
        for<'de> R: serde::Deserialize<'de> + Send + Sync + std::fmt::Debug + 'static,
    {
        let (tx, rx) = oneshot::channel();
        {
            let mut queue = self.queue.lock().await;
            if queue.len() >= self.queue_capacity {
                return Err(Error::QueueFull {
                    capacity: self.queue_capacity,
                });
            }
            let queue_depth_at_enqueue = queue.len() + 1;
            queue.push_back(Box::new(PendingRequest {
                id: self.next_request_id.fetch_add(1, Ordering::Relaxed),
                method: method.to_string(),
                params,
                enqueued_at: Instant::now(),
                queue_depth_at_enqueue,
                queue_depth_at_dequeue: 0,
                tx: Some(tx),
            }));
        }

        let res = rx.await.map_err(|_| Error::ResponseChannelClosed)?;
        match res {
            Response::Success(result) => Ok(result),
            Response::Error { kind, message } => Err(Error::Script { kind, message }),
        }
    }

    pub async fn notification<P>(&self, method: &str, params: Option<P>) -> Result<(), Error>
    where
        P: serde::Serialize + Send + Sync + 'static,
    {
        let (tx, rx) = oneshot::channel::<Response<()>>();
        {
            let mut queue = self.queue.lock().await;
            if queue.len() >= self.queue_capacity {
                return Err(Error::QueueFull {
                    capacity: self.queue_capacity,
                });
            }
            let queue_depth_at_enqueue = queue.len() + 1;
            queue.push_back(Box::new(PendingRequest {
                id: self.next_request_id.fetch_add(1, Ordering::Relaxed),
                method: method.to_string(),
                params,
                enqueued_at: Instant::now(),
                queue_depth_at_enqueue,
                queue_depth_at_dequeue: 0,
                tx: Some(tx),
            }));
        }

        match rx.await.map_err(|_| Error::ResponseChannelClosed)? {
            Response::Success(()) => Ok(()),
            Response::Error { kind, message } => Err(Error::Script { kind, message }),
        }
    }

    pub async fn events(&self) -> impl Stream<Item = E> + use<E> {
        let (tx, rx) = mpsc::channel(1024);
        {
            let mut subscriptions = self.subscriptions.lock().await;
            subscriptions.push(tx);
        }
        ReceiverStream::new(rx)
    }
}

impl<E> Default for IPC<E> {
    fn default() -> Self {
        Self::with_queue_capacity(DEFAULT_QUEUE_CAPACITY)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error from mission script: {message}")]
    Script {
        kind: Option<String>,
        message: String,
    },
    #[error("IPC request queue is full (capacity {capacity})")]
    QueueFull { capacity: usize },
    #[error("IPC response channel closed before a result was delivered")]
    ResponseChannelClosed,
    #[error("Failed to deserialize params: {0}")]
    DeserializeParams(#[source] mlua::Error),
    #[error("Failed to deserialize result for method {method}: {err}\n{result}")]
    DeserializeResult {
        #[source]
        err: mlua::Error,
        method: String,
        result: String,
    },
    #[error("Failed to serialize params: {0}")]
    SerializeParams(#[source] mlua::Error),
}

impl<E> Clone for IPC<E> {
    fn clone(&self) -> Self {
        IPC {
            queue: self.queue.clone(),
            queue_capacity: self.queue_capacity,
            next_request_id: self.next_request_id.clone(),
            discarded_cancelled: self.discarded_cancelled.clone(),
            subscriptions: self.subscriptions.clone(),
        }
    }
}

impl<P, R> Request for PendingRequest<P, R>
where
    P: Serialize,
    for<'de> R: Deserialize<'de> + std::fmt::Debug,
{
    fn id(&self) -> u64 {
        self.id
    }

    fn method(&self) -> &str {
        &self.method
    }

    fn queue_wait(&self) -> Duration {
        self.enqueued_at.elapsed()
    }

    fn queue_depth_at_enqueue(&self) -> usize {
        self.queue_depth_at_enqueue
    }

    fn queue_depth_at_dequeue(&self) -> usize {
        self.queue_depth_at_dequeue
    }

    fn set_queue_depth_at_dequeue(&mut self, depth: usize) {
        self.queue_depth_at_dequeue = depth;
    }

    fn is_cancelled(&self) -> bool {
        self.tx.as_ref().is_none_or(oneshot::Sender::is_closed)
    }

    fn params(&self, lua: &Lua) -> Result<Option<Value>, mlua::Error> {
        self.params
            .as_ref()
            .map(|params| {
                lua.to_value_with(
                    params,
                    SerializeOptions::new().serialize_none_to_null(false),
                )
            })
            .transpose()
    }

    fn success(&mut self, lua: &Lua, value: &Value) -> Result<(), mlua::Error> {
        let res = lua.from_value(value.clone())?;
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(Response::Success(res));
        } else {
            log::error!("Failed to send IPC success result: channel gone");
        }

        Ok(())
    }

    fn error(&mut self, message: String, kind: Option<String>) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(Response::Error { kind, message });
        } else {
            log::error!("Failed to send IPC error result: channel gone");
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response<R> {
    Success(R),
    Error {
        kind: Option<String>,
        message: String,
    },
}

#[allow(unused)]
fn pretty_print_value(val: Value, indent: usize) -> LuaResult<String> {
    Ok(match val {
        Value::Nil => "nil".to_string(),
        Value::Boolean(v) => v.to_string(),
        Value::LightUserData(_) => String::new(),
        Value::Integer(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::String(v) => format!("\"{}\"", v.to_str()?),
        Value::Table(t) => {
            let mut s = "{\n".to_string();
            for pair in t.pairs::<Value, Value>() {
                let (key, value) = pair?;
                s += &format!(
                    "{}{} = {},\n",
                    "  ".repeat(indent + 1),
                    pretty_print_value(key, indent + 1)?,
                    pretty_print_value(value, indent + 1)?
                );
            }
            s += &format!("{}}}", "  ".repeat(indent));
            s
        }
        Value::Function(_) => "[function]".to_string(),
        Value::Thread(_) => String::new(),
        Value::UserData(_) => String::new(),
        Value::Error(err) => err.to_string(),
        Value::Other(_) => "(unknown type)".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn request_rejects_enqueue_when_queue_is_full() {
        let ipc = IPC::<()>::with_queue_capacity(1);
        let mut queued = Box::pin(ipc.request::<_, ()>("first", Some(1_u8)));
        tokio::select! {
            result = &mut queued => panic!("request completed before dequeue: {result:?}"),
            _ = tokio::task::yield_now() => {}
        }

        let error = ipc
            .request::<_, ()>("second", Some(2_u8))
            .await
            .unwrap_err();
        assert!(matches!(error, Error::QueueFull { capacity: 1 }));
        drop(queued);
    }

    #[tokio::test]
    async fn try_next_discards_cancelled_requests() {
        let ipc = IPC::<()>::with_queue_capacity(2);
        let mut queued = Box::pin(ipc.request::<_, ()>("cancelled", Some(1_u8)));
        tokio::select! {
            result = &mut queued => panic!("request completed before dequeue: {result:?}"),
            _ = tokio::task::yield_now() => {}
        }
        drop(queued);

        assert!(ipc.try_next().is_none());
        assert_eq!(ipc.take_discarded_cancelled(), 1);
    }

    #[tokio::test]
    async fn dequeued_request_exposes_correlation_and_queue_metadata() {
        let ipc = IPC::<()>::with_queue_capacity(2);
        let mut queued = Box::pin(ipc.request::<_, ()>("observed", Some(1_u8)));
        tokio::select! {
            result = &mut queued => panic!("request completed before dequeue: {result:?}"),
            _ = tokio::task::yield_now() => {}
        }

        let mut request = ipc.try_next().expect("request must be queued");
        assert_eq!(request.id(), 1);
        assert_eq!(request.method(), "observed");
        assert_eq!(request.queue_depth_at_enqueue(), 1);
        assert_eq!(request.queue_depth_at_dequeue(), 0);
        assert!(!request.is_cancelled());
        request.error("expected test response".to_string(), None);

        let error = queued.await.unwrap_err();
        assert!(matches!(error, Error::Script { .. }));
    }

    #[tokio::test]
    async fn live_requests_remain_fifo() {
        let ipc = IPC::<()>::with_queue_capacity(3);
        let mut first = Box::pin(ipc.request::<_, ()>("first", Some(1_u8)));
        let mut second = Box::pin(ipc.request::<_, ()>("second", Some(2_u8)));
        tokio::select! {
            result = &mut first => panic!("request completed before dequeue: {result:?}"),
            _ = tokio::task::yield_now() => {}
        }
        tokio::select! {
            result = &mut second => panic!("request completed before dequeue: {result:?}"),
            _ = tokio::task::yield_now() => {}
        }

        let mut first_request = ipc.try_next().expect("first request must be queued");
        let mut second_request = ipc.try_next().expect("second request must be queued");
        assert_eq!(first_request.method(), "first");
        assert_eq!(second_request.method(), "second");
        assert!(second_request.queue_depth_at_enqueue() >= first_request.queue_depth_at_enqueue());
        first_request.error("expected test response".to_string(), None);
        second_request.error("expected test response".to_string(), None);
        assert!(first.await.is_err());
        assert!(second.await.is_err());
    }
}
