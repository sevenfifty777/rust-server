use tonic::{Request, Response, Status};

use crate::rpc::MissionRpc;
use stubs::spot;
use stubs::spot::v0::spot_service_server::SpotService;

#[tonic::async_trait]
impl SpotService for MissionRpc {
    async fn create_laser(
        &self,
        request: Request<spot::v0::CreateLaserRequest>,
    ) -> Result<Response<spot::v0::CreateLaserResponse>, Status> {
        let res = self.request("createLaser", request).await?;
        Ok(Response::new(res))
    }

    async fn create_infra_red(
        &self,
        request: Request<spot::v0::CreateInfraRedRequest>,
    ) -> Result<Response<spot::v0::CreateInfraRedResponse>, Status> {
        let res = self.request("createInfraRed", request).await?;
        Ok(Response::new(res))
    }

    async fn destroy(
        &self,
        request: Request<spot::v0::DestroyRequest>,
    ) -> Result<Response<spot::v0::DestroyResponse>, Status> {
        let res = self.request("destroySpot", request).await?;
        Ok(Response::new(res))
    }

    async fn get_point(
        &self,
        request: Request<spot::v0::GetPointRequest>,
    ) -> Result<Response<spot::v0::GetPointResponse>, Status> {
        let res = self.request("getSpotPoint", request).await?;
        Ok(Response::new(res))
    }

    async fn set_code(
        &self,
        request: Request<spot::v0::SetCodeRequest>,
    ) -> Result<Response<spot::v0::SetCodeResponse>, Status> {
        let res = self.request("setSpotCode", request).await?;
        Ok(Response::new(res))
    }

    async fn set_point(
        &self,
        request: Request<spot::v0::SetPointRequest>,
    ) -> Result<Response<spot::v0::SetPointResponse>, Status> {
        let res = self.request("setSpotPoint", request).await?;
        Ok(Response::new(res))
    }

    async fn get_code(
        &self,
        request: Request<spot::v0::GetCodeRequest>,
    ) -> Result<Response<spot::v0::GetCodeResponse>, Status> {
        let res = self.request("getSpotCode", request).await?;
        Ok(Response::new(res))
    }

    async fn get_category(
        &self,
        request: Request<spot::v0::GetCategoryRequest>,
    ) -> Result<Response<spot::v0::GetCategoryResponse>, Status> {
        let res = self.request("getSpotCategory", request).await?;
        Ok(Response::new(res))
    }
}
