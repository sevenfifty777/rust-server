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
        let res = self.request("getRecoverySnapshot", request).await?;
        Ok(Response::new(res))
    }
}
