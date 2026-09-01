use serde::Serialize;
use stubs::controller;
use stubs::controller::v0::controller_service_server::ControllerService;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl ControllerService for MissionRpc {
    async fn set_alarm_state(
        &self,
        request: Request<controller::v0::SetAlarmStateRequest>,
    ) -> Result<Response<controller::v0::SetAlarmStateResponse>, Status> {
        let res = self.request("setAlarmState", request).await?;
        Ok(Response::new(res))
    }
    async fn get_detected_targets(
        &self,
        request: Request<controller::v0::GetDetectedTargetsRequest>,
    ) -> Result<Response<controller::v0::GetDetectedTargetsResponse>, Status> {
        let res = self.request("getDetectedTargets", request).await?;
        Ok(Response::new(res))
    }

    async fn has_task(
        &self,
        request: Request<controller::v0::HasTaskRequest>,
    ) -> Result<Response<controller::v0::HasTaskResponse>, Status> {
        let res = self.request("hasTask", request).await?;
        Ok(Response::new(res))
    }

    async fn set_on_off(
        &self,
        request: Request<controller::v0::SetOnOffRequest>,
    ) -> Result<Response<controller::v0::SetOnOffResponse>, Status> {
        let res = self.request("setOnOff", request).await?;
        Ok(Response::new(res))
    }

    async fn set_option(
        &self,
        request: Request<controller::v0::SetOptionRequest>,
    ) -> Result<Response<controller::v0::SetOptionResponse>, Status> {
        let res = self.request("setOption", request).await?;
        Ok(Response::new(res))
    }

    async fn is_target_detected(
        &self,
        request: Request<controller::v0::IsTargetDetectedRequest>,
    ) -> Result<Response<controller::v0::IsTargetDetectedResponse>, Status> {
        let res = self.request("isTargetDetected", request).await?;
        Ok(Response::new(res))
    }

    async fn know_target(
        &self,
        request: Request<controller::v0::KnowTargetRequest>,
    ) -> Result<Response<controller::v0::KnowTargetResponse>, Status> {
        let res = self.request("knowTarget", request).await?;
        Ok(Response::new(res))
    }

    async fn set_task(
        &self,
        request: Request<controller::v0::SetTaskRequest>,
    ) -> Result<Response<controller::v0::SetTaskResponse>, Status> {
        let req = request.into_inner();
        let task: serde_json::Value = serde_json::from_str(&req.task_json)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        #[derive(Serialize)]
        struct Payload {
            name: Option<controller::v0::set_task_request::Name>,
            task: serde_json::Value,
        }

        let _res: () = self
            .request(
                "setTask",
                Request::new(Payload {
                    name: req.name,
                    task,
                }),
            )
            .await?;
        Ok(Response::new(controller::v0::SetTaskResponse {}))
    }

    async fn push_task(
        &self,
        request: Request<controller::v0::PushTaskRequest>,
    ) -> Result<Response<controller::v0::PushTaskResponse>, Status> {
        let req = request.into_inner();
        let task: serde_json::Value = serde_json::from_str(&req.task_json)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        #[derive(Serialize)]
        struct Payload {
            name: Option<controller::v0::push_task_request::Name>,
            task: serde_json::Value,
        }

        let _res: () = self
            .request(
                "pushTask",
                Request::new(Payload {
                    name: req.name,
                    task,
                }),
            )
            .await?;
        Ok(Response::new(controller::v0::PushTaskResponse {}))
    }

    async fn pop_task(
        &self,
        request: Request<controller::v0::PopTaskRequest>,
    ) -> Result<Response<controller::v0::PopTaskResponse>, Status> {
        let _res: () = self.request("popTask", request).await?;
        Ok(Response::new(controller::v0::PopTaskResponse {}))
    }

    async fn reset_task(
        &self,
        request: Request<controller::v0::ResetTaskRequest>,
    ) -> Result<Response<controller::v0::ResetTaskResponse>, Status> {
        let _res: () = self.request("resetTask", request).await?;
        Ok(Response::new(controller::v0::ResetTaskResponse {}))
    }

    async fn set_command(
        &self,
        request: Request<controller::v0::SetCommandRequest>,
    ) -> Result<Response<controller::v0::SetCommandResponse>, Status> {
        let req = request.into_inner();
        let command: serde_json::Value = serde_json::from_str(&req.command_json)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        #[derive(Serialize)]
        struct Payload {
            name: Option<controller::v0::set_command_request::Name>,
            command: serde_json::Value,
        }

        let _res: () = self
            .request(
                "setCommand",
                Request::new(Payload {
                    name: req.name,
                    command,
                }),
            )
            .await?;
        Ok(Response::new(controller::v0::SetCommandResponse {}))
    }
}
