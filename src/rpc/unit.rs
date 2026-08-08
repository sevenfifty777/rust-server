use stubs::unit;
use stubs::unit::v0::unit_service_server::UnitService;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl UnitService for MissionRpc {
    async fn get_radar(
        &self,
        request: Request<unit::v0::GetRadarRequest>,
    ) -> Result<Response<unit::v0::GetRadarResponse>, Status> {
        let res = self.request("getRadar", request).await?;
        Ok(Response::new(res))
    }

    async fn get_position(
        &self,
        request: Request<unit::v0::GetPositionRequest>,
    ) -> Result<Response<unit::v0::GetPositionResponse>, Status> {
        let res = self.request("getUnitPosition", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_name(
        &self,
        request: Request<unit::v0::GetPlayerNameRequest>,
    ) -> Result<Response<unit::v0::GetPlayerNameResponse>, Status> {
        let res = self.request("getUnitPlayerName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_descriptor(
        &self,
        request: Request<unit::v0::GetDescriptorRequest>,
    ) -> Result<Response<unit::v0::GetDescriptorResponse>, Status> {
        let res = self.request("getUnitDescriptor", request).await?;
        Ok(Response::new(res))
    }

    async fn set_emission(
        &self,
        request: Request<unit::v0::SetEmissionRequest>,
    ) -> Result<Response<unit::v0::SetEmissionResponse>, Status> {
        let res = self.request("setEmission", request).await?;
        Ok(Response::new(res))
    }

    async fn get_draw_argument_value(
        &self,
        request: Request<unit::v0::GetDrawArgumentValueRequest>,
    ) -> Result<Response<unit::v0::GetDrawArgumentValueResponse>, Status> {
        let res = self.request("getDrawArgumentValue", request).await?;
        Ok(Response::new(res))
    }

    async fn get(
        &self,
        request: Request<unit::v0::GetRequest>,
    ) -> Result<Response<unit::v0::GetResponse>, Status> {
        let res = self.request("getUnit", request).await?;
        Ok(Response::new(res))
    }

    async fn get_transform(
        &self,
        request: Request<unit::v0::GetTransformRequest>,
    ) -> Result<Response<unit::v0::GetTransformResponse>, Status> {
        let res = self.request("getUnitTransform", request).await?;
        Ok(Response::new(res))
    }

    async fn destroy(
        &self,
        request: Request<unit::v0::DestroyRequest>,
    ) -> Result<Response<unit::v0::DestroyResponse>, Status> {
        let res = self.request("unitDestroy", request).await?;
        Ok(Response::new(res))
    }

    async fn get_sensors(
        &self,
        request: Request<unit::v0::GetSensorsRequest>,
    ) -> Result<Response<unit::v0::GetSensorsResponse>, Status> {
        let res = self.request("getSensors", request).await?;
        Ok(Response::new(res))
    }

    async fn get_life(
        &self,
        request: Request<unit::v0::GetLifeRequest>,
    ) -> Result<Response<unit::v0::GetLifeResponse>, Status> {
        let res = self.request("getUnitLife", request).await?;
        Ok(Response::new(res))
    }

    async fn get_fuel(
        &self,
        request: Request<unit::v0::GetFuelRequest>,
    ) -> Result<Response<unit::v0::GetFuelResponse>, Status> {
        let res = self.request("getUnitFuel", request).await?;
        Ok(Response::new(res))
    }

    async fn get_ammo(
        &self,
        request: Request<unit::v0::GetAmmoRequest>,
    ) -> Result<Response<unit::v0::GetAmmoResponse>, Status> {
        let res = self.request("getUnitAmmo", request).await?;
        Ok(Response::new(res))
    }

    async fn in_air(
        &self,
        request: Request<unit::v0::InAirRequest>,
    ) -> Result<Response<unit::v0::InAirResponse>, Status> {
        let res = self.request("getUnitInAir", request).await?;
        Ok(Response::new(res))
    }

    async fn is_active(
        &self,
        request: Request<unit::v0::IsActiveRequest>,
    ) -> Result<Response<unit::v0::IsActiveResponse>, Status> {
        let res = self.request("getUnitIsActive", request).await?;
        Ok(Response::new(res))
    }

    async fn get_country(
        &self,
        request: Request<unit::v0::GetCountryRequest>,
    ) -> Result<Response<unit::v0::GetCountryResponse>, Status> {
        let res = self.request("getUnitCountry", request).await?;
        Ok(Response::new(res))
    }

    async fn get_number(
        &self,
        request: Request<unit::v0::GetNumberRequest>,
    ) -> Result<Response<unit::v0::GetNumberResponse>, Status> {
        let res = self.request("getUnitNumber", request).await?;
        Ok(Response::new(res))
    }

    async fn get_group(
        &self,
        request: Request<unit::v0::GetGroupRequest>,
    ) -> Result<Response<unit::v0::GetGroupResponse>, Status> {
        let res = self.request("getUnitGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn get_life0(
        &self,
        request: Request<unit::v0::GetLife0Request>,
    ) -> Result<Response<unit::v0::GetLife0Response>, Status> {
        let res = self.request("getUnitLife0", request).await?;
        Ok(Response::new(res))
    }

    async fn has_sensors(
        &self,
        request: Request<unit::v0::HasSensorsRequest>,
    ) -> Result<Response<unit::v0::HasSensorsResponse>, Status> {
        let res = self.request("unitHasSensors", request).await?;
        Ok(Response::new(res))
    }

    async fn get_nearest_cargos(
        &self,
        request: Request<unit::v0::GetNearestCargosRequest>,
    ) -> Result<Response<unit::v0::GetNearestCargosResponse>, Status> {
        let res = self.request("getUnitNearestCargos", request).await?;
        Ok(Response::new(res))
    }

    async fn get_descent_capacity(
        &self,
        request: Request<unit::v0::GetDescentCapacityRequest>,
    ) -> Result<Response<unit::v0::GetDescentCapacityResponse>, Status> {
        let res = self.request("getUnitDescentCapacity", request).await?;
        Ok(Response::new(res))
    }

    async fn get_desc_by_name(
        &self,
        request: Request<unit::v0::GetDescByNameRequest>,
    ) -> Result<Response<unit::v0::GetDescByNameResponse>, Status> {
        let res = self.request("getUnitDescByName", request).await?;
        Ok(Response::new(res))
    }
}
