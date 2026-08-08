use stubs::warehouse;
use stubs::warehouse::v0::warehouse_service_server::WarehouseService;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl WarehouseService for MissionRpc {
    async fn get_inventory(
        &self,
        request: Request<warehouse::v0::GetInventoryRequest>,
    ) -> Result<Response<warehouse::v0::GetInventoryResponse>, Status> {
        let res = self.request("getInventory", request).await?;
        Ok(Response::new(res))
    }

    async fn get_item_count(
        &self,
        request: Request<warehouse::v0::GetItemCountRequest>,
    ) -> Result<Response<warehouse::v0::GetItemCountResponse>, Status> {
        let res = self.request("getItemCount", request).await?;
        Ok(Response::new(res))
    }

    async fn add_item(
        &self,
        request: Request<warehouse::v0::AddItemRequest>,
    ) -> Result<Response<warehouse::v0::AddItemResponse>, Status> {
        let res = self.request("addItem", request).await?;
        Ok(Response::new(res))
    }

    async fn remove_item(
        &self,
        request: Request<warehouse::v0::RemoveItemRequest>,
    ) -> Result<Response<warehouse::v0::RemoveItemResponse>, Status> {
        let res = self.request("removeItem", request).await?;
        Ok(Response::new(res))
    }

    async fn set_item(
        &self,
        request: Request<warehouse::v0::SetItemRequest>,
    ) -> Result<Response<warehouse::v0::SetItemResponse>, Status> {
        let res = self.request("setItem", request).await?;
        Ok(Response::new(res))
    }

    async fn get_liquid_amount(
        &self,
        request: Request<warehouse::v0::GetLiquidAmountRequest>,
    ) -> Result<Response<warehouse::v0::GetLiquidAmountResponse>, Status> {
        let res = self.request("getLiquidAmount", request).await?;
        Ok(Response::new(res))
    }

    async fn add_liquid(
        &self,
        request: Request<warehouse::v0::AddLiquidRequest>,
    ) -> Result<Response<warehouse::v0::AddLiquidResponse>, Status> {
        let res = self.request("addLiquid", request).await?;
        Ok(Response::new(res))
    }

    async fn set_liquid_amount(
        &self,
        request: Request<warehouse::v0::SetLiquidAmountRequest>,
    ) -> Result<Response<warehouse::v0::SetLiquidAmountResponse>, Status> {
        let res = self.request("setLiquidAmount", request).await?;
        Ok(Response::new(res))
    }

    async fn get_owner(
        &self,
        request: Request<warehouse::v0::GetOwnerRequest>,
    ) -> Result<Response<warehouse::v0::GetOwnerResponse>, Status> {
        let res = self.request("getOwner", request).await?;
        Ok(Response::new(res))
    }
}
