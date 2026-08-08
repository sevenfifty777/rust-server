use stubs::world::v0::world_service_server::WorldService;
use stubs::*;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl WorldService for MissionRpc {
    async fn get_airbases(
        &self,
        request: Request<world::v0::GetAirbasesRequest>,
    ) -> Result<Response<world::v0::GetAirbasesResponse>, Status> {
        let res = self.request("getAirbases", request).await?;
        Ok(Response::new(res))
    }

    async fn get_mark_panels(
        &self,
        request: Request<world::v0::GetMarkPanelsRequest>,
    ) -> Result<Response<world::v0::GetMarkPanelsResponse>, Status> {
        let res = self.request("getMarkPanels", request).await?;
        Ok(Response::new(res))
    }

    async fn get_theatre(
        &self,
        request: Request<world::v0::GetTheatreRequest>,
    ) -> Result<Response<world::v0::GetTheatreResponse>, Status> {
        let res = self.request("getTheatre", request).await?;
        Ok(Response::new(res))
    }

    async fn search_objects(
        &self,
        request: Request<world::v0::SearchObjectsRequest>,
    ) -> Result<Response<world::v0::SearchObjectsResponse>, Status> {
        let res = self.request("searchObjects", request).await?;
        Ok(Response::new(res))
    }

    async fn get_airbase_parking(
        &self,
        request: Request<world::v0::GetAirbaseParkingRequest>,
    ) -> Result<Response<world::v0::GetAirbaseParkingResponse>, Status> {
        let res = self.request("getAirbaseParking", request).await?;
        Ok(Response::new(res))
    }

    async fn get_airbase_runways(
        &self,
        request: Request<world::v0::GetAirbaseRunwaysRequest>,
    ) -> Result<Response<world::v0::GetAirbaseRunwaysResponse>, Status> {
        let res = self.request("getAirbaseRunways", request).await?;
        Ok(Response::new(res))
    }

    async fn get_airbase_id(
        &self,
        request: Request<world::v0::GetAirbaseIdRequest>,
    ) -> Result<Response<world::v0::GetAirbaseIdResponse>, Status> {
        let res = self.request("getAirbaseID", request).await?;
        Ok(Response::new(res))
    }

    async fn get_airbase_radio_silent_mode(
        &self,
        request: Request<world::v0::GetAirbaseRadioSilentModeRequest>,
    ) -> Result<Response<world::v0::GetAirbaseRadioSilentModeResponse>, Status> {
        let res = self.request("getAirbaseRadioSilentMode", request).await?;
        Ok(Response::new(res))
    }

    async fn set_airbase_radio_silent_mode(
        &self,
        request: Request<world::v0::SetAirbaseRadioSilentModeRequest>,
    ) -> Result<Response<world::v0::SetAirbaseRadioSilentModeResponse>, Status> {
        let res = self.request("setAirbaseRadioSilentMode", request).await?;
        Ok(Response::new(res))
    }

    async fn set_airbase_coalition(
        &self,
        request: Request<world::v0::SetAirbaseCoalitionRequest>,
    ) -> Result<Response<world::v0::SetAirbaseCoalitionResponse>, Status> {
        let res = self.request("setAirbaseCoalition", request).await?;
        Ok(Response::new(res))
    }
}
