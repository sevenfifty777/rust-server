use stubs::recovery;
use stubs::recovery::v0::recovery_service_server::RecoveryService;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl RecoveryService for MissionRpc {
    async fn get_recovery_snapshot(
        &self,
        request: Request<recovery::v0::GetRecoverySnapshotRequest>,
    ) -> Result<Response<recovery::v0::GetRecoverySnapshotResponse>, Status> {
        // The queue-wait / queue-depth diagnostics are measured by the bridge at dequeue time and
        // handed to the Lua callback as request metadata (see `next` in `lib.rs`), which puts them
        // into the response; `lua_exec_ms` is measured inside the Lua callback itself.
        let res: recovery::v0::GetRecoverySnapshotResponse =
            self.request("getRecoverySnapshot", request).await?;
        log::debug!(
            "recovery snapshot diagnostics: sequence={} queue_wait_ms={:?} lua_exec_ms={:?} queue_depth={:?}",
            res.sequence,
            res.queue_wait_ms,
            res.lua_exec_ms,
            res.queue_depth
        );
        Ok(Response::new(res))
    }
}
