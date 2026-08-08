use stubs::land;
use stubs::land::v0::land_service_server::LandService;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl LandService for MissionRpc {
    async fn get_terrain_height(
        &self,
        request: Request<land::v0::GetTerrainHeightRequest>,
    ) -> Result<Response<land::v0::GetTerrainHeightResponse>, Status> {
        let res = self.request("getTerrainHeight", request).await?;
        Ok(Response::new(res))
    }

    async fn get_surface_type(
        &self,
        request: Request<land::v0::GetSurfaceTypeRequest>,
    ) -> Result<Response<land::v0::GetSurfaceTypeResponse>, Status> {
        let res = self.request("getSurfaceType", request).await?;
        Ok(Response::new(res))
    }

    async fn is_visible(
        &self,
        request: Request<land::v0::IsVisibleRequest>,
    ) -> Result<Response<land::v0::IsVisibleResponse>, Status> {
        let res = self.request("isTerrainVisible", request).await?;
        Ok(Response::new(res))
    }

    async fn get_closest_point_on_roads(
        &self,
        request: Request<land::v0::GetClosestPointOnRoadsRequest>,
    ) -> Result<Response<land::v0::GetClosestPointOnRoadsResponse>, Status> {
        let res = self.request("getClosestPointOnRoads", request).await?;
        Ok(Response::new(res))
    }

    async fn get_surface_height_with_seabed(
        &self,
        request: Request<land::v0::GetSurfaceHeightWithSeabedRequest>,
    ) -> Result<Response<land::v0::GetSurfaceHeightWithSeabedResponse>, Status> {
        let res = self.request("getSurfaceHeightWithSeabed", request).await?;
        Ok(Response::new(res))
    }

    async fn find_path_on_roads(
        &self,
        request: Request<land::v0::FindPathOnRoadsRequest>,
    ) -> Result<Response<land::v0::FindPathOnRoadsResponse>, Status> {
        let res = self.request("findPathOnRoads", request).await?;
        Ok(Response::new(res))
    }

    async fn get_ip(
        &self,
        request: Request<land::v0::GetIpRequest>,
    ) -> Result<Response<land::v0::GetIpResponse>, Status> {
        let res = self.request("getIP", request).await?;
        Ok(Response::new(res))
    }

    async fn profile(
        &self,
        request: Request<land::v0::ProfileRequest>,
    ) -> Result<Response<land::v0::ProfileResponse>, Status> {
        let res = self.request("profile", request).await?;
        Ok(Response::new(res))
    }
}
