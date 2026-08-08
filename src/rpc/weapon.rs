use stubs::weapon::v0::weapon_service_server::WeaponService;
use stubs::*;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl WeaponService for MissionRpc {
    async fn get_launcher(
        &self,
        request: Request<weapon::v0::GetLauncherRequest>,
    ) -> Result<Response<weapon::v0::GetLauncherResponse>, Status> {
        let res = self.request("weaponGetLauncher", request).await?;
        Ok(Response::new(res))
    }

    async fn get_target(
        &self,
        request: Request<weapon::v0::GetTargetRequest>,
    ) -> Result<Response<weapon::v0::GetTargetResponse>, Status> {
        let res = self.request("weaponGetTarget", request).await?;
        Ok(Response::new(res))
    }

    async fn get_category(
        &self,
        request: Request<weapon::v0::GetCategoryRequest>,
    ) -> Result<Response<weapon::v0::GetCategoryResponse>, Status> {
        let res = self.request("weaponGetCategory", request).await?;
        Ok(Response::new(res))
    }

    async fn get_desc(
        &self,
        request: Request<weapon::v0::GetDescRequest>,
    ) -> Result<Response<weapon::v0::GetDescResponse>, Status> {
        let res = self.request("weaponGetDesc", request).await?;
        Ok(Response::new(res))
    }

    async fn get_position(
        &self,
        request: Request<weapon::v0::GetPositionRequest>,
    ) -> Result<Response<weapon::v0::GetPositionResponse>, Status> {
        let res = self.request("weaponGetPosition", request).await?;
        Ok(Response::new(res))
    }

    async fn get_velocity(
        &self,
        request: Request<weapon::v0::GetVelocityRequest>,
    ) -> Result<Response<weapon::v0::GetVelocityResponse>, Status> {
        let res = self.request("weaponGetVelocity", request).await?;
        Ok(Response::new(res))
    }

    async fn in_air(
        &self,
        request: Request<weapon::v0::InAirRequest>,
    ) -> Result<Response<weapon::v0::InAirResponse>, Status> {
        let res = self.request("weaponInAir", request).await?;
        Ok(Response::new(res))
    }

    async fn is_exist(
        &self,
        request: Request<weapon::v0::IsExistRequest>,
    ) -> Result<Response<weapon::v0::IsExistResponse>, Status> {
        let res = self.request("weaponIsExist", request).await?;
        Ok(Response::new(res))
    }

    async fn destroy(
        &self,
        request: Request<weapon::v0::DestroyRequest>,
    ) -> Result<Response<weapon::v0::DestroyResponse>, Status> {
        let res = self.request("weaponDestroy", request).await?;
        Ok(Response::new(res))
    }

    async fn get_coalition(
        &self,
        request: Request<weapon::v0::GetCoalitionRequest>,
    ) -> Result<Response<weapon::v0::GetCoalitionResponse>, Status> {
        let res = self.request("weaponGetCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn get_country(
        &self,
        request: Request<weapon::v0::GetCountryRequest>,
    ) -> Result<Response<weapon::v0::GetCountryResponse>, Status> {
        let res = self.request("weaponGetCountry", request).await?;
        Ok(Response::new(res))
    }

    async fn get_name(
        &self,
        request: Request<weapon::v0::GetNameRequest>,
    ) -> Result<Response<weapon::v0::GetNameResponse>, Status> {
        let res = self.request("weaponGetName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_type_name(
        &self,
        request: Request<weapon::v0::GetTypeNameRequest>,
    ) -> Result<Response<weapon::v0::GetTypeNameResponse>, Status> {
        let res = self.request("weaponGetTypeName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_point(
        &self,
        request: Request<weapon::v0::GetPointRequest>,
    ) -> Result<Response<weapon::v0::GetPointResponse>, Status> {
        let res = self.request("weaponGetPoint", request).await?;
        Ok(Response::new(res))
    }
}
