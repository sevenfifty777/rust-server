use stubs::trigger;
use stubs::trigger::v0::trigger_service_server::TriggerService;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl TriggerService for MissionRpc {
    async fn out_text(
        &self,
        request: Request<trigger::v0::OutTextRequest>,
    ) -> Result<Response<trigger::v0::OutTextResponse>, Status> {
        let res = self.request("outText", request).await?;
        Ok(Response::new(res))
    }

    async fn out_text_for_coalition(
        &self,
        request: Request<trigger::v0::OutTextForCoalitionRequest>,
    ) -> Result<Response<trigger::v0::OutTextForCoalitionResponse>, Status> {
        let res = self.request("outTextForCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn out_text_for_group(
        &self,
        request: Request<trigger::v0::OutTextForGroupRequest>,
    ) -> Result<Response<trigger::v0::OutTextForGroupResponse>, Status> {
        let res = self.request("outTextForGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn out_text_for_unit(
        &self,
        request: Request<trigger::v0::OutTextForUnitRequest>,
    ) -> Result<Response<trigger::v0::OutTextForUnitResponse>, Status> {
        let res = self.request("outTextForUnit", request).await?;
        Ok(Response::new(res))
    }

    async fn get_user_flag(
        &self,
        request: Request<trigger::v0::GetUserFlagRequest>,
    ) -> Result<Response<trigger::v0::GetUserFlagResponse>, Status> {
        let res = self.request("getUserFlag", request).await?;
        Ok(Response::new(res))
    }

    async fn set_user_flag(
        &self,
        request: Request<trigger::v0::SetUserFlagRequest>,
    ) -> Result<Response<trigger::v0::SetUserFlagResponse>, Status> {
        let res = self.request("setUserFlag", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_all(
        &self,
        request: Request<trigger::v0::MarkToAllRequest>,
    ) -> Result<Response<trigger::v0::MarkToAllResponse>, Status> {
        let res = self.request("markToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_coalition(
        &self,
        request: Request<trigger::v0::MarkToCoalitionRequest>,
    ) -> Result<Response<trigger::v0::MarkToCoalitionResponse>, Status> {
        let res = self.request("markToCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_group(
        &self,
        request: Request<trigger::v0::MarkToGroupRequest>,
    ) -> Result<Response<trigger::v0::MarkToGroupResponse>, Status> {
        let res = self.request("markToGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn remove_mark(
        &self,
        request: Request<trigger::v0::RemoveMarkRequest>,
    ) -> Result<Response<trigger::v0::RemoveMarkResponse>, Status> {
        let res = self.request("removeMark", request).await?;
        Ok(Response::new(res))
    }

    async fn markup_to_all(
        &self,
        request: Request<trigger::v0::MarkupToAllRequest>,
    ) -> Result<Response<trigger::v0::MarkupToAllResponse>, Status> {
        let res = self.request("markupToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn markup_to_coalition(
        &self,
        request: Request<trigger::v0::MarkupToCoalitionRequest>,
    ) -> Result<Response<trigger::v0::MarkupToCoalitionResponse>, Status> {
        let res = self.request("markupToCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn explosion(
        &self,
        request: Request<trigger::v0::ExplosionRequest>,
    ) -> Result<Response<trigger::v0::ExplosionResponse>, Status> {
        let res = self.request("explosion", request).await?;
        Ok(Response::new(res))
    }

    async fn smoke(
        &self,
        request: Request<trigger::v0::SmokeRequest>,
    ) -> Result<Response<trigger::v0::SmokeResponse>, Status> {
        let res = self.request("smoke", request).await?;
        Ok(Response::new(res))
    }

    async fn illumination_bomb(
        &self,
        request: Request<trigger::v0::IlluminationBombRequest>,
    ) -> Result<Response<trigger::v0::IlluminationBombResponse>, Status> {
        let res = self.request("illuminationBomb", request).await?;
        Ok(Response::new(res))
    }

    async fn signal_flare(
        &self,
        request: Request<trigger::v0::SignalFlareRequest>,
    ) -> Result<Response<trigger::v0::SignalFlareResponse>, Status> {
        let res = self.request("signalFlare", request).await?;
        Ok(Response::new(res))
    }

    async fn get_zone(
        &self,
        request: Request<trigger::v0::GetZoneRequest>,
    ) -> Result<Response<trigger::v0::GetZoneResponse>, Status> {
        let res = self.request("getZone", request).await?;
        Ok(Response::new(res))
    }

    async fn effect_smoke_big(
        &self,
        request: Request<trigger::v0::EffectSmokeBigRequest>,
    ) -> Result<Response<trigger::v0::EffectSmokeBigResponse>, Status> {
        let res = self.request("effectSmokeBig", request).await?;
        Ok(Response::new(res))
    }

    async fn effect_smoke_stop(
        &self,
        request: Request<trigger::v0::EffectSmokeStopRequest>,
    ) -> Result<Response<trigger::v0::EffectSmokeStopResponse>, Status> {
        let res = self.request("effectSmokeStop", request).await?;
        Ok(Response::new(res))
    }

    async fn set_unit_internal_cargo(
        &self,
        request: Request<trigger::v0::SetUnitInternalCargoRequest>,
    ) -> Result<Response<trigger::v0::SetUnitInternalCargoResponse>, Status> {
        let res = self.request("setUnitInternalCargo", request).await?;
        Ok(Response::new(res))
    }

    async fn activate_group(
        &self,
        request: Request<trigger::v0::ActivateGroupRequest>,
    ) -> Result<Response<trigger::v0::ActivateGroupResponse>, Status> {
        let res = self.request("activateGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn deactivate_group(
        &self,
        request: Request<trigger::v0::DeactivateGroupRequest>,
    ) -> Result<Response<trigger::v0::DeactivateGroupResponse>, Status> {
        let res = self.request("deactivateGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn set_group_ai_on(
        &self,
        request: Request<trigger::v0::SetGroupAiOnRequest>,
    ) -> Result<Response<trigger::v0::SetGroupAiOnResponse>, Status> {
        let res = self.request("setGroupAIOn", request).await?;
        Ok(Response::new(res))
    }

    async fn set_group_ai_off(
        &self,
        request: Request<trigger::v0::SetGroupAiOffRequest>,
    ) -> Result<Response<trigger::v0::SetGroupAiOffResponse>, Status> {
        let res = self.request("setGroupAIOff", request).await?;
        Ok(Response::new(res))
    }

    async fn group_stop_moving(
        &self,
        request: Request<trigger::v0::GroupStopMovingRequest>,
    ) -> Result<Response<trigger::v0::GroupStopMovingResponse>, Status> {
        let res = self.request("groupStopMoving", request).await?;
        Ok(Response::new(res))
    }

    async fn group_continue_moving(
        &self,
        request: Request<trigger::v0::GroupContinueMovingRequest>,
    ) -> Result<Response<trigger::v0::GroupContinueMovingResponse>, Status> {
        let res = self.request("groupContinueMoving", request).await?;
        Ok(Response::new(res))
    }

    async fn set_ai_task(
        &self,
        request: Request<trigger::v0::SetAiTaskRequest>,
    ) -> Result<Response<trigger::v0::SetAiTaskResponse>, Status> {
        let res = self.request("setAITask", request).await?;
        Ok(Response::new(res))
    }

    async fn push_ai_task(
        &self,
        request: Request<trigger::v0::PushAiTaskRequest>,
    ) -> Result<Response<trigger::v0::PushAiTaskResponse>, Status> {
        let res = self.request("pushAITask", request).await?;
        Ok(Response::new(res))
    }
    async fn set_markup_radius(
        &self,
        request: Request<trigger::v0::SetMarkupRadiusRequest>,
    ) -> Result<Response<trigger::v0::SetMarkupRadiusResponse>, Status> {
        let res = self.request("setMarkupRadius", request).await?;
        Ok(Response::new(res))
    }

    async fn set_markup_text(
        &self,
        request: Request<trigger::v0::SetMarkupTextRequest>,
    ) -> Result<Response<trigger::v0::SetMarkupTextResponse>, Status> {
        let res = self.request("setMarkupText", request).await?;
        Ok(Response::new(res))
    }

    async fn set_markup_font_size(
        &self,
        request: Request<trigger::v0::SetMarkupFontSizeRequest>,
    ) -> Result<Response<trigger::v0::SetMarkupFontSizeResponse>, Status> {
        let res = self.request("setMarkupFontSize", request).await?;
        Ok(Response::new(res))
    }

    async fn set_markup_color(
        &self,
        request: Request<trigger::v0::SetMarkupColorRequest>,
    ) -> Result<Response<trigger::v0::SetMarkupColorResponse>, Status> {
        let res = self.request("setMarkupColor", request).await?;
        Ok(Response::new(res))
    }

    async fn set_markup_color_fill(
        &self,
        request: Request<trigger::v0::SetMarkupColorFillRequest>,
    ) -> Result<Response<trigger::v0::SetMarkupColorFillResponse>, Status> {
        let res = self.request("setMarkupColorFill", request).await?;
        Ok(Response::new(res))
    }

    async fn set_markup_type_line(
        &self,
        request: Request<trigger::v0::SetMarkupTypeLineRequest>,
    ) -> Result<Response<trigger::v0::SetMarkupTypeLineResponse>, Status> {
        let res = self.request("setMarkupTypeLine", request).await?;
        Ok(Response::new(res))
    }

    async fn set_markup_position_end(
        &self,
        request: Request<trigger::v0::SetMarkupPositionEndRequest>,
    ) -> Result<Response<trigger::v0::SetMarkupPositionEndResponse>, Status> {
        let res = self.request("setMarkupPositionEnd", request).await?;
        Ok(Response::new(res))
    }

    async fn set_markup_position_start(
        &self,
        request: Request<trigger::v0::SetMarkupPositionStartRequest>,
    ) -> Result<Response<trigger::v0::SetMarkupPositionStartResponse>, Status> {
        let res = self.request("setMarkupPositionStart", request).await?;
        Ok(Response::new(res))
    }

    async fn line_to_all(
        &self,
        request: Request<trigger::v0::LineToAllRequest>,
    ) -> Result<Response<trigger::v0::LineToAllResponse>, Status> {
        let res = self.request("lineToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn circle_to_all(
        &self,
        request: Request<trigger::v0::CircleToAllRequest>,
    ) -> Result<Response<trigger::v0::CircleToAllResponse>, Status> {
        let res = self.request("circleToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn rect_to_all(
        &self,
        request: Request<trigger::v0::RectToAllRequest>,
    ) -> Result<Response<trigger::v0::RectToAllResponse>, Status> {
        let res = self.request("rectToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn quad_to_all(
        &self,
        request: Request<trigger::v0::QuadToAllRequest>,
    ) -> Result<Response<trigger::v0::QuadToAllResponse>, Status> {
        let res = self.request("quadToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn text_to_all(
        &self,
        request: Request<trigger::v0::TextToAllRequest>,
    ) -> Result<Response<trigger::v0::TextToAllResponse>, Status> {
        let res = self.request("textToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn arrow_to_all(
        &self,
        request: Request<trigger::v0::ArrowToAllRequest>,
    ) -> Result<Response<trigger::v0::ArrowToAllResponse>, Status> {
        let res = self.request("arrowToAll", request).await?;
        Ok(Response::new(res))
    }
}
