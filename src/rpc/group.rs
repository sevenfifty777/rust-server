use stubs::group::v0::group_service_server::GroupService;
use stubs::*;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl GroupService for MissionRpc {
    async fn get_units(
        &self,
        request: Request<group::v0::GetUnitsRequest>,
    ) -> Result<Response<group::v0::GetUnitsResponse>, Status> {
        let res = self.request("getUnits", request).await?;
        Ok(Response::new(res))
    }

    async fn activate(
        &self,
        request: Request<group::v0::ActivateRequest>,
    ) -> Result<Response<group::v0::ActivateResponse>, Status> {
        let res = self.request("groupActivate", request).await?;
        Ok(Response::new(res))
    }

    async fn destroy(
        &self,
        request: Request<group::v0::DestroyRequest>,
    ) -> Result<Response<group::v0::DestroyResponse>, Status> {
        let res = self.request("groupDestroy", request).await?;
        Ok(Response::new(res))
    }

    async fn get_size(
        &self,
        request: Request<group::v0::GetSizeRequest>,
    ) -> Result<Response<group::v0::GetSizeResponse>, Status> {
        let res = self.request("getGroupSize", request).await?;
        Ok(Response::new(res))
    }

    async fn exists(
        &self,
        request: Request<group::v0::ExistsRequest>,
    ) -> Result<Response<group::v0::ExistsResponse>, Status> {
        let res = self.request("groupExists", request).await?;
        Ok(Response::new(res))
    }

    async fn enable_emission(
        &self,
        request: Request<group::v0::EnableEmissionRequest>,
    ) -> Result<Response<group::v0::EnableEmissionResponse>, Status> {
        let res = self.request("enableEmission", request).await?;
        Ok(Response::new(res))
    }

    async fn get_group(
        &self,
        request: Request<group::v0::GetGroupRequest>,
    ) -> Result<Response<group::v0::GetGroupResponse>, Status> {
        let res = self.request("getGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn get_unit(
        &self,
        request: Request<group::v0::GetUnitRequest>,
    ) -> Result<Response<group::v0::GetUnitResponse>, Status> {
        let res = self.request("getGroupUnit", request).await?;
        Ok(Response::new(res))
    }
}
