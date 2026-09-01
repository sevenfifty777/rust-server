# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [dcs/atmosphere/v0/atmosphere.proto](#dcs_atmosphere_v0_atmosphere-proto)
    - [GetTemperatureAndPressureRequest](#dcs-atmosphere-v0-GetTemperatureAndPressureRequest)
    - [GetTemperatureAndPressureResponse](#dcs-atmosphere-v0-GetTemperatureAndPressureResponse)
    - [GetWindRequest](#dcs-atmosphere-v0-GetWindRequest)
    - [GetWindResponse](#dcs-atmosphere-v0-GetWindResponse)
    - [GetWindWithTurbulenceRequest](#dcs-atmosphere-v0-GetWindWithTurbulenceRequest)
    - [GetWindWithTurbulenceResponse](#dcs-atmosphere-v0-GetWindWithTurbulenceResponse)
  
    - [AtmosphereService](#dcs-atmosphere-v0-AtmosphereService)
  
- [dcs/coalition/v0/coalition.proto](#dcs_coalition_v0_coalition-proto)
    - [AddGroupRequest](#dcs-coalition-v0-AddGroupRequest)
    - [AddGroupRequest.GroundGroupTemplate](#dcs-coalition-v0-AddGroupRequest-GroundGroupTemplate)
    - [AddGroupRequest.GroundUnitTemplate](#dcs-coalition-v0-AddGroupRequest-GroundUnitTemplate)
    - [AddGroupRequest.HelicopterGroupTemplate](#dcs-coalition-v0-AddGroupRequest-HelicopterGroupTemplate)
    - [AddGroupRequest.HelicopterUnitTemplate](#dcs-coalition-v0-AddGroupRequest-HelicopterUnitTemplate)
    - [AddGroupRequest.PlaneGroupTemplate](#dcs-coalition-v0-AddGroupRequest-PlaneGroupTemplate)
    - [AddGroupRequest.PlaneUnitTemplate](#dcs-coalition-v0-AddGroupRequest-PlaneUnitTemplate)
    - [AddGroupRequest.Point](#dcs-coalition-v0-AddGroupRequest-Point)
    - [AddGroupRequest.ShipGroupTemplate](#dcs-coalition-v0-AddGroupRequest-ShipGroupTemplate)
    - [AddGroupRequest.ShipUnitTemplate](#dcs-coalition-v0-AddGroupRequest-ShipUnitTemplate)
    - [AddGroupRequest.Task](#dcs-coalition-v0-AddGroupRequest-Task)
    - [AddGroupResponse](#dcs-coalition-v0-AddGroupResponse)
    - [AddLinkedStaticRequest](#dcs-coalition-v0-AddLinkedStaticRequest)
    - [AddLinkedStaticResponse](#dcs-coalition-v0-AddLinkedStaticResponse)
    - [AddStaticObjectRequest](#dcs-coalition-v0-AddStaticObjectRequest)
    - [AddStaticObjectResponse](#dcs-coalition-v0-AddStaticObjectResponse)
    - [GetBullseyeRequest](#dcs-coalition-v0-GetBullseyeRequest)
    - [GetBullseyeResponse](#dcs-coalition-v0-GetBullseyeResponse)
    - [GetGroupsRequest](#dcs-coalition-v0-GetGroupsRequest)
    - [GetGroupsResponse](#dcs-coalition-v0-GetGroupsResponse)
    - [GetPlayerUnitsRequest](#dcs-coalition-v0-GetPlayerUnitsRequest)
    - [GetPlayerUnitsResponse](#dcs-coalition-v0-GetPlayerUnitsResponse)
    - [GetStaticObjectsRequest](#dcs-coalition-v0-GetStaticObjectsRequest)
    - [GetStaticObjectsResponse](#dcs-coalition-v0-GetStaticObjectsResponse)
  
    - [AddGroupRequest.Point.AltitudeType](#dcs-coalition-v0-AddGroupRequest-Point-AltitudeType)
    - [AddGroupRequest.Point.PointType](#dcs-coalition-v0-AddGroupRequest-Point-PointType)
    - [AddGroupRequest.Skill](#dcs-coalition-v0-AddGroupRequest-Skill)
  
    - [CoalitionService](#dcs-coalition-v0-CoalitionService)
  
- [dcs/common/v0/common.proto](#dcs_common_v0_common-proto)
    - [Airbase](#dcs-common-v0-Airbase)
    - [Cargo](#dcs-common-v0-Cargo)
    - [Contact](#dcs-common-v0-Contact)
    - [Group](#dcs-common-v0-Group)
    - [Initiator](#dcs-common-v0-Initiator)
    - [InputPosition](#dcs-common-v0-InputPosition)
    - [MarkPanel](#dcs-common-v0-MarkPanel)
    - [Orientation](#dcs-common-v0-Orientation)
    - [Position](#dcs-common-v0-Position)
    - [Scenery](#dcs-common-v0-Scenery)
    - [Static](#dcs-common-v0-Static)
    - [Target](#dcs-common-v0-Target)
    - [Unit](#dcs-common-v0-Unit)
    - [Unknown](#dcs-common-v0-Unknown)
    - [Vector](#dcs-common-v0-Vector)
    - [Velocity](#dcs-common-v0-Velocity)
    - [Weapon](#dcs-common-v0-Weapon)
  
    - [AirbaseCategory](#dcs-common-v0-AirbaseCategory)
    - [Coalition](#dcs-common-v0-Coalition)
    - [Country](#dcs-common-v0-Country)
    - [GroupCategory](#dcs-common-v0-GroupCategory)
    - [ObjectCategory](#dcs-common-v0-ObjectCategory)
  
- [dcs/controller/v0/controller.proto](#dcs_controller_v0_controller-proto)
    - [GetDetectedTargetsRequest](#dcs-controller-v0-GetDetectedTargetsRequest)
    - [GetDetectedTargetsResponse](#dcs-controller-v0-GetDetectedTargetsResponse)
    - [HasTaskRequest](#dcs-controller-v0-HasTaskRequest)
    - [HasTaskResponse](#dcs-controller-v0-HasTaskResponse)
    - [IsTargetDetectedRequest](#dcs-controller-v0-IsTargetDetectedRequest)
    - [IsTargetDetectedResponse](#dcs-controller-v0-IsTargetDetectedResponse)
    - [KnowTargetRequest](#dcs-controller-v0-KnowTargetRequest)
    - [KnowTargetResponse](#dcs-controller-v0-KnowTargetResponse)
    - [PopTaskRequest](#dcs-controller-v0-PopTaskRequest)
    - [PopTaskResponse](#dcs-controller-v0-PopTaskResponse)
    - [PushTaskRequest](#dcs-controller-v0-PushTaskRequest)
    - [PushTaskResponse](#dcs-controller-v0-PushTaskResponse)
    - [ResetTaskRequest](#dcs-controller-v0-ResetTaskRequest)
    - [ResetTaskResponse](#dcs-controller-v0-ResetTaskResponse)
    - [SetAlarmStateRequest](#dcs-controller-v0-SetAlarmStateRequest)
    - [SetAlarmStateResponse](#dcs-controller-v0-SetAlarmStateResponse)
    - [SetCommandRequest](#dcs-controller-v0-SetCommandRequest)
    - [SetCommandResponse](#dcs-controller-v0-SetCommandResponse)
    - [SetOnOffRequest](#dcs-controller-v0-SetOnOffRequest)
    - [SetOnOffResponse](#dcs-controller-v0-SetOnOffResponse)
    - [SetOptionRequest](#dcs-controller-v0-SetOptionRequest)
    - [SetOptionResponse](#dcs-controller-v0-SetOptionResponse)
    - [SetTaskRequest](#dcs-controller-v0-SetTaskRequest)
    - [SetTaskResponse](#dcs-controller-v0-SetTaskResponse)
  
    - [GetDetectedTargetsRequest.DetectionType](#dcs-controller-v0-GetDetectedTargetsRequest-DetectionType)
    - [SetAlarmStateRequest.AlarmState](#dcs-controller-v0-SetAlarmStateRequest-AlarmState)
  
    - [ControllerService](#dcs-controller-v0-ControllerService)
  
- [dcs/custom/v0/custom.proto](#dcs_custom_v0_custom-proto)
    - [AbortMissionRequest](#dcs-custom-v0-AbortMissionRequest)
    - [AbortMissionResponse](#dcs-custom-v0-AbortMissionResponse)
    - [EvalRequest](#dcs-custom-v0-EvalRequest)
    - [EvalResponse](#dcs-custom-v0-EvalResponse)
    - [GetMagneticDeclinationRequest](#dcs-custom-v0-GetMagneticDeclinationRequest)
    - [GetMagneticDeclinationResponse](#dcs-custom-v0-GetMagneticDeclinationResponse)
    - [GetMissionStatusRequest](#dcs-custom-v0-GetMissionStatusRequest)
    - [GetMissionStatusResponse](#dcs-custom-v0-GetMissionStatusResponse)
    - [JoinMissionRequest](#dcs-custom-v0-JoinMissionRequest)
    - [JoinMissionResponse](#dcs-custom-v0-JoinMissionResponse)
    - [RequestMissionAssignmentRequest](#dcs-custom-v0-RequestMissionAssignmentRequest)
    - [RequestMissionAssignmentResponse](#dcs-custom-v0-RequestMissionAssignmentResponse)
  
    - [CustomService](#dcs-custom-v0-CustomService)
  
- [dcs/dcs.proto](#dcs_dcs-proto)
- [dcs/group/v0/group.proto](#dcs_group_v0_group-proto)
    - [ActivateRequest](#dcs-group-v0-ActivateRequest)
    - [ActivateResponse](#dcs-group-v0-ActivateResponse)
    - [DestroyRequest](#dcs-group-v0-DestroyRequest)
    - [DestroyResponse](#dcs-group-v0-DestroyResponse)
    - [EnableEmissionRequest](#dcs-group-v0-EnableEmissionRequest)
    - [EnableEmissionResponse](#dcs-group-v0-EnableEmissionResponse)
    - [ExistsRequest](#dcs-group-v0-ExistsRequest)
    - [ExistsResponse](#dcs-group-v0-ExistsResponse)
    - [GetGroupRequest](#dcs-group-v0-GetGroupRequest)
    - [GetGroupResponse](#dcs-group-v0-GetGroupResponse)
    - [GetSizeRequest](#dcs-group-v0-GetSizeRequest)
    - [GetSizeResponse](#dcs-group-v0-GetSizeResponse)
    - [GetUnitRequest](#dcs-group-v0-GetUnitRequest)
    - [GetUnitResponse](#dcs-group-v0-GetUnitResponse)
    - [GetUnitsRequest](#dcs-group-v0-GetUnitsRequest)
    - [GetUnitsResponse](#dcs-group-v0-GetUnitsResponse)
  
    - [GroupService](#dcs-group-v0-GroupService)
  
- [dcs/hook/v0/hook.proto](#dcs_hook_v0_hook-proto)
    - [BanDetails](#dcs-hook-v0-BanDetails)
    - [BanPlayerRequest](#dcs-hook-v0-BanPlayerRequest)
    - [BanPlayerResponse](#dcs-hook-v0-BanPlayerResponse)
    - [EvalRequest](#dcs-hook-v0-EvalRequest)
    - [EvalResponse](#dcs-hook-v0-EvalResponse)
    - [ExitProcessRequest](#dcs-hook-v0-ExitProcessRequest)
    - [ExitProcessResponse](#dcs-hook-v0-ExitProcessResponse)
    - [GetAvailableCoalitionsRequest](#dcs-hook-v0-GetAvailableCoalitionsRequest)
    - [GetAvailableCoalitionsResponse](#dcs-hook-v0-GetAvailableCoalitionsResponse)
    - [GetAvailableSlotsRequest](#dcs-hook-v0-GetAvailableSlotsRequest)
    - [GetAvailableSlotsResponse](#dcs-hook-v0-GetAvailableSlotsResponse)
    - [GetBallisticsCountRequest](#dcs-hook-v0-GetBallisticsCountRequest)
    - [GetBallisticsCountResponse](#dcs-hook-v0-GetBallisticsCountResponse)
    - [GetBannedPlayersRequest](#dcs-hook-v0-GetBannedPlayersRequest)
    - [GetBannedPlayersResponse](#dcs-hook-v0-GetBannedPlayersResponse)
    - [GetCurrentMissionRequest](#dcs-hook-v0-GetCurrentMissionRequest)
    - [GetCurrentMissionResponse](#dcs-hook-v0-GetCurrentMissionResponse)
    - [GetMissionDescriptionRequest](#dcs-hook-v0-GetMissionDescriptionRequest)
    - [GetMissionDescriptionResponse](#dcs-hook-v0-GetMissionDescriptionResponse)
    - [GetMissionFilenameRequest](#dcs-hook-v0-GetMissionFilenameRequest)
    - [GetMissionFilenameResponse](#dcs-hook-v0-GetMissionFilenameResponse)
    - [GetMissionNameRequest](#dcs-hook-v0-GetMissionNameRequest)
    - [GetMissionNameResponse](#dcs-hook-v0-GetMissionNameResponse)
    - [GetMissionOptionsRequest](#dcs-hook-v0-GetMissionOptionsRequest)
    - [GetMissionOptionsResponse](#dcs-hook-v0-GetMissionOptionsResponse)
    - [GetMissionResultRequest](#dcs-hook-v0-GetMissionResultRequest)
    - [GetMissionResultResponse](#dcs-hook-v0-GetMissionResultResponse)
    - [GetModelTimeRequest](#dcs-hook-v0-GetModelTimeRequest)
    - [GetModelTimeResponse](#dcs-hook-v0-GetModelTimeResponse)
    - [GetPausedRequest](#dcs-hook-v0-GetPausedRequest)
    - [GetPausedResponse](#dcs-hook-v0-GetPausedResponse)
    - [GetRealTimeRequest](#dcs-hook-v0-GetRealTimeRequest)
    - [GetRealTimeResponse](#dcs-hook-v0-GetRealTimeResponse)
    - [GetUnitPropertyRequest](#dcs-hook-v0-GetUnitPropertyRequest)
    - [GetUnitPropertyResponse](#dcs-hook-v0-GetUnitPropertyResponse)
    - [GetUnitTypeRequest](#dcs-hook-v0-GetUnitTypeRequest)
    - [GetUnitTypeResponse](#dcs-hook-v0-GetUnitTypeResponse)
    - [IsMultiplayerRequest](#dcs-hook-v0-IsMultiplayerRequest)
    - [IsMultiplayerResponse](#dcs-hook-v0-IsMultiplayerResponse)
    - [IsServerRequest](#dcs-hook-v0-IsServerRequest)
    - [IsServerResponse](#dcs-hook-v0-IsServerResponse)
    - [LoadMissionRequest](#dcs-hook-v0-LoadMissionRequest)
    - [LoadMissionResponse](#dcs-hook-v0-LoadMissionResponse)
    - [LoadNextMissionRequest](#dcs-hook-v0-LoadNextMissionRequest)
    - [LoadNextMissionResponse](#dcs-hook-v0-LoadNextMissionResponse)
    - [ReloadCurrentMissionRequest](#dcs-hook-v0-ReloadCurrentMissionRequest)
    - [ReloadCurrentMissionResponse](#dcs-hook-v0-ReloadCurrentMissionResponse)
    - [SetPausedRequest](#dcs-hook-v0-SetPausedRequest)
    - [SetPausedResponse](#dcs-hook-v0-SetPausedResponse)
    - [StopMissionRequest](#dcs-hook-v0-StopMissionRequest)
    - [StopMissionResponse](#dcs-hook-v0-StopMissionResponse)
    - [UnbanPlayerRequest](#dcs-hook-v0-UnbanPlayerRequest)
    - [UnbanPlayerResponse](#dcs-hook-v0-UnbanPlayerResponse)
  
    - [HookService](#dcs-hook-v0-HookService)
  
- [dcs/land/v0/land.proto](#dcs_land_v0_land-proto)
    - [FindPathOnRoadsRequest](#dcs-land-v0-FindPathOnRoadsRequest)
    - [FindPathOnRoadsResponse](#dcs-land-v0-FindPathOnRoadsResponse)
    - [GetClosestPointOnRoadsRequest](#dcs-land-v0-GetClosestPointOnRoadsRequest)
    - [GetClosestPointOnRoadsResponse](#dcs-land-v0-GetClosestPointOnRoadsResponse)
    - [GetIPRequest](#dcs-land-v0-GetIPRequest)
    - [GetIPResponse](#dcs-land-v0-GetIPResponse)
    - [GetSurfaceHeightWithSeabedRequest](#dcs-land-v0-GetSurfaceHeightWithSeabedRequest)
    - [GetSurfaceHeightWithSeabedResponse](#dcs-land-v0-GetSurfaceHeightWithSeabedResponse)
    - [GetSurfaceTypeRequest](#dcs-land-v0-GetSurfaceTypeRequest)
    - [GetSurfaceTypeResponse](#dcs-land-v0-GetSurfaceTypeResponse)
    - [GetTerrainHeightRequest](#dcs-land-v0-GetTerrainHeightRequest)
    - [GetTerrainHeightResponse](#dcs-land-v0-GetTerrainHeightResponse)
    - [IsVisibleRequest](#dcs-land-v0-IsVisibleRequest)
    - [IsVisibleResponse](#dcs-land-v0-IsVisibleResponse)
    - [ProfileRequest](#dcs-land-v0-ProfileRequest)
    - [ProfileResponse](#dcs-land-v0-ProfileResponse)
  
    - [SurfaceType](#dcs-land-v0-SurfaceType)
  
    - [LandService](#dcs-land-v0-LandService)
  
- [dcs/metadata/v0/metadata.proto](#dcs_metadata_v0_metadata-proto)
    - [GetHealthRequest](#dcs-metadata-v0-GetHealthRequest)
    - [GetHealthResponse](#dcs-metadata-v0-GetHealthResponse)
    - [GetVersionRequest](#dcs-metadata-v0-GetVersionRequest)
    - [GetVersionResponse](#dcs-metadata-v0-GetVersionResponse)
  
    - [MetadataService](#dcs-metadata-v0-MetadataService)
  
- [dcs/mission/v0/mission.proto](#dcs_mission_v0_mission-proto)
    - [AddCoalitionCommandRequest](#dcs-mission-v0-AddCoalitionCommandRequest)
    - [AddCoalitionCommandResponse](#dcs-mission-v0-AddCoalitionCommandResponse)
    - [AddCoalitionCommandSubMenuRequest](#dcs-mission-v0-AddCoalitionCommandSubMenuRequest)
    - [AddCoalitionCommandSubMenuResponse](#dcs-mission-v0-AddCoalitionCommandSubMenuResponse)
    - [AddGroupCommandRequest](#dcs-mission-v0-AddGroupCommandRequest)
    - [AddGroupCommandResponse](#dcs-mission-v0-AddGroupCommandResponse)
    - [AddGroupCommandSubMenuRequest](#dcs-mission-v0-AddGroupCommandSubMenuRequest)
    - [AddGroupCommandSubMenuResponse](#dcs-mission-v0-AddGroupCommandSubMenuResponse)
    - [AddMissionCommandRequest](#dcs-mission-v0-AddMissionCommandRequest)
    - [AddMissionCommandResponse](#dcs-mission-v0-AddMissionCommandResponse)
    - [AddMissionCommandSubMenuRequest](#dcs-mission-v0-AddMissionCommandSubMenuRequest)
    - [AddMissionCommandSubMenuResponse](#dcs-mission-v0-AddMissionCommandSubMenuResponse)
    - [GetScenarioCurrentTimeRequest](#dcs-mission-v0-GetScenarioCurrentTimeRequest)
    - [GetScenarioCurrentTimeResponse](#dcs-mission-v0-GetScenarioCurrentTimeResponse)
    - [GetScenarioStartTimeRequest](#dcs-mission-v0-GetScenarioStartTimeRequest)
    - [GetScenarioStartTimeResponse](#dcs-mission-v0-GetScenarioStartTimeResponse)
    - [GetSessionIdRequest](#dcs-mission-v0-GetSessionIdRequest)
    - [GetSessionIdResponse](#dcs-mission-v0-GetSessionIdResponse)
    - [RemoveCoalitionCommandItemRequest](#dcs-mission-v0-RemoveCoalitionCommandItemRequest)
    - [RemoveCoalitionCommandItemResponse](#dcs-mission-v0-RemoveCoalitionCommandItemResponse)
    - [RemoveGroupCommandItemRequest](#dcs-mission-v0-RemoveGroupCommandItemRequest)
    - [RemoveGroupCommandItemResponse](#dcs-mission-v0-RemoveGroupCommandItemResponse)
    - [RemoveMissionCommandItemRequest](#dcs-mission-v0-RemoveMissionCommandItemRequest)
    - [RemoveMissionCommandItemResponse](#dcs-mission-v0-RemoveMissionCommandItemResponse)
    - [StreamEventsRequest](#dcs-mission-v0-StreamEventsRequest)
    - [StreamEventsResponse](#dcs-mission-v0-StreamEventsResponse)
    - [StreamEventsResponse.BaseCaptureEvent](#dcs-mission-v0-StreamEventsResponse-BaseCaptureEvent)
    - [StreamEventsResponse.BdaEvent](#dcs-mission-v0-StreamEventsResponse-BdaEvent)
    - [StreamEventsResponse.BirthEvent](#dcs-mission-v0-StreamEventsResponse-BirthEvent)
    - [StreamEventsResponse.CoalitionCommandEvent](#dcs-mission-v0-StreamEventsResponse-CoalitionCommandEvent)
    - [StreamEventsResponse.ConnectEvent](#dcs-mission-v0-StreamEventsResponse-ConnectEvent)
    - [StreamEventsResponse.CrashEvent](#dcs-mission-v0-StreamEventsResponse-CrashEvent)
    - [StreamEventsResponse.DeadEvent](#dcs-mission-v0-StreamEventsResponse-DeadEvent)
    - [StreamEventsResponse.DetailedFailureEvent](#dcs-mission-v0-StreamEventsResponse-DetailedFailureEvent)
    - [StreamEventsResponse.DiscardChairAfterEjectionEvent](#dcs-mission-v0-StreamEventsResponse-DiscardChairAfterEjectionEvent)
    - [StreamEventsResponse.DisconnectEvent](#dcs-mission-v0-StreamEventsResponse-DisconnectEvent)
    - [StreamEventsResponse.EjectionEvent](#dcs-mission-v0-StreamEventsResponse-EjectionEvent)
    - [StreamEventsResponse.EngineShutdownEvent](#dcs-mission-v0-StreamEventsResponse-EngineShutdownEvent)
    - [StreamEventsResponse.EngineStartupEvent](#dcs-mission-v0-StreamEventsResponse-EngineStartupEvent)
    - [StreamEventsResponse.GroupCommandEvent](#dcs-mission-v0-StreamEventsResponse-GroupCommandEvent)
    - [StreamEventsResponse.HitEvent](#dcs-mission-v0-StreamEventsResponse-HitEvent)
    - [StreamEventsResponse.HumanFailureEvent](#dcs-mission-v0-StreamEventsResponse-HumanFailureEvent)
    - [StreamEventsResponse.KillEvent](#dcs-mission-v0-StreamEventsResponse-KillEvent)
    - [StreamEventsResponse.LandEvent](#dcs-mission-v0-StreamEventsResponse-LandEvent)
    - [StreamEventsResponse.LandingAfterEjectionEvent](#dcs-mission-v0-StreamEventsResponse-LandingAfterEjectionEvent)
    - [StreamEventsResponse.LandingQualityMarkEvent](#dcs-mission-v0-StreamEventsResponse-LandingQualityMarkEvent)
    - [StreamEventsResponse.MarkAddEvent](#dcs-mission-v0-StreamEventsResponse-MarkAddEvent)
    - [StreamEventsResponse.MarkChangeEvent](#dcs-mission-v0-StreamEventsResponse-MarkChangeEvent)
    - [StreamEventsResponse.MarkRemoveEvent](#dcs-mission-v0-StreamEventsResponse-MarkRemoveEvent)
    - [StreamEventsResponse.MissionCommandEvent](#dcs-mission-v0-StreamEventsResponse-MissionCommandEvent)
    - [StreamEventsResponse.MissionEndEvent](#dcs-mission-v0-StreamEventsResponse-MissionEndEvent)
    - [StreamEventsResponse.MissionStartEvent](#dcs-mission-v0-StreamEventsResponse-MissionStartEvent)
    - [StreamEventsResponse.PilotDeadEvent](#dcs-mission-v0-StreamEventsResponse-PilotDeadEvent)
    - [StreamEventsResponse.PlayerChangeSlotEvent](#dcs-mission-v0-StreamEventsResponse-PlayerChangeSlotEvent)
    - [StreamEventsResponse.PlayerCommentEvent](#dcs-mission-v0-StreamEventsResponse-PlayerCommentEvent)
    - [StreamEventsResponse.PlayerEnterUnitEvent](#dcs-mission-v0-StreamEventsResponse-PlayerEnterUnitEvent)
    - [StreamEventsResponse.PlayerLeaveUnitEvent](#dcs-mission-v0-StreamEventsResponse-PlayerLeaveUnitEvent)
    - [StreamEventsResponse.PlayerSendChatEvent](#dcs-mission-v0-StreamEventsResponse-PlayerSendChatEvent)
    - [StreamEventsResponse.RefuelingEvent](#dcs-mission-v0-StreamEventsResponse-RefuelingEvent)
    - [StreamEventsResponse.RefuelingStopEvent](#dcs-mission-v0-StreamEventsResponse-RefuelingStopEvent)
    - [StreamEventsResponse.RunwayTakeoffEvent](#dcs-mission-v0-StreamEventsResponse-RunwayTakeoffEvent)
    - [StreamEventsResponse.RunwayTouchEvent](#dcs-mission-v0-StreamEventsResponse-RunwayTouchEvent)
    - [StreamEventsResponse.ScoreEvent](#dcs-mission-v0-StreamEventsResponse-ScoreEvent)
    - [StreamEventsResponse.ShootingEndEvent](#dcs-mission-v0-StreamEventsResponse-ShootingEndEvent)
    - [StreamEventsResponse.ShootingStartEvent](#dcs-mission-v0-StreamEventsResponse-ShootingStartEvent)
    - [StreamEventsResponse.ShotEvent](#dcs-mission-v0-StreamEventsResponse-ShotEvent)
    - [StreamEventsResponse.SimulationFpsEvent](#dcs-mission-v0-StreamEventsResponse-SimulationFpsEvent)
    - [StreamEventsResponse.SrsConnectEvent](#dcs-mission-v0-StreamEventsResponse-SrsConnectEvent)
    - [StreamEventsResponse.SrsDisconnectEvent](#dcs-mission-v0-StreamEventsResponse-SrsDisconnectEvent)
    - [StreamEventsResponse.TakeoffEvent](#dcs-mission-v0-StreamEventsResponse-TakeoffEvent)
    - [StreamEventsResponse.TookControlEvent](#dcs-mission-v0-StreamEventsResponse-TookControlEvent)
    - [StreamEventsResponse.TriggerZoneEvent](#dcs-mission-v0-StreamEventsResponse-TriggerZoneEvent)
    - [StreamEventsResponse.TtsEvent](#dcs-mission-v0-StreamEventsResponse-TtsEvent)
    - [StreamEventsResponse.UnitLostEvent](#dcs-mission-v0-StreamEventsResponse-UnitLostEvent)
    - [StreamEventsResponse.WeaponAddEvent](#dcs-mission-v0-StreamEventsResponse-WeaponAddEvent)
    - [StreamUnitsRequest](#dcs-mission-v0-StreamUnitsRequest)
    - [StreamUnitsResponse](#dcs-mission-v0-StreamUnitsResponse)
    - [StreamUnitsResponse.UnitGone](#dcs-mission-v0-StreamUnitsResponse-UnitGone)
  
    - [StreamEventsResponse.DisconnectReason](#dcs-mission-v0-StreamEventsResponse-DisconnectReason)
  
    - [MissionService](#dcs-mission-v0-MissionService)
  
- [dcs/net/v0/net.proto](#dcs_net_v0_net-proto)
    - [ForcePlayerSlotRequest](#dcs-net-v0-ForcePlayerSlotRequest)
    - [ForcePlayerSlotResponse](#dcs-net-v0-ForcePlayerSlotResponse)
    - [GetPlayersRequest](#dcs-net-v0-GetPlayersRequest)
    - [GetPlayersResponse](#dcs-net-v0-GetPlayersResponse)
    - [GetPlayersResponse.GetPlayerInfo](#dcs-net-v0-GetPlayersResponse-GetPlayerInfo)
    - [KickPlayerRequest](#dcs-net-v0-KickPlayerRequest)
    - [KickPlayerResponse](#dcs-net-v0-KickPlayerResponse)
    - [SendChatRequest](#dcs-net-v0-SendChatRequest)
    - [SendChatResponse](#dcs-net-v0-SendChatResponse)
    - [SendChatToRequest](#dcs-net-v0-SendChatToRequest)
    - [SendChatToResponse](#dcs-net-v0-SendChatToResponse)
  
    - [NetService](#dcs-net-v0-NetService)
  
- [dcs/spot/v0/spot.proto](#dcs_spot_v0_spot-proto)
    - [CreateInfraRedRequest](#dcs-spot-v0-CreateInfraRedRequest)
    - [CreateInfraRedResponse](#dcs-spot-v0-CreateInfraRedResponse)
    - [CreateLaserRequest](#dcs-spot-v0-CreateLaserRequest)
    - [CreateLaserResponse](#dcs-spot-v0-CreateLaserResponse)
    - [DestroyRequest](#dcs-spot-v0-DestroyRequest)
    - [DestroyResponse](#dcs-spot-v0-DestroyResponse)
    - [GetCategoryRequest](#dcs-spot-v0-GetCategoryRequest)
    - [GetCategoryResponse](#dcs-spot-v0-GetCategoryResponse)
    - [GetCodeRequest](#dcs-spot-v0-GetCodeRequest)
    - [GetCodeResponse](#dcs-spot-v0-GetCodeResponse)
    - [GetPointRequest](#dcs-spot-v0-GetPointRequest)
    - [GetPointResponse](#dcs-spot-v0-GetPointResponse)
    - [SetCodeRequest](#dcs-spot-v0-SetCodeRequest)
    - [SetCodeResponse](#dcs-spot-v0-SetCodeResponse)
    - [SetPointRequest](#dcs-spot-v0-SetPointRequest)
    - [SetPointResponse](#dcs-spot-v0-SetPointResponse)
  
    - [SpotService](#dcs-spot-v0-SpotService)
  
- [dcs/srs/v0/srs.proto](#dcs_srs_v0_srs-proto)
    - [GetClientsRequest](#dcs-srs-v0-GetClientsRequest)
    - [GetClientsResponse](#dcs-srs-v0-GetClientsResponse)
    - [GetClientsResponse.Client](#dcs-srs-v0-GetClientsResponse-Client)
    - [TransmitRequest](#dcs-srs-v0-TransmitRequest)
    - [TransmitRequest.Aws](#dcs-srs-v0-TransmitRequest-Aws)
    - [TransmitRequest.Azure](#dcs-srs-v0-TransmitRequest-Azure)
    - [TransmitRequest.GCloud](#dcs-srs-v0-TransmitRequest-GCloud)
    - [TransmitRequest.Windows](#dcs-srs-v0-TransmitRequest-Windows)
    - [TransmitResponse](#dcs-srs-v0-TransmitResponse)
  
    - [SrsService](#dcs-srs-v0-SrsService)
  
- [dcs/timer/v0/timer.proto](#dcs_timer_v0_timer-proto)
    - [GetAbsoluteTimeRequest](#dcs-timer-v0-GetAbsoluteTimeRequest)
    - [GetAbsoluteTimeResponse](#dcs-timer-v0-GetAbsoluteTimeResponse)
    - [GetTimeRequest](#dcs-timer-v0-GetTimeRequest)
    - [GetTimeResponse](#dcs-timer-v0-GetTimeResponse)
    - [GetTimeZeroRequest](#dcs-timer-v0-GetTimeZeroRequest)
    - [GetTimeZeroResponse](#dcs-timer-v0-GetTimeZeroResponse)
  
    - [TimerService](#dcs-timer-v0-TimerService)
  
- [dcs/trigger/v0/trigger.proto](#dcs_trigger_v0_trigger-proto)
    - [ActivateGroupRequest](#dcs-trigger-v0-ActivateGroupRequest)
    - [ActivateGroupResponse](#dcs-trigger-v0-ActivateGroupResponse)
    - [ArrowToAllRequest](#dcs-trigger-v0-ArrowToAllRequest)
    - [ArrowToAllResponse](#dcs-trigger-v0-ArrowToAllResponse)
    - [CircleToAllRequest](#dcs-trigger-v0-CircleToAllRequest)
    - [CircleToAllResponse](#dcs-trigger-v0-CircleToAllResponse)
    - [Color](#dcs-trigger-v0-Color)
    - [DeactivateGroupRequest](#dcs-trigger-v0-DeactivateGroupRequest)
    - [DeactivateGroupResponse](#dcs-trigger-v0-DeactivateGroupResponse)
    - [EffectSmokeBigRequest](#dcs-trigger-v0-EffectSmokeBigRequest)
    - [EffectSmokeBigResponse](#dcs-trigger-v0-EffectSmokeBigResponse)
    - [EffectSmokeStopRequest](#dcs-trigger-v0-EffectSmokeStopRequest)
    - [EffectSmokeStopResponse](#dcs-trigger-v0-EffectSmokeStopResponse)
    - [ExplosionRequest](#dcs-trigger-v0-ExplosionRequest)
    - [ExplosionResponse](#dcs-trigger-v0-ExplosionResponse)
    - [GetUserFlagRequest](#dcs-trigger-v0-GetUserFlagRequest)
    - [GetUserFlagResponse](#dcs-trigger-v0-GetUserFlagResponse)
    - [GetZoneRequest](#dcs-trigger-v0-GetZoneRequest)
    - [GetZoneResponse](#dcs-trigger-v0-GetZoneResponse)
    - [GroupContinueMovingRequest](#dcs-trigger-v0-GroupContinueMovingRequest)
    - [GroupContinueMovingResponse](#dcs-trigger-v0-GroupContinueMovingResponse)
    - [GroupStopMovingRequest](#dcs-trigger-v0-GroupStopMovingRequest)
    - [GroupStopMovingResponse](#dcs-trigger-v0-GroupStopMovingResponse)
    - [IlluminationBombRequest](#dcs-trigger-v0-IlluminationBombRequest)
    - [IlluminationBombResponse](#dcs-trigger-v0-IlluminationBombResponse)
    - [LineToAllRequest](#dcs-trigger-v0-LineToAllRequest)
    - [LineToAllResponse](#dcs-trigger-v0-LineToAllResponse)
    - [MarkToAllRequest](#dcs-trigger-v0-MarkToAllRequest)
    - [MarkToAllResponse](#dcs-trigger-v0-MarkToAllResponse)
    - [MarkToCoalitionRequest](#dcs-trigger-v0-MarkToCoalitionRequest)
    - [MarkToCoalitionResponse](#dcs-trigger-v0-MarkToCoalitionResponse)
    - [MarkToGroupRequest](#dcs-trigger-v0-MarkToGroupRequest)
    - [MarkToGroupResponse](#dcs-trigger-v0-MarkToGroupResponse)
    - [MarkupToAllRequest](#dcs-trigger-v0-MarkupToAllRequest)
    - [MarkupToAllResponse](#dcs-trigger-v0-MarkupToAllResponse)
    - [MarkupToCoalitionRequest](#dcs-trigger-v0-MarkupToCoalitionRequest)
    - [MarkupToCoalitionResponse](#dcs-trigger-v0-MarkupToCoalitionResponse)
    - [OutTextForCoalitionRequest](#dcs-trigger-v0-OutTextForCoalitionRequest)
    - [OutTextForCoalitionResponse](#dcs-trigger-v0-OutTextForCoalitionResponse)
    - [OutTextForGroupRequest](#dcs-trigger-v0-OutTextForGroupRequest)
    - [OutTextForGroupResponse](#dcs-trigger-v0-OutTextForGroupResponse)
    - [OutTextForUnitRequest](#dcs-trigger-v0-OutTextForUnitRequest)
    - [OutTextForUnitResponse](#dcs-trigger-v0-OutTextForUnitResponse)
    - [OutTextRequest](#dcs-trigger-v0-OutTextRequest)
    - [OutTextResponse](#dcs-trigger-v0-OutTextResponse)
    - [PushAITaskRequest](#dcs-trigger-v0-PushAITaskRequest)
    - [PushAITaskResponse](#dcs-trigger-v0-PushAITaskResponse)
    - [QuadToAllRequest](#dcs-trigger-v0-QuadToAllRequest)
    - [QuadToAllResponse](#dcs-trigger-v0-QuadToAllResponse)
    - [RectToAllRequest](#dcs-trigger-v0-RectToAllRequest)
    - [RectToAllResponse](#dcs-trigger-v0-RectToAllResponse)
    - [RemoveMarkRequest](#dcs-trigger-v0-RemoveMarkRequest)
    - [RemoveMarkResponse](#dcs-trigger-v0-RemoveMarkResponse)
    - [SetAITaskRequest](#dcs-trigger-v0-SetAITaskRequest)
    - [SetAITaskResponse](#dcs-trigger-v0-SetAITaskResponse)
    - [SetGroupAIOffRequest](#dcs-trigger-v0-SetGroupAIOffRequest)
    - [SetGroupAIOffResponse](#dcs-trigger-v0-SetGroupAIOffResponse)
    - [SetGroupAIOnRequest](#dcs-trigger-v0-SetGroupAIOnRequest)
    - [SetGroupAIOnResponse](#dcs-trigger-v0-SetGroupAIOnResponse)
    - [SetMarkupColorFillRequest](#dcs-trigger-v0-SetMarkupColorFillRequest)
    - [SetMarkupColorFillResponse](#dcs-trigger-v0-SetMarkupColorFillResponse)
    - [SetMarkupColorRequest](#dcs-trigger-v0-SetMarkupColorRequest)
    - [SetMarkupColorResponse](#dcs-trigger-v0-SetMarkupColorResponse)
    - [SetMarkupFontSizeRequest](#dcs-trigger-v0-SetMarkupFontSizeRequest)
    - [SetMarkupFontSizeResponse](#dcs-trigger-v0-SetMarkupFontSizeResponse)
    - [SetMarkupPositionEndRequest](#dcs-trigger-v0-SetMarkupPositionEndRequest)
    - [SetMarkupPositionEndResponse](#dcs-trigger-v0-SetMarkupPositionEndResponse)
    - [SetMarkupPositionStartRequest](#dcs-trigger-v0-SetMarkupPositionStartRequest)
    - [SetMarkupPositionStartResponse](#dcs-trigger-v0-SetMarkupPositionStartResponse)
    - [SetMarkupRadiusRequest](#dcs-trigger-v0-SetMarkupRadiusRequest)
    - [SetMarkupRadiusResponse](#dcs-trigger-v0-SetMarkupRadiusResponse)
    - [SetMarkupTextRequest](#dcs-trigger-v0-SetMarkupTextRequest)
    - [SetMarkupTextResponse](#dcs-trigger-v0-SetMarkupTextResponse)
    - [SetMarkupTypeLineRequest](#dcs-trigger-v0-SetMarkupTypeLineRequest)
    - [SetMarkupTypeLineResponse](#dcs-trigger-v0-SetMarkupTypeLineResponse)
    - [SetUnitInternalCargoRequest](#dcs-trigger-v0-SetUnitInternalCargoRequest)
    - [SetUnitInternalCargoResponse](#dcs-trigger-v0-SetUnitInternalCargoResponse)
    - [SetUserFlagRequest](#dcs-trigger-v0-SetUserFlagRequest)
    - [SetUserFlagResponse](#dcs-trigger-v0-SetUserFlagResponse)
    - [SignalFlareRequest](#dcs-trigger-v0-SignalFlareRequest)
    - [SignalFlareResponse](#dcs-trigger-v0-SignalFlareResponse)
    - [SmokeRequest](#dcs-trigger-v0-SmokeRequest)
    - [SmokeResponse](#dcs-trigger-v0-SmokeResponse)
    - [TextToAllRequest](#dcs-trigger-v0-TextToAllRequest)
    - [TextToAllResponse](#dcs-trigger-v0-TextToAllResponse)
  
    - [EffectSmokeBigRequest.SmokePreset](#dcs-trigger-v0-EffectSmokeBigRequest-SmokePreset)
    - [LineType](#dcs-trigger-v0-LineType)
    - [Shape](#dcs-trigger-v0-Shape)
    - [SignalFlareRequest.FlareColor](#dcs-trigger-v0-SignalFlareRequest-FlareColor)
    - [SmokeRequest.SmokeColor](#dcs-trigger-v0-SmokeRequest-SmokeColor)
  
    - [TriggerService](#dcs-trigger-v0-TriggerService)
  
- [dcs/unit/v0/unit.proto](#dcs_unit_v0_unit-proto)
    - [AmmoItem](#dcs-unit-v0-AmmoItem)
    - [DestroyRequest](#dcs-unit-v0-DestroyRequest)
    - [DestroyResponse](#dcs-unit-v0-DestroyResponse)
    - [DetectionDistanceAir](#dcs-unit-v0-DetectionDistanceAir)
    - [GetAmmoRequest](#dcs-unit-v0-GetAmmoRequest)
    - [GetAmmoResponse](#dcs-unit-v0-GetAmmoResponse)
    - [GetCountryRequest](#dcs-unit-v0-GetCountryRequest)
    - [GetCountryResponse](#dcs-unit-v0-GetCountryResponse)
    - [GetDescByNameRequest](#dcs-unit-v0-GetDescByNameRequest)
    - [GetDescByNameResponse](#dcs-unit-v0-GetDescByNameResponse)
    - [GetDescentCapacityRequest](#dcs-unit-v0-GetDescentCapacityRequest)
    - [GetDescentCapacityResponse](#dcs-unit-v0-GetDescentCapacityResponse)
    - [GetDescriptorRequest](#dcs-unit-v0-GetDescriptorRequest)
    - [GetDescriptorResponse](#dcs-unit-v0-GetDescriptorResponse)
    - [GetDrawArgumentValueRequest](#dcs-unit-v0-GetDrawArgumentValueRequest)
    - [GetDrawArgumentValueResponse](#dcs-unit-v0-GetDrawArgumentValueResponse)
    - [GetFuelRequest](#dcs-unit-v0-GetFuelRequest)
    - [GetFuelResponse](#dcs-unit-v0-GetFuelResponse)
    - [GetGroupRequest](#dcs-unit-v0-GetGroupRequest)
    - [GetGroupResponse](#dcs-unit-v0-GetGroupResponse)
    - [GetLife0Request](#dcs-unit-v0-GetLife0Request)
    - [GetLife0Response](#dcs-unit-v0-GetLife0Response)
    - [GetLifeRequest](#dcs-unit-v0-GetLifeRequest)
    - [GetLifeResponse](#dcs-unit-v0-GetLifeResponse)
    - [GetNearestCargosRequest](#dcs-unit-v0-GetNearestCargosRequest)
    - [GetNearestCargosResponse](#dcs-unit-v0-GetNearestCargosResponse)
    - [GetNumberRequest](#dcs-unit-v0-GetNumberRequest)
    - [GetNumberResponse](#dcs-unit-v0-GetNumberResponse)
    - [GetPlayerNameRequest](#dcs-unit-v0-GetPlayerNameRequest)
    - [GetPlayerNameResponse](#dcs-unit-v0-GetPlayerNameResponse)
    - [GetPositionRequest](#dcs-unit-v0-GetPositionRequest)
    - [GetPositionResponse](#dcs-unit-v0-GetPositionResponse)
    - [GetRadarRequest](#dcs-unit-v0-GetRadarRequest)
    - [GetRadarResponse](#dcs-unit-v0-GetRadarResponse)
    - [GetRequest](#dcs-unit-v0-GetRequest)
    - [GetResponse](#dcs-unit-v0-GetResponse)
    - [GetSensorsRequest](#dcs-unit-v0-GetSensorsRequest)
    - [GetSensorsResponse](#dcs-unit-v0-GetSensorsResponse)
    - [GetTransformRequest](#dcs-unit-v0-GetTransformRequest)
    - [GetTransformResponse](#dcs-unit-v0-GetTransformResponse)
    - [HasSensorsRequest](#dcs-unit-v0-HasSensorsRequest)
    - [HasSensorsResponse](#dcs-unit-v0-HasSensorsResponse)
    - [Hemisphere](#dcs-unit-v0-Hemisphere)
    - [InAirRequest](#dcs-unit-v0-InAirRequest)
    - [InAirResponse](#dcs-unit-v0-InAirResponse)
    - [IrstSensor](#dcs-unit-v0-IrstSensor)
    - [IsActiveRequest](#dcs-unit-v0-IsActiveRequest)
    - [IsActiveResponse](#dcs-unit-v0-IsActiveResponse)
    - [OpticalSensor](#dcs-unit-v0-OpticalSensor)
    - [RadarSensor](#dcs-unit-v0-RadarSensor)
    - [RwrSensor](#dcs-unit-v0-RwrSensor)
    - [Sensor](#dcs-unit-v0-Sensor)
    - [SensorCategory](#dcs-unit-v0-SensorCategory)
    - [SetEmissionRequest](#dcs-unit-v0-SetEmissionRequest)
    - [SetEmissionResponse](#dcs-unit-v0-SetEmissionResponse)
  
    - [UnitService](#dcs-unit-v0-UnitService)
  
- [dcs/warehouse/v0/warehouse.proto](#dcs_warehouse_v0_warehouse-proto)
    - [AddItemRequest](#dcs-warehouse-v0-AddItemRequest)
    - [AddItemResponse](#dcs-warehouse-v0-AddItemResponse)
    - [AddLiquidRequest](#dcs-warehouse-v0-AddLiquidRequest)
    - [AddLiquidResponse](#dcs-warehouse-v0-AddLiquidResponse)
    - [GetInventoryRequest](#dcs-warehouse-v0-GetInventoryRequest)
    - [GetInventoryResponse](#dcs-warehouse-v0-GetInventoryResponse)
    - [GetItemCountRequest](#dcs-warehouse-v0-GetItemCountRequest)
    - [GetItemCountResponse](#dcs-warehouse-v0-GetItemCountResponse)
    - [GetLiquidAmountRequest](#dcs-warehouse-v0-GetLiquidAmountRequest)
    - [GetLiquidAmountResponse](#dcs-warehouse-v0-GetLiquidAmountResponse)
    - [GetOwnerRequest](#dcs-warehouse-v0-GetOwnerRequest)
    - [GetOwnerResponse](#dcs-warehouse-v0-GetOwnerResponse)
    - [RemoveItemRequest](#dcs-warehouse-v0-RemoveItemRequest)
    - [RemoveItemResponse](#dcs-warehouse-v0-RemoveItemResponse)
    - [SetItemRequest](#dcs-warehouse-v0-SetItemRequest)
    - [SetItemResponse](#dcs-warehouse-v0-SetItemResponse)
    - [SetLiquidAmountRequest](#dcs-warehouse-v0-SetLiquidAmountRequest)
    - [SetLiquidAmountResponse](#dcs-warehouse-v0-SetLiquidAmountResponse)
  
    - [WarehouseService](#dcs-warehouse-v0-WarehouseService)
  
- [dcs/weapon/v0/weapon.proto](#dcs_weapon_v0_weapon-proto)
    - [DestroyRequest](#dcs-weapon-v0-DestroyRequest)
    - [DestroyResponse](#dcs-weapon-v0-DestroyResponse)
    - [GetCategoryRequest](#dcs-weapon-v0-GetCategoryRequest)
    - [GetCategoryResponse](#dcs-weapon-v0-GetCategoryResponse)
    - [GetCoalitionRequest](#dcs-weapon-v0-GetCoalitionRequest)
    - [GetCoalitionResponse](#dcs-weapon-v0-GetCoalitionResponse)
    - [GetCountryRequest](#dcs-weapon-v0-GetCountryRequest)
    - [GetCountryResponse](#dcs-weapon-v0-GetCountryResponse)
    - [GetDescRequest](#dcs-weapon-v0-GetDescRequest)
    - [GetDescResponse](#dcs-weapon-v0-GetDescResponse)
    - [GetLauncherRequest](#dcs-weapon-v0-GetLauncherRequest)
    - [GetLauncherResponse](#dcs-weapon-v0-GetLauncherResponse)
    - [GetNameRequest](#dcs-weapon-v0-GetNameRequest)
    - [GetNameResponse](#dcs-weapon-v0-GetNameResponse)
    - [GetPointRequest](#dcs-weapon-v0-GetPointRequest)
    - [GetPointResponse](#dcs-weapon-v0-GetPointResponse)
    - [GetPositionRequest](#dcs-weapon-v0-GetPositionRequest)
    - [GetPositionResponse](#dcs-weapon-v0-GetPositionResponse)
    - [GetTargetRequest](#dcs-weapon-v0-GetTargetRequest)
    - [GetTargetResponse](#dcs-weapon-v0-GetTargetResponse)
    - [GetTypeNameRequest](#dcs-weapon-v0-GetTypeNameRequest)
    - [GetTypeNameResponse](#dcs-weapon-v0-GetTypeNameResponse)
    - [GetVelocityRequest](#dcs-weapon-v0-GetVelocityRequest)
    - [GetVelocityResponse](#dcs-weapon-v0-GetVelocityResponse)
    - [InAirRequest](#dcs-weapon-v0-InAirRequest)
    - [InAirResponse](#dcs-weapon-v0-InAirResponse)
    - [IsExistRequest](#dcs-weapon-v0-IsExistRequest)
    - [IsExistResponse](#dcs-weapon-v0-IsExistResponse)
  
    - [WeaponService](#dcs-weapon-v0-WeaponService)
  
- [dcs/world/v0/world.proto](#dcs_world_v0_world-proto)
    - [AirbaseParking](#dcs-world-v0-AirbaseParking)
    - [AirbaseRunway](#dcs-world-v0-AirbaseRunway)
    - [BoxVolume](#dcs-world-v0-BoxVolume)
    - [GetAirbaseIDRequest](#dcs-world-v0-GetAirbaseIDRequest)
    - [GetAirbaseIDResponse](#dcs-world-v0-GetAirbaseIDResponse)
    - [GetAirbaseParkingRequest](#dcs-world-v0-GetAirbaseParkingRequest)
    - [GetAirbaseParkingResponse](#dcs-world-v0-GetAirbaseParkingResponse)
    - [GetAirbaseRadioSilentModeRequest](#dcs-world-v0-GetAirbaseRadioSilentModeRequest)
    - [GetAirbaseRadioSilentModeResponse](#dcs-world-v0-GetAirbaseRadioSilentModeResponse)
    - [GetAirbaseRunwaysRequest](#dcs-world-v0-GetAirbaseRunwaysRequest)
    - [GetAirbaseRunwaysResponse](#dcs-world-v0-GetAirbaseRunwaysResponse)
    - [GetAirbasesRequest](#dcs-world-v0-GetAirbasesRequest)
    - [GetAirbasesResponse](#dcs-world-v0-GetAirbasesResponse)
    - [GetMarkPanelsRequest](#dcs-world-v0-GetMarkPanelsRequest)
    - [GetMarkPanelsResponse](#dcs-world-v0-GetMarkPanelsResponse)
    - [GetTheatreRequest](#dcs-world-v0-GetTheatreRequest)
    - [GetTheatreResponse](#dcs-world-v0-GetTheatreResponse)
    - [PyramidVolume](#dcs-world-v0-PyramidVolume)
    - [SearchObjectsRequest](#dcs-world-v0-SearchObjectsRequest)
    - [SearchObjectsResponse](#dcs-world-v0-SearchObjectsResponse)
    - [SearchVolume](#dcs-world-v0-SearchVolume)
    - [SegmentVolume](#dcs-world-v0-SegmentVolume)
    - [SetAirbaseCoalitionRequest](#dcs-world-v0-SetAirbaseCoalitionRequest)
    - [SetAirbaseCoalitionResponse](#dcs-world-v0-SetAirbaseCoalitionResponse)
    - [SetAirbaseRadioSilentModeRequest](#dcs-world-v0-SetAirbaseRadioSilentModeRequest)
    - [SetAirbaseRadioSilentModeResponse](#dcs-world-v0-SetAirbaseRadioSilentModeResponse)
    - [SphereVolume](#dcs-world-v0-SphereVolume)
  
    - [WorldService](#dcs-world-v0-WorldService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="dcs_atmosphere_v0_atmosphere-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/atmosphere/v0/atmosphere.proto



<a name="dcs-atmosphere-v0-GetTemperatureAndPressureRequest"></a>

### GetTemperatureAndPressureRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | The position on the map we want the wind information for. Requires lat/lon/alt fields to be populated, there are no default values |






<a name="dcs-atmosphere-v0-GetTemperatureAndPressureResponse"></a>

### GetTemperatureAndPressureResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| temperature | [float](#float) |  | The temperature in Kelvin |
| pressure | [float](#float) |  | The pressure in Pascals |






<a name="dcs-atmosphere-v0-GetWindRequest"></a>

### GetWindRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | The position on the map we want the wind information for. Requires lat/lon/alt fields to be populated, there are no default values |






<a name="dcs-atmosphere-v0-GetWindResponse"></a>

### GetWindResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| heading | [float](#float) |  | The heading the wind is coming from. |
| strength | [float](#float) |  | The strength of the wind in meters per second |






<a name="dcs-atmosphere-v0-GetWindWithTurbulenceRequest"></a>

### GetWindWithTurbulenceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | The position on the map we want the wind information for. Requires lat/lon/alt fields to be populated, there are no default values |






<a name="dcs-atmosphere-v0-GetWindWithTurbulenceResponse"></a>

### GetWindWithTurbulenceResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| heading | [float](#float) |  | The heading the wind is coming from. |
| strength | [float](#float) |  | The strength of the wind in meters per second. |





 

 

 


<a name="dcs-atmosphere-v0-AtmosphereService"></a>

### AtmosphereService
https://wiki.hoggitworld.com/view/DCS_singleton_atmosphere

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetWind | [GetWindRequest](#dcs-atmosphere-v0-GetWindRequest) | [GetWindResponse](#dcs-atmosphere-v0-GetWindResponse) | https://wiki.hoggitworld.com/view/DCS_func_getWind |
| GetWindWithTurbulence | [GetWindWithTurbulenceRequest](#dcs-atmosphere-v0-GetWindWithTurbulenceRequest) | [GetWindWithTurbulenceResponse](#dcs-atmosphere-v0-GetWindWithTurbulenceResponse) | https://wiki.hoggitworld.com/view/DCS_func_getWindWithTurbulence |
| GetTemperatureAndPressure | [GetTemperatureAndPressureRequest](#dcs-atmosphere-v0-GetTemperatureAndPressureRequest) | [GetTemperatureAndPressureResponse](#dcs-atmosphere-v0-GetTemperatureAndPressureResponse) | https://wiki.hoggitworld.com/view/DCS_func_getWindWithTurbulence |

 



<a name="dcs_coalition_v0_coalition-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/coalition/v0/coalition.proto



<a name="dcs-coalition-v0-AddGroupRequest"></a>

### AddGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| country | [dcs.common.v0.Country](#dcs-common-v0-Country) |  | The coalition is determined by the provided Country and the coalition setup of the mission |
| group_category | [dcs.common.v0.GroupCategory](#dcs-common-v0-GroupCategory) |  |  |
| ground_template | [AddGroupRequest.GroundGroupTemplate](#dcs-coalition-v0-AddGroupRequest-GroundGroupTemplate) |  |  |
| ship_template | [AddGroupRequest.ShipGroupTemplate](#dcs-coalition-v0-AddGroupRequest-ShipGroupTemplate) |  |  |
| helicopter_template | [AddGroupRequest.HelicopterGroupTemplate](#dcs-coalition-v0-AddGroupRequest-HelicopterGroupTemplate) |  |  |
| plane_template | [AddGroupRequest.PlaneGroupTemplate](#dcs-coalition-v0-AddGroupRequest-PlaneGroupTemplate) |  |  |






<a name="dcs-coalition-v0-AddGroupRequest-GroundGroupTemplate"></a>

### AddGroupRequest.GroundGroupTemplate



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_id | [uint32](#uint32) | optional |  |
| hidden | [bool](#bool) |  |  |
| late_activation | [bool](#bool) |  |  |
| name | [string](#string) |  |  |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| waypoints | [AddGroupRequest.Point](#dcs-coalition-v0-AddGroupRequest-Point) | repeated |  |
| start_time | [uint32](#uint32) |  |  |
| task | [string](#string) |  |  |
| task_selected | [bool](#bool) |  |  |
| tasks | [AddGroupRequest.Task](#dcs-coalition-v0-AddGroupRequest-Task) | repeated |  |
| uncontrollable | [bool](#bool) |  |  |
| units | [AddGroupRequest.GroundUnitTemplate](#dcs-coalition-v0-AddGroupRequest-GroundUnitTemplate) | repeated |  |
| visible | [bool](#bool) |  |  |






<a name="dcs-coalition-v0-AddGroupRequest-GroundUnitTemplate"></a>

### AddGroupRequest.GroundUnitTemplate



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| type | [string](#string) |  |  |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| unit_id | [uint32](#uint32) | optional |  |
| heading | [uint32](#uint32) | optional |  |
| skill | [AddGroupRequest.Skill](#dcs-coalition-v0-AddGroupRequest-Skill) |  |  |






<a name="dcs-coalition-v0-AddGroupRequest-HelicopterGroupTemplate"></a>

### AddGroupRequest.HelicopterGroupTemplate







<a name="dcs-coalition-v0-AddGroupRequest-HelicopterUnitTemplate"></a>

### AddGroupRequest.HelicopterUnitTemplate







<a name="dcs-coalition-v0-AddGroupRequest-PlaneGroupTemplate"></a>

### AddGroupRequest.PlaneGroupTemplate







<a name="dcs-coalition-v0-AddGroupRequest-PlaneUnitTemplate"></a>

### AddGroupRequest.PlaneUnitTemplate







<a name="dcs-coalition-v0-AddGroupRequest-Point"></a>

### AddGroupRequest.Point



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| altitude_type | [AddGroupRequest.Point.AltitudeType](#dcs-coalition-v0-AddGroupRequest-Point-AltitudeType) |  |  |
| type | [AddGroupRequest.Point.PointType](#dcs-coalition-v0-AddGroupRequest-Point-PointType) |  |  |
| action | [string](#string) |  |  |
| form | [string](#string) |  |  |
| speed | [double](#double) |  |  |






<a name="dcs-coalition-v0-AddGroupRequest-ShipGroupTemplate"></a>

### AddGroupRequest.ShipGroupTemplate







<a name="dcs-coalition-v0-AddGroupRequest-ShipUnitTemplate"></a>

### AddGroupRequest.ShipUnitTemplate







<a name="dcs-coalition-v0-AddGroupRequest-Task"></a>

### AddGroupRequest.Task







<a name="dcs-coalition-v0-AddGroupResponse"></a>

### AddGroupResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group | [dcs.common.v0.Group](#dcs-common-v0-Group) |  |  |






<a name="dcs-coalition-v0-AddLinkedStaticRequest"></a>

### AddLinkedStaticRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | the name of the static; must be unique or would destroy previous object |
| country | [dcs.common.v0.Country](#dcs-common-v0-Country) |  | country the unit belongs to |
| type | [string](#string) |  | type of the static object (e.g. &#34;Farm A&#34;, &#34;AS32-31A&#34;) |
| livery | [string](#string) |  | string name of the livery for the aircraft |
| dead | [bool](#bool) |  | boolean for whether or not the object will appear as a wreck |
| rate | [uint32](#uint32) | optional | number value for the &#34;score&#34; of the object when it is killed |
| unit | [string](#string) |  | the name of the unit to offset from |
| angle | [double](#double) |  | the angle to relative to the linked unit, in a clockwise direction. negative values are anti-clockwise |
| x | [double](#double) |  | x offset from linked unit center (positive is forward; negative is aft) |
| y | [double](#double) |  | y offset from linked unit center (positive is starboard-side; negative is port-side) |






<a name="dcs-coalition-v0-AddLinkedStaticResponse"></a>

### AddLinkedStaticResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-coalition-v0-AddStaticObjectRequest"></a>

### AddStaticObjectRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | the name of the static; must be unique or would destroy previous object |
| country | [dcs.common.v0.Country](#dcs-common-v0-Country) |  | country the unit belongs to |
| type | [string](#string) |  | type of the static object (e.g. &#34;Farm A&#34;, &#34;AS32-31A&#34;) |
| livery | [string](#string) |  | string name of the livery for the aircraft |
| dead | [bool](#bool) |  | boolean for whether or not the object will appear as a wreck |
| rate | [uint32](#uint32) | optional | number value for the &#34;score&#34; of the object when it is killed |
| heading | [double](#double) |  |  |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| cargo_mass | [uint32](#uint32) |  | cargo mass in kilograms |






<a name="dcs-coalition-v0-AddStaticObjectResponse"></a>

### AddStaticObjectResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-coalition-v0-GetBullseyeRequest"></a>

### GetBullseyeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | A specific coalition must be used for this API call. Do not use `COALITION_ALL` |






<a name="dcs-coalition-v0-GetBullseyeResponse"></a>

### GetBullseyeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |






<a name="dcs-coalition-v0-GetGroupsRequest"></a>

### GetGroupsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| category | [dcs.common.v0.GroupCategory](#dcs-common-v0-GroupCategory) |  |  |






<a name="dcs-coalition-v0-GetGroupsResponse"></a>

### GetGroupsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| groups | [dcs.common.v0.Group](#dcs-common-v0-Group) | repeated |  |






<a name="dcs-coalition-v0-GetPlayerUnitsRequest"></a>

### GetPlayerUnitsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |






<a name="dcs-coalition-v0-GetPlayerUnitsResponse"></a>

### GetPlayerUnitsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| units | [dcs.common.v0.Unit](#dcs-common-v0-Unit) | repeated |  |






<a name="dcs-coalition-v0-GetStaticObjectsRequest"></a>

### GetStaticObjectsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | the coalition which the statics belong to |






<a name="dcs-coalition-v0-GetStaticObjectsResponse"></a>

### GetStaticObjectsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| statics | [dcs.common.v0.Static](#dcs-common-v0-Static) | repeated | the list of statics |





 


<a name="dcs-coalition-v0-AddGroupRequest-Point-AltitudeType"></a>

### AddGroupRequest.Point.AltitudeType


| Name | Number | Description |
| ---- | ------ | ----------- |
| ALTITUDE_TYPE_UNSPECIFIED | 0 |  |
| ALTITUDE_TYPE_BAROMETRIC | 1 |  |
| ALTITUDE_TYPE_RADIO | 2 |  |



<a name="dcs-coalition-v0-AddGroupRequest-Point-PointType"></a>

### AddGroupRequest.Point.PointType


| Name | Number | Description |
| ---- | ------ | ----------- |
| POINT_TYPE_RANDOM | 0 | protolint:disable:next ENUM_FIELD_NAMES_ZERO_VALUE_END_WITH |
| POINT_TYPE_TAKEOFF | 1 |  |
| POINT_TYPE_TAKEOFF_PARKING | 2 |  |
| POINT_TYPE_TURNING_POINT | 3 |  |
| POINT_TYPE_TAKEOFF_PARKING_HOT | 4 |  |
| POINT_TYPE_LAND | 5 |  |



<a name="dcs-coalition-v0-AddGroupRequest-Skill"></a>

### AddGroupRequest.Skill


| Name | Number | Description |
| ---- | ------ | ----------- |
| SKILL_RANDOM | 0 | protolint:disable:next ENUM_FIELD_NAMES_ZERO_VALUE_END_WITH |
| SKILL_AVERAGE | 1 |  |
| SKILL_GOOD | 2 |  |
| SKILL_HIGH | 3 |  |
| SKILL_EXCELLENT | 4 |  |
| SKILL_PLAYER | 5 |  |


 

 


<a name="dcs-coalition-v0-CoalitionService"></a>

### CoalitionService
https://wiki.hoggitworld.com/view/DCS_singleton_coalition

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| AddGroup | [AddGroupRequest](#dcs-coalition-v0-AddGroupRequest) | [AddGroupResponse](#dcs-coalition-v0-AddGroupResponse) | https://wiki.hoggitworld.com/view/DCS_func_addGroup |
| GetStaticObjects | [GetStaticObjectsRequest](#dcs-coalition-v0-GetStaticObjectsRequest) | [GetStaticObjectsResponse](#dcs-coalition-v0-GetStaticObjectsResponse) | https://wiki.hoggitworld.com/view/DCS_func_getStaticObjects |
| AddStaticObject | [AddStaticObjectRequest](#dcs-coalition-v0-AddStaticObjectRequest) | [AddStaticObjectResponse](#dcs-coalition-v0-AddStaticObjectResponse) | Focussed on statics (linked statics - see `AddLinkedStatic`) https://wiki.hoggitworld.com/view/DCS_func_addStaticObject |
| AddLinkedStatic | [AddLinkedStaticRequest](#dcs-coalition-v0-AddLinkedStaticRequest) | [AddLinkedStaticResponse](#dcs-coalition-v0-AddLinkedStaticResponse) | Focussed on properties relevant to linked static objects https://wiki.hoggitworld.com/view/DCS_func_addStaticObject |
| GetGroups | [GetGroupsRequest](#dcs-coalition-v0-GetGroupsRequest) | [GetGroupsResponse](#dcs-coalition-v0-GetGroupsResponse) | https://wiki.hoggitworld.com/view/DCS_func_getGroups |
| GetBullseye | [GetBullseyeRequest](#dcs-coalition-v0-GetBullseyeRequest) | [GetBullseyeResponse](#dcs-coalition-v0-GetBullseyeResponse) | Get the Bullseye for the coalition

This position is set at mission start and does not change for the duration of the mission.

See https://wiki.hoggitworld.com/view/DCS_func_getMainRefPoint for more details |
| GetPlayerUnits | [GetPlayerUnitsRequest](#dcs-coalition-v0-GetPlayerUnitsRequest) | [GetPlayerUnitsResponse](#dcs-coalition-v0-GetPlayerUnitsResponse) | https://wiki.hoggitworld.com/view/DCS_func_getPlayers |

 



<a name="dcs_common_v0_common-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/common/v0/common.proto



<a name="dcs-common-v0-Airbase"></a>

### Airbase
An instance of a DCS Airfield


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit | [Unit](#dcs-common-v0-Unit) | optional | Information about the unit, if the airbase is one (e.g. in case of a carrier). |
| name | [string](#string) |  | TODO: Fill this in |
| callsign | [string](#string) |  | TODO: Fill this in |
| coalition | [Coalition](#dcs-common-v0-Coalition) |  | The coalition the unit belongs to. This can change mid-mission if an airfield is captured |
| position | [Position](#dcs-common-v0-Position) |  | The position of the center point of the airfield. |
| category | [AirbaseCategory](#dcs-common-v0-AirbaseCategory) |  | What category the airfield belongs to. |
| display_name | [string](#string) |  | TODO: Fill this in |






<a name="dcs-common-v0-Cargo"></a>

### Cargo
An instance of a DCS Cargo object






<a name="dcs-common-v0-Contact"></a>

### Contact
An instance of a contact in a DCS AI controller&#39;s detection table

This is a target that the AI controller has detected and is actively tracking


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | The DCS generated ID |
| visible | [bool](#bool) |  | Can the sensor see the contact |
| distance | [bool](#bool) |  | Does the controller know the distance to the contact? |
| object | [Unknown](#dcs-common-v0-Unknown) |  |  |
| unit | [Unit](#dcs-common-v0-Unit) |  |  |
| weapon | [Weapon](#dcs-common-v0-Weapon) |  |  |






<a name="dcs-common-v0-Group"></a>

### Group
An instance of a DCS group


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | The DCS generated ID |
| name | [string](#string) |  | The name of the group as assigned in the mission editor |
| coalition | [Coalition](#dcs-common-v0-Coalition) |  | The coalition of the group |
| category | [GroupCategory](#dcs-common-v0-GroupCategory) |  | The group category. |






<a name="dcs-common-v0-Initiator"></a>

### Initiator
The initiator of an event

The initiator of an event. For things like shooting events it is usually a
vehicle but it can be almost anything depending on the event


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unknown | [Unknown](#dcs-common-v0-Unknown) |  |  |
| unit | [Unit](#dcs-common-v0-Unit) |  |  |
| weapon | [Weapon](#dcs-common-v0-Weapon) |  |  |
| static | [Static](#dcs-common-v0-Static) |  |  |
| scenery | [Scenery](#dcs-common-v0-Scenery) |  |  |
| airbase | [Airbase](#dcs-common-v0-Airbase) |  |  |
| cargo | [Cargo](#dcs-common-v0-Cargo) |  |  |






<a name="dcs-common-v0-InputPosition"></a>

### InputPosition
Position used in requests to DCS-gRPC.

Latitude and Longitude are in Decimal Degrees format (e.g. 41.33 / 37.21).
Negative values are used for West of the meridian and south of the equator.

Altitude is given in meters above Mean Sea Level (MSL) and can be a decimal
value.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lat | [double](#double) |  | Latitude in Decimal Degrees format |
| lon | [double](#double) |  | Longitude in Decimal Degrees format |
| alt | [double](#double) |  | Altitude in Meters above Mean Sea Level (MSL) |






<a name="dcs-common-v0-MarkPanel"></a>

### MarkPanel
A MarkPanel

A MarkPanel visible on the F10 map. These can be used for reference by
players but can also be used by things like Jester for setting waypoints


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | The id of the mark panel. |
| time | [double](#double) |  | The time in seconds relative to the mission start the mark got created. |
| initiator | [Unit](#dcs-common-v0-Unit) | optional | The unit of the player that created the mark. Not set if the player isn&#39;t controlling any unit anymore (disconnected, spectator, game master, ...). |
| coalition | [Coalition](#dcs-common-v0-Coalition) | optional | If set, the mark is only visible for the specified coalition. |
| group_id | [uint32](#uint32) | optional | The ID of the group the player was in when creating the mark panel. This will still be set even if the player isn&#39;t controlling the unit in that group anymore. |
| text | [string](#string) | optional | The text content of the mark. |
| position | [Position](#dcs-common-v0-Position) |  | The position of the mark. |






<a name="dcs-common-v0-Orientation"></a>

### Orientation
The orientation of an object in 3D space.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| heading | [double](#double) |  | The heading the nose of the object points to on a flat world. |
| yaw | [double](#double) |  | Yaw in degrees - clockwise relative to the true north (this is similar to the heading, just corrected by the projection error when going from a flat to a spherical world). |
| pitch | [double](#double) |  | Pitch in degrees - positive when taking-off. |
| roll | [double](#double) |  | Roll in degrees - positive when rolling the aircraft to the right. |
| forward | [Vector](#dcs-common-v0-Vector) |  | The normalized direction the object is pointing to. |
| right | [Vector](#dcs-common-v0-Vector) |  | The normalized direction the three line (right wing) is pointing to. |
| up | [Vector](#dcs-common-v0-Vector) |  | The normalized up vector (orthogonal to forward and right). |






<a name="dcs-common-v0-Position"></a>

### Position
Position of an object in DCS

Latitude and Longitude are in Decimal Degrees format (e.g. 41.33 / 37.21).
Negative values are used for West of the meridian and south of the equator

Altitude is given in meters above Mean Sea Level (MSL) and can be a decimal
value.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lat | [double](#double) |  | Latitude in Decimal Degrees format |
| lon | [double](#double) |  | Longitude in Decimal Degrees format |
| alt | [double](#double) |  | Altitude in Meters above Mean Sea Level (MSL) |
| u | [double](#double) |  | Distance between DCS&#39; map origin to object in meters on west-east axis. |
| v | [double](#double) |  | Distance between DCS&#39; map origin to object in meters on north-south axis. |






<a name="dcs-common-v0-Scenery"></a>

### Scenery
An instance of a DCS scenery object


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | The id of the scenery |
| type | [string](#string) |  | The DCS type-name of the scenery |
| position | [Position](#dcs-common-v0-Position) |  | The position of the scenery |






<a name="dcs-common-v0-Static"></a>

### Static
An instance of a DCS static object

These objects are often buildings but can also be vehicles that have no AI or
other game behaviour aside from being destroyable


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | The DCS generated ID |
| type | [string](#string) | optional | The DCS type-name of the static |
| name | [string](#string) |  | The name of the static |
| coalition | [Coalition](#dcs-common-v0-Coalition) |  | The coalition the static belongs to |
| position | [Position](#dcs-common-v0-Position) |  | The position of the static |






<a name="dcs-common-v0-Target"></a>

### Target
The target of an event

The target of an event. For things like shooting events it is usually a
vehicle but it can be almost anything depending on the event


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unknown | [Unknown](#dcs-common-v0-Unknown) |  |  |
| unit | [Unit](#dcs-common-v0-Unit) |  |  |
| weapon | [Weapon](#dcs-common-v0-Weapon) |  |  |
| static | [Static](#dcs-common-v0-Static) |  |  |
| scenery | [Scenery](#dcs-common-v0-Scenery) |  |  |
| airbase | [Airbase](#dcs-common-v0-Airbase) |  |  |
| cargo | [Cargo](#dcs-common-v0-Cargo) |  |  |






<a name="dcs-common-v0-Unit"></a>

### Unit
An instance of a DCS Unit

A unit is an &#34;active&#34; unit in a DCS mission. This means it has an attached AI
that moves and shoots. Units include aircraft, ground units, ships, weapons
etc.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | The DCS generated ID |
| name | [string](#string) |  | The name of the unit as assigned in the mission editor |
| callsign | [string](#string) |  | The DCS assigned callsign if one exists. e.g. &#34;Enfield 11&#34; |
| coalition | [Coalition](#dcs-common-v0-Coalition) |  | The coalition the unit belongs to |
| type | [string](#string) | optional | The DCS type-name of the unit. e.g &#34;MiG-29A&#34;, &#34;ZSU_57_2&#34; or &#34;Hawk ln&#34; |
| position | [Position](#dcs-common-v0-Position) |  | The position of the unit |
| orientation | [Orientation](#dcs-common-v0-Orientation) |  | The orientation of the unit in both 2D and 3D space |
| velocity | [Velocity](#dcs-common-v0-Velocity) |  | The velocity of the unit in both 2D and 3D space |
| player_name | [string](#string) | optional | The name of the player if one is in control of the unit |
| group | [Group](#dcs-common-v0-Group) |  | The group that the unit belongs to |
| number_in_group | [uint32](#uint32) |  | The number of this unit in the group. Does not change as units are destroyed |






<a name="dcs-common-v0-Unknown"></a>

### Unknown
This type is returned if an object category cannot be determined

The base object includes the `getName()` function so even for an unknown type
we _should_ be able to get the name


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-common-v0-Vector"></a>

### Vector
A vector in a right-handed coordinate system where &#43;x is north, -x south, &#43;z
is east, -z west, &#43;y up and -y down.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| x | [double](#double) |  |  |
| y | [double](#double) |  |  |
| z | [double](#double) |  |  |






<a name="dcs-common-v0-Velocity"></a>

### Velocity
The orientation of an object in 3D space.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| heading | [double](#double) |  | The heading the object is moving to (use `orientation.heading` to get the heading the nose is pointing to). |
| speed | [double](#double) |  | The horizontal speed of the unit. If it is doing mach one straight up then the speed will be 0 |
| velocity | [Vector](#dcs-common-v0-Vector) |  | The direction the object is traveling to, and speed (magnitude of the vector) the object is traveling with. |






<a name="dcs-common-v0-Weapon"></a>

### Weapon
An instance of a DCS weapon

These weapons include everything from autocannon HE shells up to massive
ship-killer missiles


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | The DCS generated ID |
| type | [string](#string) | optional | The DCS type-name of the weapon. e.g &#34;Matra_S530D&#34;, &#34;HAWK_RAKETA&#34; or &#34;weapons.shells.53-UOR-281U&#34; |
| position | [Position](#dcs-common-v0-Position) |  | The position of the Weapon |
| orientation | [Orientation](#dcs-common-v0-Orientation) |  | The orientation of the unit in both 2D and 3D space |
| velocity | [Velocity](#dcs-common-v0-Velocity) |  | The velocity of the unit in both 2D and 3D space |





 


<a name="dcs-common-v0-AirbaseCategory"></a>

### AirbaseCategory
The category the object belongs to

Some of these are less than obvious. For example an oilrig counts as a
HELIPAD airfield.

| Name | Number | Description |
| ---- | ------ | ----------- |
| AIRBASE_CATEGORY_UNSPECIFIED | 0 |  |
| AIRBASE_CATEGORY_AIRDROME | 1 |  |
| AIRBASE_CATEGORY_HELIPAD | 2 |  |
| AIRBASE_CATEGORY_SHIP | 3 |  |



<a name="dcs-common-v0-Coalition"></a>

### Coalition
Coalitions in DCS

The coalitions supported by DCS. The NEUTRAL coalition is a relatively new
one and may not be as supported as the belligerant ones.

| Name | Number | Description |
| ---- | ------ | ----------- |
| COALITION_ALL | 0 | protolint:disable:next ENUM_FIELD_NAMES_ZERO_VALUE_END_WITH |
| COALITION_NEUTRAL | 1 |  |
| COALITION_RED | 2 |  |
| COALITION_BLUE | 3 |  |



<a name="dcs-common-v0-Country"></a>

### Country
Countries in DCS

Every country belongs to a coalition and this association is set per mission.
The values of these enums are correct such that they will work with DCS
however the text names have been Made to follow gRPC conventions to to aid
in language bindings and acronyms have been replaced with their full english
names to aid in recognition. In some cases this can be a big change
(e.g. USSR -&gt; Soviet Union).

We have also added a dummy value for the missing enum value 14 to prevent
possible issues in the various language bindings

See https://wiki.hoggitworld.com/view/DCS_enum_country for more information

| Name | Number | Description |
| ---- | ------ | ----------- |
| COUNTRY_UNSPECIFIED | 0 |  |
| COUNTRY_RUSSIA | 1 |  |
| COUNTRY_UKRAINE | 2 |  |
| COUNTRY_UNITED_STATES_OF_AMERICA | 3 |  |
| COUNTRY_TURKEY | 4 |  |
| COUNTRY_UNITED_KINGDOM | 5 |  |
| COUNTRY_FRANCE | 6 |  |
| COUNTRY_GERMANY | 7 |  |
| COUNTRY_AGGRESSORS | 8 |  |
| COUNTRY_CANADA | 9 |  |
| COUNTRY_SPAIN | 10 |  |
| COUNTRY_THE_NETHERLANDS | 11 |  |
| COUNTRY_BELGIUM | 12 |  |
| COUNTRY_NORWAY | 13 |  |
| COUNTRY_DENMARK | 14 |  |
| COUNTRY_UNUSED | 15 |  |
| COUNTRY_ISRAEL | 16 |  |
| COUNTRY_GEORGIA | 17 |  |
| COUNTRY_INSURGENTS | 18 |  |
| COUNTRY_ABKHAZIA | 19 |  |
| COUNTRY_SOUTH_OSETIA | 20 |  |
| COUNTRY_ITALY | 21 |  |
| COUNTRY_AUSTRALIA | 22 |  |
| COUNTRY_SWITZERLAND | 23 |  |
| COUNTRY_AUSTRIA | 24 |  |
| COUNTRY_BELARUS | 25 |  |
| COUNTRY_BULGARIA | 26 |  |
| COUNTRY_CZECH_REPUBLIC | 27 |  |
| COUNTRY_CHINA | 28 |  |
| COUNTRY_CROATIA | 29 |  |
| COUNTRY_EGYPT | 30 |  |
| COUNTRY_FINLAND | 31 |  |
| COUNTRY_GREECE | 32 |  |
| COUNTRY_HUNGARY | 33 |  |
| COUNTRY_INDIA | 34 |  |
| COUNTRY_IRAN | 35 |  |
| COUNTRY_IRAQ | 36 |  |
| COUNTRY_JAPAN | 37 |  |
| COUNTRY_KAZAKHSTAN | 38 |  |
| COUNTRY_NORTH_KOREA | 39 |  |
| COUNTRY_PAKISTAN | 40 |  |
| COUNTRY_POLAND | 41 |  |
| COUNTRY_ROMANIA | 42 |  |
| COUNTRY_SAUDI_ARABIA | 43 |  |
| COUNTRY_SERBIA | 44 |  |
| COUNTRY_SLOVAKIA | 45 |  |
| COUNTRY_SOUTH_KOREA | 46 |  |
| COUNTRY_SWEDEN | 47 |  |
| COUNTRY_SYRIA | 48 |  |
| COUNTRY_YEMEN | 49 |  |
| COUNTRY_VIETNAM | 50 |  |
| COUNTRY_VENEZUELA | 51 |  |
| COUNTRY_TUNISIA | 52 |  |
| COUNTRY_THAILAND | 53 |  |
| COUNTRY_SUDAN | 54 |  |
| COUNTRY_PHILIPPINES | 55 |  |
| COUNTRY_MOROCCO | 56 |  |
| COUNTRY_MEXICO | 57 |  |
| COUNTRY_MALAYSIA | 58 |  |
| COUNTRY_LIBYA | 59 |  |
| COUNTRY_JORDAN | 60 |  |
| COUNTRY_INDONESIA | 61 |  |
| COUNTRY_HONDURAS | 62 |  |
| COUNTRY_ETHIOPIA | 63 |  |
| COUNTRY_CHILE | 64 |  |
| COUNTRY_BRAZIL | 65 |  |
| COUNTRY_BAHRAIN | 66 |  |
| COUNTRY_THIRDREICH | 67 |  |
| COUNTRY_YUGOSLAVIA | 68 |  |
| COUNTRY_SOVIET_UNION | 69 |  |
| COUNTRY_ITALIAN_SOCIAL_REPUBLIC | 70 |  |
| COUNTRY_ALGERIA | 71 |  |
| COUNTRY_KUWAIT | 72 |  |
| COUNTRY_QATAR | 73 |  |
| COUNTRY_OMAN | 74 |  |
| COUNTRY_UNITED_ARAB_EMIRATES | 75 |  |
| COUNTRY_SOUTH_AFRICA | 76 |  |
| COUNTRY_CUBA | 77 |  |
| COUNTRY_PORTUGAL | 78 |  |
| COUNTRY_GERMAN_DEMOCRATIC_REPUBLIC | 79 |  |
| COUNTRY_LEBANON | 80 |  |
| COUNTRY_COMBINED_JOINT_TASK_FORCE_BLUE | 81 |  |
| COUNTRY_COMBINED_JOINT_TASK_FORCE_RED | 82 |  |
| COUNTRY_UNITED_NATIONS_PEACEKEEPERS | 83 |  |
| COUNTRY_ARGENTINA | 84 |  |
| COUNTRY_CYPRUS | 85 |  |
| COUNTRY_SLOVENIA | 86 |  |



<a name="dcs-common-v0-GroupCategory"></a>

### GroupCategory
Group category enumerator.

| Name | Number | Description |
| ---- | ------ | ----------- |
| GROUP_CATEGORY_UNSPECIFIED | 0 |  |
| GROUP_CATEGORY_AIRPLANE | 1 |  |
| GROUP_CATEGORY_HELICOPTER | 2 |  |
| GROUP_CATEGORY_GROUND | 3 |  |
| GROUP_CATEGORY_SHIP | 4 |  |
| GROUP_CATEGORY_TRAIN | 5 |  |



<a name="dcs-common-v0-ObjectCategory"></a>

### ObjectCategory
The category the object belongs to

All DCS objects are one of the following categories. Unlike many other
enums created by DCS, this one is not 0 indexed. Therefore we do not
need to do any modification of the value by incrementing it by one to
make it work with gRPC and DCS.

See https://wiki.hoggitworld.com/view/DCS_Class_Object for more information

| Name | Number | Description |
| ---- | ------ | ----------- |
| OBJECT_CATEGORY_UNSPECIFIED | 0 |  |
| OBJECT_CATEGORY_UNIT | 1 |  |
| OBJECT_CATEGORY_WEAPON | 2 |  |
| OBJECT_CATEGORY_STATIC | 3 |  |
| OBJECT_CATEGORY_SCENERY | 4 |  |
| OBJECT_CATEGORY_BASE | 5 |  |
| OBJECT_CATEGORY_CARGO | 6 |  |


 

 

 



<a name="dcs_controller_v0_controller-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/controller/v0/controller.proto



<a name="dcs-controller-v0-GetDetectedTargetsRequest"></a>

### GetDetectedTargetsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit_name | [string](#string) |  |  |
| include_object | [bool](#bool) | optional |  |
| detection_type | [GetDetectedTargetsRequest.DetectionType](#dcs-controller-v0-GetDetectedTargetsRequest-DetectionType) | optional |  |






<a name="dcs-controller-v0-GetDetectedTargetsResponse"></a>

### GetDetectedTargetsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| contacts | [dcs.common.v0.Contact](#dcs-common-v0-Contact) | repeated |  |






<a name="dcs-controller-v0-HasTaskRequest"></a>

### HasTaskRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |






<a name="dcs-controller-v0-HasTaskResponse"></a>

### HasTaskResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| has_task | [bool](#bool) |  |  |






<a name="dcs-controller-v0-IsTargetDetectedRequest"></a>

### IsTargetDetectedRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |
| target_name | [string](#string) |  |  |






<a name="dcs-controller-v0-IsTargetDetectedResponse"></a>

### IsTargetDetectedResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_detected | [bool](#bool) |  |  |






<a name="dcs-controller-v0-KnowTargetRequest"></a>

### KnowTargetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |
| target_name | [string](#string) |  |  |
| type | [bool](#bool) |  |  |
| distance | [bool](#bool) |  |  |






<a name="dcs-controller-v0-KnowTargetResponse"></a>

### KnowTargetResponse







<a name="dcs-controller-v0-PopTaskRequest"></a>

### PopTaskRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |






<a name="dcs-controller-v0-PopTaskResponse"></a>

### PopTaskResponse







<a name="dcs-controller-v0-PushTaskRequest"></a>

### PushTaskRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |
| task_json | [string](#string) |  |  |






<a name="dcs-controller-v0-PushTaskResponse"></a>

### PushTaskResponse







<a name="dcs-controller-v0-ResetTaskRequest"></a>

### ResetTaskRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |






<a name="dcs-controller-v0-ResetTaskResponse"></a>

### ResetTaskResponse







<a name="dcs-controller-v0-SetAlarmStateRequest"></a>

### SetAlarmStateRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |
| alarm_state | [SetAlarmStateRequest.AlarmState](#dcs-controller-v0-SetAlarmStateRequest-AlarmState) |  |  |






<a name="dcs-controller-v0-SetAlarmStateResponse"></a>

### SetAlarmStateResponse







<a name="dcs-controller-v0-SetCommandRequest"></a>

### SetCommandRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |
| command_json | [string](#string) |  |  |






<a name="dcs-controller-v0-SetCommandResponse"></a>

### SetCommandResponse







<a name="dcs-controller-v0-SetOnOffRequest"></a>

### SetOnOffRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |
| status | [bool](#bool) |  |  |






<a name="dcs-controller-v0-SetOnOffResponse"></a>

### SetOnOffResponse







<a name="dcs-controller-v0-SetOptionRequest"></a>

### SetOptionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |
| option_id | [int32](#int32) |  |  |
| bool_value | [bool](#bool) |  |  |
| int_value | [int32](#int32) |  |  |
| string_value | [string](#string) |  |  |
| double_value | [double](#double) |  |  |






<a name="dcs-controller-v0-SetOptionResponse"></a>

### SetOptionResponse







<a name="dcs-controller-v0-SetTaskRequest"></a>

### SetTaskRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| unit_name | [string](#string) |  |  |
| task_json | [string](#string) |  |  |






<a name="dcs-controller-v0-SetTaskResponse"></a>

### SetTaskResponse






 


<a name="dcs-controller-v0-GetDetectedTargetsRequest-DetectionType"></a>

### GetDetectedTargetsRequest.DetectionType


| Name | Number | Description |
| ---- | ------ | ----------- |
| DETECTION_TYPE_UNSPECIFIED | 0 |  |
| DETECTION_TYPE_VISUAL | 1 |  |
| DETECTION_TYPE_OPTIC | 2 |  |
| DETECTION_TYPE_RADAR | 4 |  |
| DETECTION_TYPE_IRST | 8 |  |
| DETECTION_TYPE_RWR | 16 |  |
| DETECTION_TYPE_DLINK | 32 |  |



<a name="dcs-controller-v0-SetAlarmStateRequest-AlarmState"></a>

### SetAlarmStateRequest.AlarmState


| Name | Number | Description |
| ---- | ------ | ----------- |
| ALARM_STATE_UNSPECIFIED | 0 |  |
| ALARM_STATE_AUTO | 1 |  |
| ALARM_STATE_GREEN | 2 |  |
| ALARM_STATE_RED | 3 |  |


 

 


<a name="dcs-controller-v0-ControllerService"></a>

### ControllerService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SetAlarmState | [SetAlarmStateRequest](#dcs-controller-v0-SetAlarmStateRequest) | [SetAlarmStateResponse](#dcs-controller-v0-SetAlarmStateResponse) | https://wiki.hoggitworld.com/view/DCS_option_alarmState |
| GetDetectedTargets | [GetDetectedTargetsRequest](#dcs-controller-v0-GetDetectedTargetsRequest) | [GetDetectedTargetsResponse](#dcs-controller-v0-GetDetectedTargetsResponse) | https://wiki.hoggitworld.com/view/DCS_func_getDetectedTargets |
| HasTask | [HasTaskRequest](#dcs-controller-v0-HasTaskRequest) | [HasTaskResponse](#dcs-controller-v0-HasTaskResponse) | https://wiki.hoggitworld.com/view/DCS_func_hasTask |
| SetOnOff | [SetOnOffRequest](#dcs-controller-v0-SetOnOffRequest) | [SetOnOffResponse](#dcs-controller-v0-SetOnOffResponse) | https://wiki.hoggitworld.com/view/DCS_func_setOnOff |
| SetOption | [SetOptionRequest](#dcs-controller-v0-SetOptionRequest) | [SetOptionResponse](#dcs-controller-v0-SetOptionResponse) | https://wiki.hoggitworld.com/view/DCS_func_setOption |
| IsTargetDetected | [IsTargetDetectedRequest](#dcs-controller-v0-IsTargetDetectedRequest) | [IsTargetDetectedResponse](#dcs-controller-v0-IsTargetDetectedResponse) | https://wiki.hoggitworld.com/view/DCS_func_isTargetDetected |
| KnowTarget | [KnowTargetRequest](#dcs-controller-v0-KnowTargetRequest) | [KnowTargetResponse](#dcs-controller-v0-KnowTargetResponse) | https://wiki.hoggitworld.com/view/DCS_func_knowTarget |
| SetTask | [SetTaskRequest](#dcs-controller-v0-SetTaskRequest) | [SetTaskResponse](#dcs-controller-v0-SetTaskResponse) | https://wiki.hoggitworld.com/view/DCS_func_setTask |
| PushTask | [PushTaskRequest](#dcs-controller-v0-PushTaskRequest) | [PushTaskResponse](#dcs-controller-v0-PushTaskResponse) | https://wiki.hoggitworld.com/view/DCS_func_pushTask |
| PopTask | [PopTaskRequest](#dcs-controller-v0-PopTaskRequest) | [PopTaskResponse](#dcs-controller-v0-PopTaskResponse) | https://wiki.hoggitworld.com/view/DCS_func_popTask |
| ResetTask | [ResetTaskRequest](#dcs-controller-v0-ResetTaskRequest) | [ResetTaskResponse](#dcs-controller-v0-ResetTaskResponse) | https://wiki.hoggitworld.com/view/DCS_func_resetTask |
| SetCommand | [SetCommandRequest](#dcs-controller-v0-SetCommandRequest) | [SetCommandResponse](#dcs-controller-v0-SetCommandResponse) | https://wiki.hoggitworld.com/view/DCS_func_setCommand |

 



<a name="dcs_custom_v0_custom-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/custom/v0/custom.proto



<a name="dcs-custom-v0-AbortMissionRequest"></a>

### AbortMissionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit_name | [string](#string) |  |  |






<a name="dcs-custom-v0-AbortMissionResponse"></a>

### AbortMissionResponse







<a name="dcs-custom-v0-EvalRequest"></a>

### EvalRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lua | [string](#string) |  |  |






<a name="dcs-custom-v0-EvalResponse"></a>

### EvalResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| json | [string](#string) |  |  |






<a name="dcs-custom-v0-GetMagneticDeclinationRequest"></a>

### GetMagneticDeclinationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lat | [double](#double) |  | Latitude in Decimal Degrees format |
| lon | [double](#double) |  | Longitude in Decimal Degrees format |
| alt | [double](#double) |  | Altitude in Meters above Mean Sea Level (MSL) |






<a name="dcs-custom-v0-GetMagneticDeclinationResponse"></a>

### GetMagneticDeclinationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| declination | [double](#double) |  | Magnetic declination in degrees. A negative value is an westerly / declination, while a positive value is a easterly declination. `True / North` &#43; `declination` = `Magnetic North` |






<a name="dcs-custom-v0-GetMissionStatusRequest"></a>

### GetMissionStatusRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit_name | [string](#string) |  |  |






<a name="dcs-custom-v0-GetMissionStatusResponse"></a>

### GetMissionStatusResponse







<a name="dcs-custom-v0-JoinMissionRequest"></a>

### JoinMissionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit_name | [string](#string) |  |  |
| mission_code | [int32](#int32) |  |  |






<a name="dcs-custom-v0-JoinMissionResponse"></a>

### JoinMissionResponse







<a name="dcs-custom-v0-RequestMissionAssignmentRequest"></a>

### RequestMissionAssignmentRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit_name | [string](#string) |  |  |
| mission_type | [string](#string) |  |  |






<a name="dcs-custom-v0-RequestMissionAssignmentResponse"></a>

### RequestMissionAssignmentResponse






 

 

 


<a name="dcs-custom-v0-CustomService"></a>

### CustomService
The Custom service is for APIs that do not map to the &#34;standard library&#34; of
DCS APIs provided by Eagle Dynamics.

Expect to find APIs here that may be useful for mission frameworks etc.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| RequestMissionAssignment | [RequestMissionAssignmentRequest](#dcs-custom-v0-RequestMissionAssignmentRequest) | [RequestMissionAssignmentResponse](#dcs-custom-v0-RequestMissionAssignmentResponse) | DCT Function |
| JoinMission | [JoinMissionRequest](#dcs-custom-v0-JoinMissionRequest) | [JoinMissionResponse](#dcs-custom-v0-JoinMissionResponse) | DCT Function |
| AbortMission | [AbortMissionRequest](#dcs-custom-v0-AbortMissionRequest) | [AbortMissionResponse](#dcs-custom-v0-AbortMissionResponse) | DCT Function |
| GetMissionStatus | [GetMissionStatusRequest](#dcs-custom-v0-GetMissionStatusRequest) | [GetMissionStatusResponse](#dcs-custom-v0-GetMissionStatusResponse) | DCT Function |
| Eval | [EvalRequest](#dcs-custom-v0-EvalRequest) | [EvalResponse](#dcs-custom-v0-EvalResponse) | Evaluate some Lua inside of the mission and return the result as a JSON string. Disabled by default. |
| GetMagneticDeclination | [GetMagneticDeclinationRequest](#dcs-custom-v0-GetMagneticDeclinationRequest) | [GetMagneticDeclinationResponse](#dcs-custom-v0-GetMagneticDeclinationResponse) | Calculates the magnetic declination at the given position using the International Geomagnetic Reference Field (IGRF) model. The result is not always exactly the same as what DCS seem to use, but it is very close (DCS doesn&#39;t expose its declination). |

 



<a name="dcs_dcs-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/dcs.proto


 

 

 

 



<a name="dcs_group_v0_group-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/group/v0/group.proto



<a name="dcs-group-v0-ActivateRequest"></a>

### ActivateRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-group-v0-ActivateResponse"></a>

### ActivateResponse







<a name="dcs-group-v0-DestroyRequest"></a>

### DestroyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-group-v0-DestroyResponse"></a>

### DestroyResponse







<a name="dcs-group-v0-EnableEmissionRequest"></a>

### EnableEmissionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| enable | [bool](#bool) |  |  |






<a name="dcs-group-v0-EnableEmissionResponse"></a>

### EnableEmissionResponse







<a name="dcs-group-v0-ExistsRequest"></a>

### ExistsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-group-v0-ExistsResponse"></a>

### ExistsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| exists | [bool](#bool) |  |  |






<a name="dcs-group-v0-GetGroupRequest"></a>

### GetGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-group-v0-GetGroupResponse"></a>

### GetGroupResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group | [dcs.common.v0.Group](#dcs-common-v0-Group) |  |  |






<a name="dcs-group-v0-GetSizeRequest"></a>

### GetSizeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-group-v0-GetSizeResponse"></a>

### GetSizeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| size | [uint32](#uint32) |  |  |
| initial_size | [uint32](#uint32) |  |  |






<a name="dcs-group-v0-GetUnitRequest"></a>

### GetUnitRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| index | [uint32](#uint32) |  |  |






<a name="dcs-group-v0-GetUnitResponse"></a>

### GetUnitResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit | [dcs.common.v0.Unit](#dcs-common-v0-Unit) |  |  |






<a name="dcs-group-v0-GetUnitsRequest"></a>

### GetUnitsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| active | [bool](#bool) | optional | Whether the response should include only active units (`true`), only inactive units (`false`), or all units (`nil`). |






<a name="dcs-group-v0-GetUnitsResponse"></a>

### GetUnitsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| units | [dcs.common.v0.Unit](#dcs-common-v0-Unit) | repeated |  |





 

 

 


<a name="dcs-group-v0-GroupService"></a>

### GroupService
https://wiki.hoggitworld.com/view/DCS_Class_Group

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetUnits | [GetUnitsRequest](#dcs-group-v0-GetUnitsRequest) | [GetUnitsResponse](#dcs-group-v0-GetUnitsResponse) | https://wiki.hoggitworld.com/view/DCS_func_getUnits |
| Activate | [ActivateRequest](#dcs-group-v0-ActivateRequest) | [ActivateResponse](#dcs-group-v0-ActivateResponse) | https://wiki.hoggitworld.com/view/DCS_func_activate |
| Destroy | [DestroyRequest](#dcs-group-v0-DestroyRequest) | [DestroyResponse](#dcs-group-v0-DestroyResponse) | https://wiki.hoggitworld.com/view/DCS_func_destroy |
| GetSize | [GetSizeRequest](#dcs-group-v0-GetSizeRequest) | [GetSizeResponse](#dcs-group-v0-GetSizeResponse) | https://wiki.hoggitworld.com/view/DCS_func_getSize |
| Exists | [ExistsRequest](#dcs-group-v0-ExistsRequest) | [ExistsResponse](#dcs-group-v0-ExistsResponse) | https://wiki.hoggitworld.com/view/DCS_func_isExist |
| EnableEmission | [EnableEmissionRequest](#dcs-group-v0-EnableEmissionRequest) | [EnableEmissionResponse](#dcs-group-v0-EnableEmissionResponse) |  |
| GetGroup | [GetGroupRequest](#dcs-group-v0-GetGroupRequest) | [GetGroupResponse](#dcs-group-v0-GetGroupResponse) |  |
| GetUnit | [GetUnitRequest](#dcs-group-v0-GetUnitRequest) | [GetUnitResponse](#dcs-group-v0-GetUnitResponse) |  |

 



<a name="dcs_hook_v0_hook-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/hook/v0/hook.proto



<a name="dcs-hook-v0-BanDetails"></a>

### BanDetails



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ucid | [string](#string) |  | The globally unique ID of the player |
| ip_address | [string](#string) |  | The IP address the user had when they were banned |
| player_name | [string](#string) |  | The Name of the player at the time of the ban |
| reason | [string](#string) |  | The reason given for the ban |
| banned_from | [uint64](#uint64) |  | When the ban was issued in unixtime |
| banned_until | [uint64](#uint64) |  | When the ban will expire in unixtime |






<a name="dcs-hook-v0-BanPlayerRequest"></a>

### BanPlayerRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | The session ID of the player |
| period | [uint32](#uint32) |  | The period of the ban in seconds |
| reason | [string](#string) |  | The reason for the ban |






<a name="dcs-hook-v0-BanPlayerResponse"></a>

### BanPlayerResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| banned | [bool](#bool) |  | Was the player successfully banned |






<a name="dcs-hook-v0-EvalRequest"></a>

### EvalRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lua | [string](#string) |  |  |






<a name="dcs-hook-v0-EvalResponse"></a>

### EvalResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| json | [string](#string) |  |  |






<a name="dcs-hook-v0-ExitProcessRequest"></a>

### ExitProcessRequest







<a name="dcs-hook-v0-ExitProcessResponse"></a>

### ExitProcessResponse







<a name="dcs-hook-v0-GetAvailableCoalitionsRequest"></a>

### GetAvailableCoalitionsRequest







<a name="dcs-hook-v0-GetAvailableCoalitionsResponse"></a>

### GetAvailableCoalitionsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalitions_json | [string](#string) |  |  |






<a name="dcs-hook-v0-GetAvailableSlotsRequest"></a>

### GetAvailableSlotsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [string](#string) |  |  |






<a name="dcs-hook-v0-GetAvailableSlotsResponse"></a>

### GetAvailableSlotsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| slots_json | [string](#string) |  |  |






<a name="dcs-hook-v0-GetBallisticsCountRequest"></a>

### GetBallisticsCountRequest







<a name="dcs-hook-v0-GetBallisticsCountResponse"></a>

### GetBallisticsCountResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| count | [uint32](#uint32) |  |  |






<a name="dcs-hook-v0-GetBannedPlayersRequest"></a>

### GetBannedPlayersRequest







<a name="dcs-hook-v0-GetBannedPlayersResponse"></a>

### GetBannedPlayersResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bans | [BanDetails](#dcs-hook-v0-BanDetails) | repeated |  |






<a name="dcs-hook-v0-GetCurrentMissionRequest"></a>

### GetCurrentMissionRequest







<a name="dcs-hook-v0-GetCurrentMissionResponse"></a>

### GetCurrentMissionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| mission_json | [string](#string) |  |  |






<a name="dcs-hook-v0-GetMissionDescriptionRequest"></a>

### GetMissionDescriptionRequest







<a name="dcs-hook-v0-GetMissionDescriptionResponse"></a>

### GetMissionDescriptionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| description | [string](#string) |  |  |






<a name="dcs-hook-v0-GetMissionFilenameRequest"></a>

### GetMissionFilenameRequest







<a name="dcs-hook-v0-GetMissionFilenameResponse"></a>

### GetMissionFilenameResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-hook-v0-GetMissionNameRequest"></a>

### GetMissionNameRequest







<a name="dcs-hook-v0-GetMissionNameResponse"></a>

### GetMissionNameResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-hook-v0-GetMissionOptionsRequest"></a>

### GetMissionOptionsRequest







<a name="dcs-hook-v0-GetMissionOptionsResponse"></a>

### GetMissionOptionsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| options_json | [string](#string) |  |  |






<a name="dcs-hook-v0-GetMissionResultRequest"></a>

### GetMissionResultRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| side | [string](#string) |  |  |






<a name="dcs-hook-v0-GetMissionResultResponse"></a>

### GetMissionResultResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [int32](#int32) |  |  |






<a name="dcs-hook-v0-GetModelTimeRequest"></a>

### GetModelTimeRequest







<a name="dcs-hook-v0-GetModelTimeResponse"></a>

### GetModelTimeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  |  |






<a name="dcs-hook-v0-GetPausedRequest"></a>

### GetPausedRequest







<a name="dcs-hook-v0-GetPausedResponse"></a>

### GetPausedResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| paused | [bool](#bool) |  |  |






<a name="dcs-hook-v0-GetRealTimeRequest"></a>

### GetRealTimeRequest







<a name="dcs-hook-v0-GetRealTimeResponse"></a>

### GetRealTimeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  | The current time in a mission relative to the DCS start time |






<a name="dcs-hook-v0-GetUnitPropertyRequest"></a>

### GetUnitPropertyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| property | [int32](#int32) |  |  |






<a name="dcs-hook-v0-GetUnitPropertyResponse"></a>

### GetUnitPropertyResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| property_value_json | [string](#string) |  |  |






<a name="dcs-hook-v0-GetUnitTypeRequest"></a>

### GetUnitTypeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | The slot or unit ID of the unit to retrieve the type of |






<a name="dcs-hook-v0-GetUnitTypeResponse"></a>

### GetUnitTypeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [string](#string) |  | Type of unit (e.g. &#34;F-14B&#34;) |






<a name="dcs-hook-v0-IsMultiplayerRequest"></a>

### IsMultiplayerRequest







<a name="dcs-hook-v0-IsMultiplayerResponse"></a>

### IsMultiplayerResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multiplayer | [bool](#bool) |  |  |






<a name="dcs-hook-v0-IsServerRequest"></a>

### IsServerRequest







<a name="dcs-hook-v0-IsServerResponse"></a>

### IsServerResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| server | [bool](#bool) |  |  |






<a name="dcs-hook-v0-LoadMissionRequest"></a>

### LoadMissionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| file_name | [string](#string) |  | The full path to the .miz file to be loaded |






<a name="dcs-hook-v0-LoadMissionResponse"></a>

### LoadMissionResponse







<a name="dcs-hook-v0-LoadNextMissionRequest"></a>

### LoadNextMissionRequest







<a name="dcs-hook-v0-LoadNextMissionResponse"></a>

### LoadNextMissionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| loaded | [bool](#bool) |  | Was the next mission successfully loaded. SHOULD return false when the end of the mission list has been reached but DCS appears to always return true |






<a name="dcs-hook-v0-ReloadCurrentMissionRequest"></a>

### ReloadCurrentMissionRequest







<a name="dcs-hook-v0-ReloadCurrentMissionResponse"></a>

### ReloadCurrentMissionResponse







<a name="dcs-hook-v0-SetPausedRequest"></a>

### SetPausedRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| paused | [bool](#bool) |  |  |






<a name="dcs-hook-v0-SetPausedResponse"></a>

### SetPausedResponse







<a name="dcs-hook-v0-StopMissionRequest"></a>

### StopMissionRequest







<a name="dcs-hook-v0-StopMissionResponse"></a>

### StopMissionResponse







<a name="dcs-hook-v0-UnbanPlayerRequest"></a>

### UnbanPlayerRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ucid | [string](#string) |  | The globally unique ID of the player |






<a name="dcs-hook-v0-UnbanPlayerResponse"></a>

### UnbanPlayerResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unbanned | [bool](#bool) |  | Was the player successfully unbanned |





 

 

 


<a name="dcs-hook-v0-HookService"></a>

### HookService
APis that are part of the hook environment

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetMissionName | [GetMissionNameRequest](#dcs-hook-v0-GetMissionNameRequest) | [GetMissionNameResponse](#dcs-hook-v0-GetMissionNameResponse) | https://wiki.hoggitworld.com/view/DCS_func_getMissionName |
| GetMissionFilename | [GetMissionFilenameRequest](#dcs-hook-v0-GetMissionFilenameRequest) | [GetMissionFilenameResponse](#dcs-hook-v0-GetMissionFilenameResponse) | https://wiki.hoggitworld.com/view/DCS_func_getMissionFilename |
| GetMissionDescription | [GetMissionDescriptionRequest](#dcs-hook-v0-GetMissionDescriptionRequest) | [GetMissionDescriptionResponse](#dcs-hook-v0-GetMissionDescriptionResponse) | https://wiki.hoggitworld.com/view/DCS_func_getMissionDescription |
| GetPaused | [GetPausedRequest](#dcs-hook-v0-GetPausedRequest) | [GetPausedResponse](#dcs-hook-v0-GetPausedResponse) | https://wiki.hoggitworld.com/view/DCS_func_getPause |
| SetPaused | [SetPausedRequest](#dcs-hook-v0-SetPausedRequest) | [SetPausedResponse](#dcs-hook-v0-SetPausedResponse) | https://wiki.hoggitworld.com/view/DCS_func_setPause |
| StopMission | [StopMissionRequest](#dcs-hook-v0-StopMissionRequest) | [StopMissionResponse](#dcs-hook-v0-StopMissionResponse) | https://wiki.hoggitworld.com/view/DCS_func_stopMission |
| ReloadCurrentMission | [ReloadCurrentMissionRequest](#dcs-hook-v0-ReloadCurrentMissionRequest) | [ReloadCurrentMissionResponse](#dcs-hook-v0-ReloadCurrentMissionResponse) | Reload the currently running mission |
| LoadNextMission | [LoadNextMissionRequest](#dcs-hook-v0-LoadNextMissionRequest) | [LoadNextMissionResponse](#dcs-hook-v0-LoadNextMissionResponse) | Load the next mission in the server mission list. Note that it does not loop back to the first mission once the end of the mission list has been reached |
| LoadMission | [LoadMissionRequest](#dcs-hook-v0-LoadMissionRequest) | [LoadMissionResponse](#dcs-hook-v0-LoadMissionResponse) | Load a specific mission file. This does not need to be in the mission list. |
| Eval | [EvalRequest](#dcs-hook-v0-EvalRequest) | [EvalResponse](#dcs-hook-v0-EvalResponse) | Evaluate some Lua inside of the hook environment and return the result as a JSON string. Disabled by default. |
| ExitProcess | [ExitProcessRequest](#dcs-hook-v0-ExitProcessRequest) | [ExitProcessResponse](#dcs-hook-v0-ExitProcessResponse) | https://wiki.hoggitworld.com/view/DCS_func_exitProcess |
| IsMultiplayer | [IsMultiplayerRequest](#dcs-hook-v0-IsMultiplayerRequest) | [IsMultiplayerResponse](#dcs-hook-v0-IsMultiplayerResponse) | https://wiki.hoggitworld.com/view/DCS_func_isMultiplayer |
| IsServer | [IsServerRequest](#dcs-hook-v0-IsServerRequest) | [IsServerResponse](#dcs-hook-v0-IsServerResponse) | https://wiki.hoggitworld.com/view/DCS_func_isServer |
| BanPlayer | [BanPlayerRequest](#dcs-hook-v0-BanPlayerRequest) | [BanPlayerResponse](#dcs-hook-v0-BanPlayerResponse) | Bans a player that is currently connected to the server |
| UnbanPlayer | [UnbanPlayerRequest](#dcs-hook-v0-UnbanPlayerRequest) | [UnbanPlayerResponse](#dcs-hook-v0-UnbanPlayerResponse) | Unbans a player via their globally unique ID |
| GetBannedPlayers | [GetBannedPlayersRequest](#dcs-hook-v0-GetBannedPlayersRequest) | [GetBannedPlayersResponse](#dcs-hook-v0-GetBannedPlayersResponse) | Get a list of all the banned players |
| GetUnitType | [GetUnitTypeRequest](#dcs-hook-v0-GetUnitTypeRequest) | [GetUnitTypeResponse](#dcs-hook-v0-GetUnitTypeResponse) | https://wiki.hoggitworld.com/view/DCS_func_getUnitType |
| GetRealTime | [GetRealTimeRequest](#dcs-hook-v0-GetRealTimeRequest) | [GetRealTimeResponse](#dcs-hook-v0-GetRealTimeResponse) | https://wiki.hoggitworld.com/view/DCS_func_getRealTime |
| GetBallisticsCount | [GetBallisticsCountRequest](#dcs-hook-v0-GetBallisticsCountRequest) | [GetBallisticsCountResponse](#dcs-hook-v0-GetBallisticsCountResponse) | Get a count of ballistics objects |
| GetModelTime | [GetModelTimeRequest](#dcs-hook-v0-GetModelTimeRequest) | [GetModelTimeResponse](#dcs-hook-v0-GetModelTimeResponse) |  |
| GetMissionOptions | [GetMissionOptionsRequest](#dcs-hook-v0-GetMissionOptionsRequest) | [GetMissionOptionsResponse](#dcs-hook-v0-GetMissionOptionsResponse) |  |
| GetCurrentMission | [GetCurrentMissionRequest](#dcs-hook-v0-GetCurrentMissionRequest) | [GetCurrentMissionResponse](#dcs-hook-v0-GetCurrentMissionResponse) |  |
| GetAvailableSlots | [GetAvailableSlotsRequest](#dcs-hook-v0-GetAvailableSlotsRequest) | [GetAvailableSlotsResponse](#dcs-hook-v0-GetAvailableSlotsResponse) |  |
| GetAvailableCoalitions | [GetAvailableCoalitionsRequest](#dcs-hook-v0-GetAvailableCoalitionsRequest) | [GetAvailableCoalitionsResponse](#dcs-hook-v0-GetAvailableCoalitionsResponse) |  |
| GetMissionResult | [GetMissionResultRequest](#dcs-hook-v0-GetMissionResultRequest) | [GetMissionResultResponse](#dcs-hook-v0-GetMissionResultResponse) |  |
| GetUnitProperty | [GetUnitPropertyRequest](#dcs-hook-v0-GetUnitPropertyRequest) | [GetUnitPropertyResponse](#dcs-hook-v0-GetUnitPropertyResponse) |  |

 



<a name="dcs_land_v0_land-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/land/v0/land.proto



<a name="dcs-land-v0-FindPathOnRoadsRequest"></a>

### FindPathOnRoadsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| road_type | [string](#string) |  |  |
| start | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| end | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |






<a name="dcs-land-v0-FindPathOnRoadsResponse"></a>

### FindPathOnRoadsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path_json | [string](#string) |  |  |






<a name="dcs-land-v0-GetClosestPointOnRoadsRequest"></a>

### GetClosestPointOnRoadsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| road_type | [string](#string) |  |  |






<a name="dcs-land-v0-GetClosestPointOnRoadsResponse"></a>

### GetClosestPointOnRoadsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |






<a name="dcs-land-v0-GetIPRequest"></a>

### GetIPRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| origin | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| direction | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| max_dist | [float](#float) |  |  |






<a name="dcs-land-v0-GetIPResponse"></a>

### GetIPResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| intersection_point | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |






<a name="dcs-land-v0-GetSurfaceHeightWithSeabedRequest"></a>

### GetSurfaceHeightWithSeabedRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |






<a name="dcs-land-v0-GetSurfaceHeightWithSeabedResponse"></a>

### GetSurfaceHeightWithSeabedResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| height | [float](#float) |  |  |






<a name="dcs-land-v0-GetSurfaceTypeRequest"></a>

### GetSurfaceTypeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |






<a name="dcs-land-v0-GetSurfaceTypeResponse"></a>

### GetSurfaceTypeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| surface_type | [SurfaceType](#dcs-land-v0-SurfaceType) |  |  |






<a name="dcs-land-v0-GetTerrainHeightRequest"></a>

### GetTerrainHeightRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |






<a name="dcs-land-v0-GetTerrainHeightResponse"></a>

### GetTerrainHeightResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| height | [float](#float) |  |  |






<a name="dcs-land-v0-IsVisibleRequest"></a>

### IsVisibleRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| from | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| to | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |






<a name="dcs-land-v0-IsVisibleResponse"></a>

### IsVisibleResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| visible | [bool](#bool) |  |  |






<a name="dcs-land-v0-ProfileRequest"></a>

### ProfileRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| from | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| to | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |






<a name="dcs-land-v0-ProfileResponse"></a>

### ProfileResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| profile_json | [string](#string) |  |  |





 


<a name="dcs-land-v0-SurfaceType"></a>

### SurfaceType
Existing values are part of the public API and cannot be renamed.
protolint:disable ENUM_FIELD_NAMES_PREFIX ENUM_FIELD_NAMES_ZERO_VALUE_END_WITH

| Name | Number | Description |
| ---- | ------ | ----------- |
| LAND | 0 |  |
| SHALLOW_WATER | 1 |  |
| WATER | 2 |  |
| ROAD | 3 |  |
| RUNWAY | 4 |  |


 

 


<a name="dcs-land-v0-LandService"></a>

### LandService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetTerrainHeight | [GetTerrainHeightRequest](#dcs-land-v0-GetTerrainHeightRequest) | [GetTerrainHeightResponse](#dcs-land-v0-GetTerrainHeightResponse) |  |
| GetSurfaceType | [GetSurfaceTypeRequest](#dcs-land-v0-GetSurfaceTypeRequest) | [GetSurfaceTypeResponse](#dcs-land-v0-GetSurfaceTypeResponse) |  |
| IsVisible | [IsVisibleRequest](#dcs-land-v0-IsVisibleRequest) | [IsVisibleResponse](#dcs-land-v0-IsVisibleResponse) |  |
| GetClosestPointOnRoads | [GetClosestPointOnRoadsRequest](#dcs-land-v0-GetClosestPointOnRoadsRequest) | [GetClosestPointOnRoadsResponse](#dcs-land-v0-GetClosestPointOnRoadsResponse) |  |
| GetSurfaceHeightWithSeabed | [GetSurfaceHeightWithSeabedRequest](#dcs-land-v0-GetSurfaceHeightWithSeabedRequest) | [GetSurfaceHeightWithSeabedResponse](#dcs-land-v0-GetSurfaceHeightWithSeabedResponse) |  |
| FindPathOnRoads | [FindPathOnRoadsRequest](#dcs-land-v0-FindPathOnRoadsRequest) | [FindPathOnRoadsResponse](#dcs-land-v0-FindPathOnRoadsResponse) |  |
| GetIP | [GetIPRequest](#dcs-land-v0-GetIPRequest) | [GetIPResponse](#dcs-land-v0-GetIPResponse) |  |
| Profile | [ProfileRequest](#dcs-land-v0-ProfileRequest) | [ProfileResponse](#dcs-land-v0-ProfileResponse) |  |

 



<a name="dcs_metadata_v0_metadata-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/metadata/v0/metadata.proto



<a name="dcs-metadata-v0-GetHealthRequest"></a>

### GetHealthRequest







<a name="dcs-metadata-v0-GetHealthResponse"></a>

### GetHealthResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| alive | [bool](#bool) |  |  |






<a name="dcs-metadata-v0-GetVersionRequest"></a>

### GetVersionRequest







<a name="dcs-metadata-v0-GetVersionResponse"></a>

### GetVersionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [string](#string) |  |  |





 

 

 


<a name="dcs-metadata-v0-MetadataService"></a>

### MetadataService
A service to get administrative/meta data like server health checks and version

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetHealth | [GetHealthRequest](#dcs-metadata-v0-GetHealthRequest) | [GetHealthResponse](#dcs-metadata-v0-GetHealthResponse) |  |
| GetVersion | [GetVersionRequest](#dcs-metadata-v0-GetVersionRequest) | [GetVersionResponse](#dcs-metadata-v0-GetVersionResponse) |  |

 



<a name="dcs_mission_v0_mission-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/mission/v0/mission.proto



<a name="dcs-mission-v0-AddCoalitionCommandRequest"></a>

### AddCoalitionCommandRequest
Adds an F10 radio command visible to all players in the specified coalition.
When the player activates the command then a `coalitionCommand` event will
be emitted to all connected DCS-gRPC clients for processing as they see fit.
The emitted event will include the coalition.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The coalition whose players will be able to see and run the command |
| name | [string](#string) |  | The name of the command that is displayed to the player. It will form the last entry in the returned path. |
| path | [string](#string) | repeated | The menu path the command will appear under. This can be empty if you want the command to be on the first level under the F10 menu. This path must already have been created. |
| details | [google.protobuf.Struct](#google-protobuf-Struct) |  | A struct containing data that will be included in the emitted event to the DCS-gRPC clients |






<a name="dcs-mission-v0-AddCoalitionCommandResponse"></a>

### AddCoalitionCommandResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | The full path to the command, including the command name. Use this path to delete the command. |






<a name="dcs-mission-v0-AddCoalitionCommandSubMenuRequest"></a>

### AddCoalitionCommandSubMenuRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The coalition whose players will be able to see the submenu |
| name | [string](#string) |  | The name of the submenu that is displayed to the player. It will form the last entry in the returned path. |
| path | [string](#string) | repeated | The menu path the submenu will appear under. This can be empty if you want the submenu to be on the first level under the F10 menu. This path must already have been created using this command. you cannot create a nested submenu tree in one command. |






<a name="dcs-mission-v0-AddCoalitionCommandSubMenuResponse"></a>

### AddCoalitionCommandSubMenuResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | The full path to the submenu, including the submenu name. Use this path to add another submenu or command underneath it or delete the submenu. |






<a name="dcs-mission-v0-AddGroupCommandRequest"></a>

### AddGroupCommandRequest
Adds an F10 radio command visible to all players in the specified group.
When the player activates the command then a `groupCommand` event will
be emitted to all connected DCS-gRPC clients for processing as they see fit.
The emitted event will include the group name.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  | The name of the group whose players will be able to see and execute the command. TODO (Figure out if this persists across spawns) |
| name | [string](#string) |  | The name of the command that is displayed to the player. It will form the last entry in the returned path. |
| path | [string](#string) | repeated | The menu path the command will appear under. This can be empty if you want the command to be on the first level under the F10 menu. This path must already have been created. |
| details | [google.protobuf.Struct](#google-protobuf-Struct) |  | A struct containing data that will be included in the emitted event to the DCS-gRPC clients |






<a name="dcs-mission-v0-AddGroupCommandResponse"></a>

### AddGroupCommandResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | The full path to the command, including the command name. Use this path to delete the command. |






<a name="dcs-mission-v0-AddGroupCommandSubMenuRequest"></a>

### AddGroupCommandSubMenuRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  | The name of the group whose players will be able to see the submenu |
| name | [string](#string) |  | The name of the submenu that is displayed to the player. It will form the last entry in the returned path. |
| path | [string](#string) | repeated | The menu path the submenu will appear under. This can be empty if you want the submenu to be on the first level under the F10 menu. This path must already have been created using this command. you cannot create a nested submenu tree in one command. |






<a name="dcs-mission-v0-AddGroupCommandSubMenuResponse"></a>

### AddGroupCommandSubMenuResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | The full path to the submenu, including the submenu name. Use this path to add another submenu or command underneath it or delete the submenu. |






<a name="dcs-mission-v0-AddMissionCommandRequest"></a>

### AddMissionCommandRequest
Adds an F10 radio command visible to all players in all coalitions.
When the player activates the command then a `missionCommand` event will be
emitted to all connected DCS-gRPC clients for processing as they see fit.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | The name of the command that is displayed to the player. It will form the last entry in the returned path. |
| path | [string](#string) | repeated | The menu path the command will appear under. This can be empty if you want the command to be on the first level under the F10 menu. This path must already have been created. |
| details | [google.protobuf.Struct](#google-protobuf-Struct) |  | A struct containing data that will be included in the emitted event to the DCS-gRPC clients |






<a name="dcs-mission-v0-AddMissionCommandResponse"></a>

### AddMissionCommandResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | The full path to the command, including the command name. Use this path to delete the command. |






<a name="dcs-mission-v0-AddMissionCommandSubMenuRequest"></a>

### AddMissionCommandSubMenuRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | The name of the submenu that is displayed to the player. It will form the last entry in the returned path. |
| path | [string](#string) | repeated | The menu path the submenu will appear under. This can be empty if you want the submenu to be on the first level under the F10 menu. This path must already have been created using this command. you cannot create a nested submenu tree in one command. |






<a name="dcs-mission-v0-AddMissionCommandSubMenuResponse"></a>

### AddMissionCommandSubMenuResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | The full path to the submenu, including the submenu name. Use this path to add another submenu or command underneath it or delete the submenu. |






<a name="dcs-mission-v0-GetScenarioCurrentTimeRequest"></a>

### GetScenarioCurrentTimeRequest







<a name="dcs-mission-v0-GetScenarioCurrentTimeResponse"></a>

### GetScenarioCurrentTimeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| datetime | [string](#string) |  |  |






<a name="dcs-mission-v0-GetScenarioStartTimeRequest"></a>

### GetScenarioStartTimeRequest







<a name="dcs-mission-v0-GetScenarioStartTimeResponse"></a>

### GetScenarioStartTimeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| datetime | [string](#string) |  |  |






<a name="dcs-mission-v0-GetSessionIdRequest"></a>

### GetSessionIdRequest







<a name="dcs-mission-v0-GetSessionIdResponse"></a>

### GetSessionIdResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| session_id | [int64](#int64) |  |  |






<a name="dcs-mission-v0-RemoveCoalitionCommandItemRequest"></a>

### RemoveCoalitionCommandItemRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The coalition whose players will have the menu item removed |
| path | [string](#string) | repeated | The full path to the menu item, which can be a submenu or a command, to be removed. Deleting a menu item will delete all children it may have. |






<a name="dcs-mission-v0-RemoveCoalitionCommandItemResponse"></a>

### RemoveCoalitionCommandItemResponse







<a name="dcs-mission-v0-RemoveGroupCommandItemRequest"></a>

### RemoveGroupCommandItemRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  | The group whose players will have the menu item removed |
| path | [string](#string) | repeated | The full path to the menu item, which can be a submenu or a command, to be removed. Deleting a menu item will delete all children it may have. |






<a name="dcs-mission-v0-RemoveGroupCommandItemResponse"></a>

### RemoveGroupCommandItemResponse







<a name="dcs-mission-v0-RemoveMissionCommandItemRequest"></a>

### RemoveMissionCommandItemRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | The full path to the menu item, which can be a submenu or a command, to be removed. Deleting a menu item will delete all children it may have. |






<a name="dcs-mission-v0-RemoveMissionCommandItemResponse"></a>

### RemoveMissionCommandItemResponse







<a name="dcs-mission-v0-StreamEventsRequest"></a>

### StreamEventsRequest







<a name="dcs-mission-v0-StreamEventsResponse"></a>

### StreamEventsResponse
The DCS Event information. Contains event information and a timestamp.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  | The event&#39;s mission time. |
| shot | [StreamEventsResponse.ShotEvent](#dcs-mission-v0-StreamEventsResponse-ShotEvent) |  |  |
| hit | [StreamEventsResponse.HitEvent](#dcs-mission-v0-StreamEventsResponse-HitEvent) |  |  |
| takeoff | [StreamEventsResponse.TakeoffEvent](#dcs-mission-v0-StreamEventsResponse-TakeoffEvent) |  |  |
| land | [StreamEventsResponse.LandEvent](#dcs-mission-v0-StreamEventsResponse-LandEvent) |  |  |
| crash | [StreamEventsResponse.CrashEvent](#dcs-mission-v0-StreamEventsResponse-CrashEvent) |  |  |
| ejection | [StreamEventsResponse.EjectionEvent](#dcs-mission-v0-StreamEventsResponse-EjectionEvent) |  |  |
| refueling | [StreamEventsResponse.RefuelingEvent](#dcs-mission-v0-StreamEventsResponse-RefuelingEvent) |  |  |
| dead | [StreamEventsResponse.DeadEvent](#dcs-mission-v0-StreamEventsResponse-DeadEvent) |  |  |
| pilot_dead | [StreamEventsResponse.PilotDeadEvent](#dcs-mission-v0-StreamEventsResponse-PilotDeadEvent) |  |  |
| base_capture | [StreamEventsResponse.BaseCaptureEvent](#dcs-mission-v0-StreamEventsResponse-BaseCaptureEvent) |  |  |
| mission_start | [StreamEventsResponse.MissionStartEvent](#dcs-mission-v0-StreamEventsResponse-MissionStartEvent) |  |  |
| mission_end | [StreamEventsResponse.MissionEndEvent](#dcs-mission-v0-StreamEventsResponse-MissionEndEvent) |  |  |
| took_control | [StreamEventsResponse.TookControlEvent](#dcs-mission-v0-StreamEventsResponse-TookControlEvent) |  |  |
| refueling_stop | [StreamEventsResponse.RefuelingStopEvent](#dcs-mission-v0-StreamEventsResponse-RefuelingStopEvent) |  |  |
| birth | [StreamEventsResponse.BirthEvent](#dcs-mission-v0-StreamEventsResponse-BirthEvent) |  |  |
| human_failure | [StreamEventsResponse.HumanFailureEvent](#dcs-mission-v0-StreamEventsResponse-HumanFailureEvent) |  |  |
| detailed_failure | [StreamEventsResponse.DetailedFailureEvent](#dcs-mission-v0-StreamEventsResponse-DetailedFailureEvent) |  |  |
| engine_startup | [StreamEventsResponse.EngineStartupEvent](#dcs-mission-v0-StreamEventsResponse-EngineStartupEvent) |  |  |
| engine_shutdown | [StreamEventsResponse.EngineShutdownEvent](#dcs-mission-v0-StreamEventsResponse-EngineShutdownEvent) |  |  |
| player_enter_unit | [StreamEventsResponse.PlayerEnterUnitEvent](#dcs-mission-v0-StreamEventsResponse-PlayerEnterUnitEvent) |  |  |
| player_leave_unit | [StreamEventsResponse.PlayerLeaveUnitEvent](#dcs-mission-v0-StreamEventsResponse-PlayerLeaveUnitEvent) |  |  |
| player_comment | [StreamEventsResponse.PlayerCommentEvent](#dcs-mission-v0-StreamEventsResponse-PlayerCommentEvent) |  |  |
| shooting_start | [StreamEventsResponse.ShootingStartEvent](#dcs-mission-v0-StreamEventsResponse-ShootingStartEvent) |  |  |
| shooting_end | [StreamEventsResponse.ShootingEndEvent](#dcs-mission-v0-StreamEventsResponse-ShootingEndEvent) |  |  |
| mark_add | [StreamEventsResponse.MarkAddEvent](#dcs-mission-v0-StreamEventsResponse-MarkAddEvent) |  |  |
| mark_change | [StreamEventsResponse.MarkChangeEvent](#dcs-mission-v0-StreamEventsResponse-MarkChangeEvent) |  |  |
| mark_remove | [StreamEventsResponse.MarkRemoveEvent](#dcs-mission-v0-StreamEventsResponse-MarkRemoveEvent) |  |  |
| kill | [StreamEventsResponse.KillEvent](#dcs-mission-v0-StreamEventsResponse-KillEvent) |  |  |
| score | [StreamEventsResponse.ScoreEvent](#dcs-mission-v0-StreamEventsResponse-ScoreEvent) |  |  |
| unit_lost | [StreamEventsResponse.UnitLostEvent](#dcs-mission-v0-StreamEventsResponse-UnitLostEvent) |  |  |
| landing_after_ejection | [StreamEventsResponse.LandingAfterEjectionEvent](#dcs-mission-v0-StreamEventsResponse-LandingAfterEjectionEvent) |  |  |
| discard_chair_after_ejection | [StreamEventsResponse.DiscardChairAfterEjectionEvent](#dcs-mission-v0-StreamEventsResponse-DiscardChairAfterEjectionEvent) |  |  |
| weapon_add | [StreamEventsResponse.WeaponAddEvent](#dcs-mission-v0-StreamEventsResponse-WeaponAddEvent) |  |  |
| trigger_zone | [StreamEventsResponse.TriggerZoneEvent](#dcs-mission-v0-StreamEventsResponse-TriggerZoneEvent) |  |  |
| landing_quality_mark | [StreamEventsResponse.LandingQualityMarkEvent](#dcs-mission-v0-StreamEventsResponse-LandingQualityMarkEvent) |  |  |
| bda | [StreamEventsResponse.BdaEvent](#dcs-mission-v0-StreamEventsResponse-BdaEvent) |  |  |
| runway_takeoff | [StreamEventsResponse.RunwayTakeoffEvent](#dcs-mission-v0-StreamEventsResponse-RunwayTakeoffEvent) |  |  |
| runway_touch | [StreamEventsResponse.RunwayTouchEvent](#dcs-mission-v0-StreamEventsResponse-RunwayTouchEvent) |  |  |
| connect | [StreamEventsResponse.ConnectEvent](#dcs-mission-v0-StreamEventsResponse-ConnectEvent) |  | The following events are additions on top of DCS&#39;s own event enum, which is why they start at 8192 to give DCS plenty of space for new built-in events. |
| disconnect | [StreamEventsResponse.DisconnectEvent](#dcs-mission-v0-StreamEventsResponse-DisconnectEvent) |  |  |
| player_send_chat | [StreamEventsResponse.PlayerSendChatEvent](#dcs-mission-v0-StreamEventsResponse-PlayerSendChatEvent) |  |  |
| player_change_slot | [StreamEventsResponse.PlayerChangeSlotEvent](#dcs-mission-v0-StreamEventsResponse-PlayerChangeSlotEvent) |  |  |
| mission_command | [StreamEventsResponse.MissionCommandEvent](#dcs-mission-v0-StreamEventsResponse-MissionCommandEvent) |  |  |
| coalition_command | [StreamEventsResponse.CoalitionCommandEvent](#dcs-mission-v0-StreamEventsResponse-CoalitionCommandEvent) |  |  |
| group_command | [StreamEventsResponse.GroupCommandEvent](#dcs-mission-v0-StreamEventsResponse-GroupCommandEvent) |  |  |
| simulation_fps | [StreamEventsResponse.SimulationFpsEvent](#dcs-mission-v0-StreamEventsResponse-SimulationFpsEvent) |  |  |
| tts | [StreamEventsResponse.TtsEvent](#dcs-mission-v0-StreamEventsResponse-TtsEvent) |  |  |
| srs_connect | [StreamEventsResponse.SrsConnectEvent](#dcs-mission-v0-StreamEventsResponse-SrsConnectEvent) |  |  |
| srs_disconnect | [StreamEventsResponse.SrsDisconnectEvent](#dcs-mission-v0-StreamEventsResponse-SrsDisconnectEvent) |  |  |






<a name="dcs-mission-v0-StreamEventsResponse-BaseCaptureEvent"></a>

### StreamEventsResponse.BaseCaptureEvent
Occurs when a ground unit captures either an airbase or a farp.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that captured the base. |
| place | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) |  | The airbase that was captured, can be a FARP or Airbase |






<a name="dcs-mission-v0-StreamEventsResponse-BdaEvent"></a>

### StreamEventsResponse.BdaEvent
Battle Damage Assessment event


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  |  |
| target | [dcs.common.v0.Target](#dcs-common-v0-Target) |  |  |






<a name="dcs-mission-v0-StreamEventsResponse-BirthEvent"></a>

### StreamEventsResponse.BirthEvent
Occurs when any object is spawned into the mission.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that was spawned. |
| place | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) | optional | The airbase, farp or ship the unit took off from. |






<a name="dcs-mission-v0-StreamEventsResponse-CoalitionCommandEvent"></a>

### StreamEventsResponse.CoalitionCommandEvent



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The coalition of the player who ran the command |
| details | [google.protobuf.Struct](#google-protobuf-Struct) |  | A struct containing details of the command that was run by a player |






<a name="dcs-mission-v0-StreamEventsResponse-ConnectEvent"></a>

### StreamEventsResponse.ConnectEvent
Fired when a player connected to the server.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| addr | [string](#string) |  | The player&#39;s IP and port. |
| name | [string](#string) |  | The name of the player. |
| ucid | [string](#string) |  | The player&#39;s unique client identifier (used to ban a player). |
| id | [uint32](#uint32) |  | The player&#39;s id in the current server session (used to for name/slot/... changes). |






<a name="dcs-mission-v0-StreamEventsResponse-CrashEvent"></a>

### StreamEventsResponse.CrashEvent
Occurs when an aircraft crashes into the ground and is completely
destroyed.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that crashed. |






<a name="dcs-mission-v0-StreamEventsResponse-DeadEvent"></a>

### StreamEventsResponse.DeadEvent
Occurs when an object is completely destroyed.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that has been destroyed. |






<a name="dcs-mission-v0-StreamEventsResponse-DetailedFailureEvent"></a>

### StreamEventsResponse.DetailedFailureEvent
Occurs when a system on an aircraft fails. This can be due to damage or due
to random failures set up in the mission editor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| target | [dcs.common.v0.Target](#dcs-common-v0-Target) |  | The target the failure occurred for. |






<a name="dcs-mission-v0-StreamEventsResponse-DiscardChairAfterEjectionEvent"></a>

### StreamEventsResponse.DiscardChairAfterEjectionEvent
A pilot detached from their ejection seat.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The ejection seat. |
| target | [dcs.common.v0.Target](#dcs-common-v0-Target) |  | The pilot. |






<a name="dcs-mission-v0-StreamEventsResponse-DisconnectEvent"></a>

### StreamEventsResponse.DisconnectEvent
Fired when a player disconnected from the server
(not fired for the server&#39;s player).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | The player&#39;s id in the current server session. |
| reason | [StreamEventsResponse.DisconnectReason](#dcs-mission-v0-StreamEventsResponse-DisconnectReason) |  | The reason a player disconnected for. |






<a name="dcs-mission-v0-StreamEventsResponse-EjectionEvent"></a>

### StreamEventsResponse.EjectionEvent
Occurs when a pilot ejects from its aircraft.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The unit a pilot ejected from. |
| target | [dcs.common.v0.Target](#dcs-common-v0-Target) |  | The ejection seat. |






<a name="dcs-mission-v0-StreamEventsResponse-EngineShutdownEvent"></a>

### StreamEventsResponse.EngineShutdownEvent



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | Occurs when any aircraft shuts down its engines. |
| place | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) |  | The airbase, farp or ship the unit shut down their engine at. |






<a name="dcs-mission-v0-StreamEventsResponse-EngineStartupEvent"></a>

### StreamEventsResponse.EngineStartupEvent
Occurs when any aircraft starts its engines.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that starts its engines. |
| place | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) |  | The airbase, farp or ship the unit started their engine at. |






<a name="dcs-mission-v0-StreamEventsResponse-GroupCommandEvent"></a>

### StreamEventsResponse.GroupCommandEvent



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group | [dcs.common.v0.Group](#dcs-common-v0-Group) |  | Details of the group to which the player who ran the command is a unit of |
| details | [google.protobuf.Struct](#google-protobuf-Struct) |  | A struct containing details of the command that was run by a player |






<a name="dcs-mission-v0-StreamEventsResponse-HitEvent"></a>

### StreamEventsResponse.HitEvent
Occurs when an object is hit by a weapon.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) | optional | The object that fired the weapon. Not set when for example fyling an aircraft into a building (building will be the target and weapon_name the name of the aircraft). |
| weapon | [dcs.common.v0.Weapon](#dcs-common-v0-Weapon) |  | The weapon that the target has been hit with. |
| target | [dcs.common.v0.Target](#dcs-common-v0-Target) |  | The object that has been hit. |
| weapon_name | [string](#string) | optional | The weapon the target got hit by. |






<a name="dcs-mission-v0-StreamEventsResponse-HumanFailureEvent"></a>

### StreamEventsResponse.HumanFailureEvent
Occurs e.g. when a player controlled aircraft blacks out.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The unit the system failure occurred in. |






<a name="dcs-mission-v0-StreamEventsResponse-KillEvent"></a>

### StreamEventsResponse.KillEvent
Occurs when an object is killed by a weapon.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that fired the weapon. |
| weapon | [dcs.common.v0.Weapon](#dcs-common-v0-Weapon) |  | The weapon that the target has been killed with. |
| target | [dcs.common.v0.Target](#dcs-common-v0-Target) |  | The object that has been killed. |
| weapon_name | [string](#string) | optional | The name of the weapon that killed the target (exists instead of weapon for weapons that trigger the shooting start and end events). |






<a name="dcs-mission-v0-StreamEventsResponse-LandEvent"></a>

### StreamEventsResponse.LandEvent
Occurs when an aircraft lands at an airbase, farp or ship.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that landed. |
| place | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) |  | The airbase, farp or ship the unit landed at. |






<a name="dcs-mission-v0-StreamEventsResponse-LandingAfterEjectionEvent"></a>

### StreamEventsResponse.LandingAfterEjectionEvent
A pilot detached from their ejection seat.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The ejected pilot. |
| place | [dcs.common.v0.Position](#dcs-common-v0-Position) |  | The position the pilot landed at. |






<a name="dcs-mission-v0-StreamEventsResponse-LandingQualityMarkEvent"></a>

### StreamEventsResponse.LandingQualityMarkEvent
Occurs when an aircraft receives an LSO rating after recovering on an
aircraft carrier.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The aircraft that received the rating. |
| comment | [string](#string) |  | The rating. |
| place | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) |  | The ship the unit landed at. |






<a name="dcs-mission-v0-StreamEventsResponse-MarkAddEvent"></a>

### StreamEventsResponse.MarkAddEvent
Occurs when marks get added to the mission by players or scripting
functions.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that added the mark. |
| group_id | [uint64](#uint64) |  | The group the mark&#39;s visibility is restricted for. |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The coalition the mark&#39;s visibility is restricted for. |
| id | [uint32](#uint32) |  | The mark&#39;s id. |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  | The position the mark has been added at. |
| text | [string](#string) |  | The mark&#39;s label. |






<a name="dcs-mission-v0-StreamEventsResponse-MarkChangeEvent"></a>

### StreamEventsResponse.MarkChangeEvent
Occurs when marks got changed.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that changed the mark. |
| group_id | [uint64](#uint64) |  | The group the mark&#39;s visibility is restricted for. |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The coalition the mark&#39;s visibility is restricted for. |
| id | [uint32](#uint32) |  | The mark&#39;s id. |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  | The position of the changed mark. |
| text | [string](#string) |  | The mark&#39;s label. |






<a name="dcs-mission-v0-StreamEventsResponse-MarkRemoveEvent"></a>

### StreamEventsResponse.MarkRemoveEvent
Occurs when marks get removed.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that removed the mark. |
| group_id | [uint64](#uint64) |  | The group the mark&#39;s visibility is restricted for. |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The coalition the mark&#39;s visibility is restricted for. |
| id | [uint32](#uint32) |  | The mark&#39;s id. |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  | The position the mark has been removed from. |
| text | [string](#string) |  | The mark&#39;s label. |






<a name="dcs-mission-v0-StreamEventsResponse-MissionCommandEvent"></a>

### StreamEventsResponse.MissionCommandEvent



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| details | [google.protobuf.Struct](#google-protobuf-Struct) |  | A struct containing details of the command that was run by a player |






<a name="dcs-mission-v0-StreamEventsResponse-MissionEndEvent"></a>

### StreamEventsResponse.MissionEndEvent
Occurs when the mission stops.






<a name="dcs-mission-v0-StreamEventsResponse-MissionStartEvent"></a>

### StreamEventsResponse.MissionStartEvent
Occurs when the mission starts.






<a name="dcs-mission-v0-StreamEventsResponse-PilotDeadEvent"></a>

### StreamEventsResponse.PilotDeadEvent
Occurs when a pilot of an aircraft is killed. Can occur either if the
player is alive and crashes (in this case both this and the [CrashEvent]
event will be fired) or if a weapon kills the pilot without completely
destroying the plane.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The unit the pilot has died in. |






<a name="dcs-mission-v0-StreamEventsResponse-PlayerChangeSlotEvent"></a>

### StreamEventsResponse.PlayerChangeSlotEvent
fired when the player changes across to a slot


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| player_id | [uint32](#uint32) |  | The player&#39;s id in the current server session. |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The slot&#39;s coalition |
| slot_id | [string](#string) |  | The slot&#39;s identifier |






<a name="dcs-mission-v0-StreamEventsResponse-PlayerCommentEvent"></a>

### StreamEventsResponse.PlayerCommentEvent
Occurs when a player comment is made.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  |  |
| comment | [string](#string) |  |  |






<a name="dcs-mission-v0-StreamEventsResponse-PlayerEnterUnitEvent"></a>

### StreamEventsResponse.PlayerEnterUnitEvent
Occurs when a player takes direct control of a unit.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The unit the player took control of. |






<a name="dcs-mission-v0-StreamEventsResponse-PlayerLeaveUnitEvent"></a>

### StreamEventsResponse.PlayerLeaveUnitEvent
Occurs when a player relieves direct control of a unit.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The unit the player relieves control of. |






<a name="dcs-mission-v0-StreamEventsResponse-PlayerSendChatEvent"></a>

### StreamEventsResponse.PlayerSendChatEvent
Occurs when a chat message is sent on the server


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| player_id | [uint32](#uint32) |  | The player&#39;s id in the current server session. |
| message | [string](#string) |  | what was typed |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | player&#39;s coalition |
| player_name | [string](#string) |  | player&#39;s name |
| to_all | [bool](#bool) |  | sent to all chat |






<a name="dcs-mission-v0-StreamEventsResponse-RefuelingEvent"></a>

### StreamEventsResponse.RefuelingEvent
Occurs when an aircraft connects with a tanker and begins taking on fuel.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that is receiving fuel. |






<a name="dcs-mission-v0-StreamEventsResponse-RefuelingStopEvent"></a>

### StreamEventsResponse.RefuelingStopEvent
Occurs when an aircraft is finished taking fuel.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | he unit that was receiving fuel. |






<a name="dcs-mission-v0-StreamEventsResponse-RunwayTakeoffEvent"></a>

### StreamEventsResponse.RunwayTakeoffEvent
Occurs when an aircraft takes off from an airbase, farp, or ship.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that took off. |
| place | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) |  | The airbase, farp or ship the unit took off from. |






<a name="dcs-mission-v0-StreamEventsResponse-RunwayTouchEvent"></a>

### StreamEventsResponse.RunwayTouchEvent
Occurs when an aircraft lands at an airbase, farp or ship.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that landed. |
| place | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) |  | The airbase, farp or ship the unit landed at. |






<a name="dcs-mission-v0-StreamEventsResponse-ScoreEvent"></a>

### StreamEventsResponse.ScoreEvent
A score change (doesn&#39;t contain any useful information)






<a name="dcs-mission-v0-StreamEventsResponse-ShootingEndEvent"></a>

### StreamEventsResponse.ShootingEndEvent
Occurs when a unit stops firing a machine gun- or autocannon-based weapon.
Event will always correspond with a [ShootingStartEvent] event.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that was shooting and has no stopped firing. |
| weapon_name | [string](#string) |  | The name of the shoot weapon. |






<a name="dcs-mission-v0-StreamEventsResponse-ShootingStartEvent"></a>

### StreamEventsResponse.ShootingStartEvent
Occurs when a unit begins firing a machine gun- or autocannon-based weapon
(weapons with a high rate of fire). Other weapons are handled by
[ShotEvent].


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that started firing. |
| weapon_name | [string](#string) |  | The name of the shoot weapon. |






<a name="dcs-mission-v0-StreamEventsResponse-ShotEvent"></a>

### StreamEventsResponse.ShotEvent
Occurs when a unit fires a weapon (but no machine gun- or autocannon-based
weapons - those are handled by [ShootingStartEvent]).


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that fired the weapon. |
| weapon | [dcs.common.v0.Weapon](#dcs-common-v0-Weapon) |  | The weapon that has been fired. |






<a name="dcs-mission-v0-StreamEventsResponse-SimulationFpsEvent"></a>

### StreamEventsResponse.SimulationFpsEvent
Fired every second containing simulation FPS information since the previous
event.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| average | [double](#double) |  | The average FPS since the last event. |






<a name="dcs-mission-v0-StreamEventsResponse-SrsConnectEvent"></a>

### StreamEventsResponse.SrsConnectEvent
Fired every time a player occuping a unit connects to a frequency on SRS.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit | [dcs.common.v0.Unit](#dcs-common-v0-Unit) |  | The unit that connected to a frequency in SRS. |
| frequency | [uint64](#uint64) |  | The radio frequency in Hz the unit connected to. |






<a name="dcs-mission-v0-StreamEventsResponse-SrsDisconnectEvent"></a>

### StreamEventsResponse.SrsDisconnectEvent
Fired every time a player occuping a unit disconnects from a frequency on
SRS. It is not fired when the player leaves the unit or the unit dies.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit | [dcs.common.v0.Unit](#dcs-common-v0-Unit) |  | The unit that disconnected from a frequency in SRS. |
| frequency | [uint64](#uint64) |  | The radio frequency in Hz the unit disconnected from. |






<a name="dcs-mission-v0-StreamEventsResponse-TakeoffEvent"></a>

### StreamEventsResponse.TakeoffEvent
Occurs when an aircraft takes off from an airbase, farp, or ship.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that took off. |
| place | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) |  | The airbase, farp or ship the unit took off from. |






<a name="dcs-mission-v0-StreamEventsResponse-TookControlEvent"></a>

### StreamEventsResponse.TookControlEvent
Occurs when a player takes control.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that the player took control of. |






<a name="dcs-mission-v0-StreamEventsResponse-TriggerZoneEvent"></a>

### StreamEventsResponse.TriggerZoneEvent
Occurs when a unit enters a trigger zone.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  |  |






<a name="dcs-mission-v0-StreamEventsResponse-TtsEvent"></a>

### StreamEventsResponse.TtsEvent
Fired for every TTS request that contains the `text_plain` field, for other
clients to use e.g. for accessibility use-cases.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [string](#string) |  | The plain text that got transmitted. |
| frequency | [uint64](#uint64) |  | The radio frequency in Hz the transmission got send to. |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The coalition of the transmission. |
| srs_client_name | [string](#string) | optional | Custom name of the SRS client used for the transmission. |






<a name="dcs-mission-v0-StreamEventsResponse-UnitLostEvent"></a>

### StreamEventsResponse.UnitLostEvent
A unit got destroyed.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that got destroyed weapon. |






<a name="dcs-mission-v0-StreamEventsResponse-WeaponAddEvent"></a>

### StreamEventsResponse.WeaponAddEvent
Fired for each payload of an aircraft spawened midair.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiator | [dcs.common.v0.Initiator](#dcs-common-v0-Initiator) |  | The object that got spawned. |
| weapon_name | [string](#string) |  | The name of the payload. |






<a name="dcs-mission-v0-StreamUnitsRequest"></a>

### StreamUnitsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| poll_rate | [uint32](#uint32) | optional | The poll rate in seconds at which the gRPC server communicates with the DCS mission to retrieve the latest unit positions. The lower the `poll_rate` the higher the amount of requests send to to the DCS mission. Default: 5 |
| max_backoff | [uint32](#uint32) | optional | The maximum backoff in seconds which the gRPC postpones polling units that haven&#39;t moved recently. This is an optimization to dynamically reduce the poll rate for stationary units. Set it to the same value as `poll_rate` to disable the backoff. Default: 30 |
| category | [dcs.common.v0.GroupCategory](#dcs-common-v0-GroupCategory) |  | The type of the unit to stream movements. Different categories of units would move at different speeds, which allows the stream to be configured with the appropriate polling rates. `GROUP_CATEGORY_UNSPECIFIED` would return all the units. |






<a name="dcs-mission-v0-StreamUnitsResponse"></a>

### StreamUnitsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  |  |
| unit | [dcs.common.v0.Unit](#dcs-common-v0-Unit) |  | The unit is either new or its position or attitude changed. |
| gone | [StreamUnitsResponse.UnitGone](#dcs-mission-v0-StreamUnitsResponse-UnitGone) |  | The unit does not exist anymore. |






<a name="dcs-mission-v0-StreamUnitsResponse-UnitGone"></a>

### StreamUnitsResponse.UnitGone



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| name | [string](#string) |  |  |





 


<a name="dcs-mission-v0-StreamEventsResponse-DisconnectReason"></a>

### StreamEventsResponse.DisconnectReason
The reason a player disconnected for.

| Name | Number | Description |
| ---- | ------ | ----------- |
| DISCONNECT_REASON_UNSPECIFIED | 0 |  |
| DISCONNECT_REASON_THATS_OKAY | 1 |  |
| DISCONNECT_REASON_INVALID_ADDRESS | 2 |  |
| DISCONNECT_REASON_CONNECT_FAILED | 3 |  |
| DISCONNECT_REASON_WRONG_VERSION | 4 |  |
| DISCONNECT_REASON_PROTOCOL_ERROR | 5 |  |
| DISCONNECT_REASON_TIMEOUT | 6 |  |
| DISCONNECT_REASON_INVALID_PASSWORD | 101 |  |
| DISCONNECT_REASON_BANNED | 102 |  |
| DISCONNECT_REASON_BAD_CALLSIGN | 103 |  |
| DISCONNECT_REASON_TAINTED_CLIENT | 104 |  |
| DISCONNECT_REASON_KICKED | 105 |  |
| DISCONNECT_REASON_REFUSED | 106 |  |
| DISCONNECT_REASON_DENIED_TRIAL_ONLY | 107 |  |


 

 


<a name="dcs-mission-v0-MissionService"></a>

### MissionService
Contains the streaming APIs that streaming information out of the DCS server.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| StreamEvents | [StreamEventsRequest](#dcs-mission-v0-StreamEventsRequest) | [StreamEventsResponse](#dcs-mission-v0-StreamEventsResponse) stream | Streams DCS game generated Events. See https://wiki.hoggitworld.com/view/Category:Events |
| StreamUnits | [StreamUnitsRequest](#dcs-mission-v0-StreamUnitsRequest) | [StreamUnitsResponse](#dcs-mission-v0-StreamUnitsResponse) stream | Streams unit updates Provides similar functionality as Tacview but at a much lower update rate so puts less load on the server. Suitable for things like online maps but not as a Tacview replacement. |
| GetScenarioStartTime | [GetScenarioStartTimeRequest](#dcs-mission-v0-GetScenarioStartTimeRequest) | [GetScenarioStartTimeResponse](#dcs-mission-v0-GetScenarioStartTimeResponse) | Returns the mission&#39;s in-game starttime as an ISO 8601 formatted datetime string. |
| GetScenarioCurrentTime | [GetScenarioCurrentTimeRequest](#dcs-mission-v0-GetScenarioCurrentTimeRequest) | [GetScenarioCurrentTimeResponse](#dcs-mission-v0-GetScenarioCurrentTimeResponse) | Returns the mission&#39;s in-game current time as an ISO 8601 formatted datetime string. |
| AddMissionCommand | [AddMissionCommandRequest](#dcs-mission-v0-AddMissionCommandRequest) | [AddMissionCommandResponse](#dcs-mission-v0-AddMissionCommandResponse) | Adds a new mission command See https://wiki.hoggitworld.com/view/DCS_func_addCommand |
| AddMissionCommandSubMenu | [AddMissionCommandSubMenuRequest](#dcs-mission-v0-AddMissionCommandSubMenuRequest) | [AddMissionCommandSubMenuResponse](#dcs-mission-v0-AddMissionCommandSubMenuResponse) | Adds a new command sub menu See https://wiki.hoggitworld.com/view/DCS_func_addSubMenu |
| RemoveMissionCommandItem | [RemoveMissionCommandItemRequest](#dcs-mission-v0-RemoveMissionCommandItemRequest) | [RemoveMissionCommandItemResponse](#dcs-mission-v0-RemoveMissionCommandItemResponse) | Removes a registered mission command. See https://wiki.hoggitworld.com/view/DCS_func_removeItem |
| AddCoalitionCommand | [AddCoalitionCommandRequest](#dcs-mission-v0-AddCoalitionCommandRequest) | [AddCoalitionCommandResponse](#dcs-mission-v0-AddCoalitionCommandResponse) | Adds a new coalition command See https://wiki.hoggitworld.com/view/DCS_func_addCommandForCoalition |
| AddCoalitionCommandSubMenu | [AddCoalitionCommandSubMenuRequest](#dcs-mission-v0-AddCoalitionCommandSubMenuRequest) | [AddCoalitionCommandSubMenuResponse](#dcs-mission-v0-AddCoalitionCommandSubMenuResponse) | Adds a new coalition command sub menu See https://wiki.hoggitworld.com/view/DCS_func_addSubMenuForCoalition |
| RemoveCoalitionCommandItem | [RemoveCoalitionCommandItemRequest](#dcs-mission-v0-RemoveCoalitionCommandItemRequest) | [RemoveCoalitionCommandItemResponse](#dcs-mission-v0-RemoveCoalitionCommandItemResponse) | Removes a registered coalition command. See https://wiki.hoggitworld.com/view/DCS_func_removeItemForCoalition |
| AddGroupCommand | [AddGroupCommandRequest](#dcs-mission-v0-AddGroupCommandRequest) | [AddGroupCommandResponse](#dcs-mission-v0-AddGroupCommandResponse) | Adds a new group command See https://wiki.hoggitworld.com/view/DCS_func_addCommandForGroup |
| AddGroupCommandSubMenu | [AddGroupCommandSubMenuRequest](#dcs-mission-v0-AddGroupCommandSubMenuRequest) | [AddGroupCommandSubMenuResponse](#dcs-mission-v0-AddGroupCommandSubMenuResponse) | Adds a new group command sub menu See https://wiki.hoggitworld.com/view/DCS_func_addSubMenuForGroup |
| RemoveGroupCommandItem | [RemoveGroupCommandItemRequest](#dcs-mission-v0-RemoveGroupCommandItemRequest) | [RemoveGroupCommandItemResponse](#dcs-mission-v0-RemoveGroupCommandItemResponse) | Removes a group coalition command. See https://wiki.hoggitworld.com/view/DCS_func_removeItemForGroup |
| GetSessionId | [GetSessionIdRequest](#dcs-mission-v0-GetSessionIdRequest) | [GetSessionIdResponse](#dcs-mission-v0-GetSessionIdResponse) | Returns an ID for the current session. The ID will change upon mission change or server restart. |

 



<a name="dcs_net_v0_net-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/net/v0/net.proto



<a name="dcs-net-v0-ForcePlayerSlotRequest"></a>

### ForcePlayerSlotRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| player_id | [uint32](#uint32) |  |  |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| slot_id | [string](#string) |  |  |






<a name="dcs-net-v0-ForcePlayerSlotResponse"></a>

### ForcePlayerSlotResponse







<a name="dcs-net-v0-GetPlayersRequest"></a>

### GetPlayersRequest







<a name="dcs-net-v0-GetPlayersResponse"></a>

### GetPlayersResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| players | [GetPlayersResponse.GetPlayerInfo](#dcs-net-v0-GetPlayersResponse-GetPlayerInfo) | repeated | list of all the players connected to the server |






<a name="dcs-net-v0-GetPlayersResponse-GetPlayerInfo"></a>

### GetPlayersResponse.GetPlayerInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  | the player id |
| name | [string](#string) |  | player&#39;s online name |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | coalition which player is slotted in |
| slot | [string](#string) |  | the slot identifier |
| ping | [uint32](#uint32) |  | the ping of the player |
| remote_address | [string](#string) |  | the connection ip address and port the client has established with the server |
| ucid | [string](#string) |  | the unique identifier for the player |
| locale | [string](#string) |  | abbreviated language (locale) e.g. &#34;en&#34; |






<a name="dcs-net-v0-KickPlayerRequest"></a>

### KickPlayerRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-net-v0-KickPlayerResponse"></a>

### KickPlayerResponse







<a name="dcs-net-v0-SendChatRequest"></a>

### SendChatRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| message | [string](#string) |  | the message to send in the chat |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | which coalition? DCS only supports ALL or NEUTRAL (only applicable to send_chat) |






<a name="dcs-net-v0-SendChatResponse"></a>

### SendChatResponse







<a name="dcs-net-v0-SendChatToRequest"></a>

### SendChatToRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| message | [string](#string) |  | the message to send in the chat |
| target_player_id | [uint32](#uint32) |  | the target player of the direct message |






<a name="dcs-net-v0-SendChatToResponse"></a>

### SendChatToResponse






 

 

 


<a name="dcs-net-v0-NetService"></a>

### NetService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SendChatTo | [SendChatToRequest](#dcs-net-v0-SendChatToRequest) | [SendChatToResponse](#dcs-net-v0-SendChatToResponse) | https://wiki.hoggitworld.com/view/DCS_func_send_chat_to |
| SendChat | [SendChatRequest](#dcs-net-v0-SendChatRequest) | [SendChatResponse](#dcs-net-v0-SendChatResponse) | https://wiki.hoggitworld.com/view/DCS_func_send_chat |
| GetPlayers | [GetPlayersRequest](#dcs-net-v0-GetPlayersRequest) | [GetPlayersResponse](#dcs-net-v0-GetPlayersResponse) | returns a list of all connected players. https://wiki.hoggitworld.com/view/DCS_func_get_player_info |
| KickPlayer | [KickPlayerRequest](#dcs-net-v0-KickPlayerRequest) | [KickPlayerResponse](#dcs-net-v0-KickPlayerResponse) | Kick a specified player from the server with a message https://wiki.hoggitworld.com/view/DCS_func_kick |
| ForcePlayerSlot | [ForcePlayerSlotRequest](#dcs-net-v0-ForcePlayerSlotRequest) | [ForcePlayerSlotResponse](#dcs-net-v0-ForcePlayerSlotResponse) | Force a player into a slot / coalition. To move the player back into spectators, use the following pseudo: `ForcePlayerSlot({ player_id: ..., coalition: NEUTRAL, slot_id: &#34;&#34; })` |

 



<a name="dcs_spot_v0_spot-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/spot/v0/spot.proto



<a name="dcs-spot-v0-CreateInfraRedRequest"></a>

### CreateInfraRedRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| source_unit_name | [string](#string) |  |  |
| offset | [dcs.common.v0.Vector](#dcs-common-v0-Vector) |  |  |
| direction | [dcs.common.v0.Vector](#dcs-common-v0-Vector) |  |  |






<a name="dcs-spot-v0-CreateInfraRedResponse"></a>

### CreateInfraRedResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| spot_id | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-CreateLaserRequest"></a>

### CreateLaserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| source_unit_name | [string](#string) |  |  |
| offset | [dcs.common.v0.Vector](#dcs-common-v0-Vector) |  |  |
| direction | [dcs.common.v0.Vector](#dcs-common-v0-Vector) |  |  |
| code | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-CreateLaserResponse"></a>

### CreateLaserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| spot_id | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-DestroyRequest"></a>

### DestroyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| spot_id | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-DestroyResponse"></a>

### DestroyResponse







<a name="dcs-spot-v0-GetCategoryRequest"></a>

### GetCategoryRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| spot_id | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-GetCategoryResponse"></a>

### GetCategoryResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| category | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-GetCodeRequest"></a>

### GetCodeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| spot_id | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-GetCodeResponse"></a>

### GetCodeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| code | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-GetPointRequest"></a>

### GetPointRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| spot_id | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-GetPointResponse"></a>

### GetPointResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |






<a name="dcs-spot-v0-SetCodeRequest"></a>

### SetCodeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| spot_id | [uint32](#uint32) |  |  |
| code | [uint32](#uint32) |  |  |






<a name="dcs-spot-v0-SetCodeResponse"></a>

### SetCodeResponse







<a name="dcs-spot-v0-SetPointRequest"></a>

### SetPointRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| spot_id | [uint32](#uint32) |  |  |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |






<a name="dcs-spot-v0-SetPointResponse"></a>

### SetPointResponse






 

 

 


<a name="dcs-spot-v0-SpotService"></a>

### SpotService
https://wiki.hoggitworld.com/view/DCS_singleton_spot

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateLaser | [CreateLaserRequest](#dcs-spot-v0-CreateLaserRequest) | [CreateLaserResponse](#dcs-spot-v0-CreateLaserResponse) |  |
| CreateInfraRed | [CreateInfraRedRequest](#dcs-spot-v0-CreateInfraRedRequest) | [CreateInfraRedResponse](#dcs-spot-v0-CreateInfraRedResponse) |  |
| Destroy | [DestroyRequest](#dcs-spot-v0-DestroyRequest) | [DestroyResponse](#dcs-spot-v0-DestroyResponse) |  |
| GetPoint | [GetPointRequest](#dcs-spot-v0-GetPointRequest) | [GetPointResponse](#dcs-spot-v0-GetPointResponse) |  |
| SetPoint | [SetPointRequest](#dcs-spot-v0-SetPointRequest) | [SetPointResponse](#dcs-spot-v0-SetPointResponse) |  |
| GetCode | [GetCodeRequest](#dcs-spot-v0-GetCodeRequest) | [GetCodeResponse](#dcs-spot-v0-GetCodeResponse) |  |
| SetCode | [SetCodeRequest](#dcs-spot-v0-SetCodeRequest) | [SetCodeResponse](#dcs-spot-v0-SetCodeResponse) |  |
| GetCategory | [GetCategoryRequest](#dcs-spot-v0-GetCategoryRequest) | [GetCategoryResponse](#dcs-spot-v0-GetCategoryResponse) |  |

 



<a name="dcs_srs_v0_srs-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/srs/v0/srs.proto



<a name="dcs-srs-v0-GetClientsRequest"></a>

### GetClientsRequest







<a name="dcs-srs-v0-GetClientsResponse"></a>

### GetClientsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| clients | [GetClientsResponse.Client](#dcs-srs-v0-GetClientsResponse-Client) | repeated |  |






<a name="dcs-srs-v0-GetClientsResponse-Client"></a>

### GetClientsResponse.Client



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit | [dcs.common.v0.Unit](#dcs-common-v0-Unit) |  | The unit that is connected to SRS. |
| frequencies | [uint64](#uint64) | repeated | The radio frequencies in Hz the unit is connected to. |






<a name="dcs-srs-v0-TransmitRequest"></a>

### TransmitRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ssml | [string](#string) |  | The text that is synthesized to speech and transmitted to SRS. Supports SSML tags (you should not wrap the text in the root `&lt;speak&gt;` tag though). |
| plaintext | [string](#string) | optional | The plain text without any transformations made to it for the purpose of getting it spoken out as desired (no SSML tags, no FOUR NINER instead of 49, ...). Even though this field is optional, please consider providing it as it can be used to display the spoken text to players with hearing impairments. |
| frequency | [uint64](#uint64) |  | The radio frequency in Hz the transmission is send to. Example: 251000000 for 251.00MHz. |
| srs_client_name | [string](#string) | optional | Name of the SRS client. Defaults to &#34;DCS-gRPC&#34;. |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | The origin of the transmission. Relevant if the SRS server has &#34;Line of Sight&#34; and/or &#34;Distance Limit&#34; enabled. |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  | The coalition of the transmission. Relevant if the SRS server has &#34;Secure Coalition Radios&#34; enabled. Only Blue and Red are supported, all other values will fallback to Spectator. |
| async | [bool](#bool) |  | Whether to keep the request open until the whole transmission was sent. If enabled, you can send the next transmission after you&#39;ve received the response for the previous one and be sure that they don&#39;t overlap (talk over each other). If disabled, you&#39;ll receive a response right away (kind of fire and forget). You can use the returned duration as a spacing between TTS requests to prevent the overlap of multiple playbacks yourself. |
| aws | [TransmitRequest.Aws](#dcs-srs-v0-TransmitRequest-Aws) |  |  |
| azure | [TransmitRequest.Azure](#dcs-srs-v0-TransmitRequest-Azure) |  |  |
| gcloud | [TransmitRequest.GCloud](#dcs-srs-v0-TransmitRequest-GCloud) |  |  |
| win | [TransmitRequest.Windows](#dcs-srs-v0-TransmitRequest-Windows) |  |  |






<a name="dcs-srs-v0-TransmitRequest-Aws"></a>

### TransmitRequest.Aws



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| voice | [string](#string) | optional | The voice the text is synthesized in, see: https://docs.aws.amazon.com/polly/latest/dg/voicelist.html |






<a name="dcs-srs-v0-TransmitRequest-Azure"></a>

### TransmitRequest.Azure



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| voice | [string](#string) | optional | The voice the text is synthesized in, see: https://learn.microsoft.com/azure/cognitive-services/speech-service/language-support |






<a name="dcs-srs-v0-TransmitRequest-GCloud"></a>

### TransmitRequest.GCloud



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| voice | [string](#string) | optional | The voice the text is synthesized in, see: https://cloud.google.com/text-to-speech/docs/voices |






<a name="dcs-srs-v0-TransmitRequest-Windows"></a>

### TransmitRequest.Windows



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| voice | [string](#string) | optional | The voice the text is synthesized in. |






<a name="dcs-srs-v0-TransmitResponse"></a>

### TransmitResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| duration_ms | [uint32](#uint32) |  | The duration in milliseconds it roughly takes to speak the transmission. |





 

 

 


<a name="dcs-srs-v0-SrsService"></a>

### SrsService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Transmit | [TransmitRequest](#dcs-srs-v0-TransmitRequest) | [TransmitResponse](#dcs-srs-v0-TransmitResponse) | Synthesize text to speech and transmit it over SRS. By default, this blocks until a transmission completed (unless `async` is set to `true`). This can be used to prevent transmission to overlap each other, by not sending another transmission on the same frequency until you&#39;ve received the response from the previous transmission on that frequency. However, it does not block or prevent any other client from transmitting over the same frequency at the same time. |
| GetClients | [GetClientsRequest](#dcs-srs-v0-GetClientsRequest) | [GetClientsResponse](#dcs-srs-v0-GetClientsResponse) | Retrieve a list of units (players) and their active frequencies that are connected to SRS. |

 



<a name="dcs_timer_v0_timer-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/timer/v0/timer.proto



<a name="dcs-timer-v0-GetAbsoluteTimeRequest"></a>

### GetAbsoluteTimeRequest







<a name="dcs-timer-v0-GetAbsoluteTimeResponse"></a>

### GetAbsoluteTimeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  | The current time in seconds since 00:00 of the start date of the mission. |
| day | [uint32](#uint32) |  |  |
| month | [uint32](#uint32) |  |  |
| year | [int32](#int32) |  |  |






<a name="dcs-timer-v0-GetTimeRequest"></a>

### GetTimeRequest







<a name="dcs-timer-v0-GetTimeResponse"></a>

### GetTimeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  |  |






<a name="dcs-timer-v0-GetTimeZeroRequest"></a>

### GetTimeZeroRequest







<a name="dcs-timer-v0-GetTimeZeroResponse"></a>

### GetTimeZeroResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  | The time in seconds since 00:00. |
| day | [uint32](#uint32) |  |  |
| month | [uint32](#uint32) |  |  |
| year | [int32](#int32) |  |  |





 

 

 


<a name="dcs-timer-v0-TimerService"></a>

### TimerService
https://wiki.hoggitworld.com/view/DCS_singleton_timer

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetTime | [GetTimeRequest](#dcs-timer-v0-GetTimeRequest) | [GetTimeResponse](#dcs-timer-v0-GetTimeResponse) | https://wiki.hoggitworld.com/view/DCS_func_getTime |
| GetAbsoluteTime | [GetAbsoluteTimeRequest](#dcs-timer-v0-GetAbsoluteTimeRequest) | [GetAbsoluteTimeResponse](#dcs-timer-v0-GetAbsoluteTimeResponse) | https://wiki.hoggitworld.com/view/DCS_func_getAbsTime |
| GetTimeZero | [GetTimeZeroRequest](#dcs-timer-v0-GetTimeZeroRequest) | [GetTimeZeroResponse](#dcs-timer-v0-GetTimeZeroResponse) | https://wiki.hoggitworld.com/view/DCS_func_getTime0 |

 



<a name="dcs_trigger_v0_trigger-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/trigger/v0/trigger.proto



<a name="dcs-trigger-v0-ActivateGroupRequest"></a>

### ActivateGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  | Uses group_name, not id |






<a name="dcs-trigger-v0-ActivateGroupResponse"></a>

### ActivateGroupResponse







<a name="dcs-trigger-v0-ArrowToAllRequest"></a>

### ArrowToAllRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| start_point | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| end_point | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| border_color | [Color](#dcs-trigger-v0-Color) |  |  |
| fill_color | [Color](#dcs-trigger-v0-Color) |  |  |
| line_type | [LineType](#dcs-trigger-v0-LineType) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-ArrowToAllResponse"></a>

### ArrowToAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-CircleToAllRequest"></a>

### CircleToAllRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| center | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| radius | [double](#double) |  |  |
| border_color | [Color](#dcs-trigger-v0-Color) |  |  |
| fill_color | [Color](#dcs-trigger-v0-Color) |  |  |
| line_type | [LineType](#dcs-trigger-v0-LineType) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-CircleToAllResponse"></a>

### CircleToAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-Color"></a>

### Color
Represents an RGBA color but instead of using 0-255 as the color
values it uses 0 to 1. A red color with 50% transparency would be
RGBA of 1, 0, 0, 0.5


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| red | [double](#double) |  |  |
| green | [double](#double) |  |  |
| blue | [double](#double) |  |  |
| alpha | [double](#double) |  |  |






<a name="dcs-trigger-v0-DeactivateGroupRequest"></a>

### DeactivateGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-trigger-v0-DeactivateGroupResponse"></a>

### DeactivateGroupResponse







<a name="dcs-trigger-v0-EffectSmokeBigRequest"></a>

### EffectSmokeBigRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| preset | [EffectSmokeBigRequest.SmokePreset](#dcs-trigger-v0-EffectSmokeBigRequest-SmokePreset) |  |  |
| density | [float](#float) |  | Optional density from 0.0 to 1.0 (defaults to 1.0 if not provided or 0) |
| name | [string](#string) |  | Optional unique string ID to allow stopping the smoke later |






<a name="dcs-trigger-v0-EffectSmokeBigResponse"></a>

### EffectSmokeBigResponse







<a name="dcs-trigger-v0-EffectSmokeStopRequest"></a>

### EffectSmokeStopRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-trigger-v0-EffectSmokeStopResponse"></a>

### EffectSmokeStopResponse







<a name="dcs-trigger-v0-ExplosionRequest"></a>

### ExplosionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| power | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-ExplosionResponse"></a>

### ExplosionResponse







<a name="dcs-trigger-v0-GetUserFlagRequest"></a>

### GetUserFlagRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| flag | [string](#string) |  |  |






<a name="dcs-trigger-v0-GetUserFlagResponse"></a>

### GetUserFlagResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-GetZoneRequest"></a>

### GetZoneRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-trigger-v0-GetZoneResponse"></a>

### GetZoneResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |
| radius | [double](#double) |  |  |






<a name="dcs-trigger-v0-GroupContinueMovingRequest"></a>

### GroupContinueMovingRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-trigger-v0-GroupContinueMovingResponse"></a>

### GroupContinueMovingResponse







<a name="dcs-trigger-v0-GroupStopMovingRequest"></a>

### GroupStopMovingRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-trigger-v0-GroupStopMovingResponse"></a>

### GroupStopMovingResponse







<a name="dcs-trigger-v0-IlluminationBombRequest"></a>

### IlluminationBombRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | The altitude of Illumination Bombs is meters above ground. Ground level will be calculated server-side |
| power | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-IlluminationBombResponse"></a>

### IlluminationBombResponse







<a name="dcs-trigger-v0-LineToAllRequest"></a>

### LineToAllRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| start_point | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| end_point | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| color | [Color](#dcs-trigger-v0-Color) |  |  |
| line_type | [LineType](#dcs-trigger-v0-LineType) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-LineToAllResponse"></a>

### LineToAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-MarkToAllRequest"></a>

### MarkToAllRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [string](#string) |  |  |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-MarkToAllResponse"></a>

### MarkToAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-MarkToCoalitionRequest"></a>

### MarkToCoalitionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| text | [string](#string) |  |  |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-MarkToCoalitionResponse"></a>

### MarkToCoalitionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-MarkToGroupRequest"></a>

### MarkToGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| text | [string](#string) |  |  |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| group_id | [uint32](#uint32) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-MarkToGroupResponse"></a>

### MarkToGroupResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-MarkupToAllRequest"></a>

### MarkupToAllRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shape | [Shape](#dcs-trigger-v0-Shape) |  |  |
| points | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) | repeated |  |
| border_color | [Color](#dcs-trigger-v0-Color) |  |  |
| fill_color | [Color](#dcs-trigger-v0-Color) |  |  |
| line_type | [LineType](#dcs-trigger-v0-LineType) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-MarkupToAllResponse"></a>

### MarkupToAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-MarkupToCoalitionRequest"></a>

### MarkupToCoalitionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shape | [Shape](#dcs-trigger-v0-Shape) |  |  |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| points | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) | repeated |  |
| border_color | [Color](#dcs-trigger-v0-Color) |  |  |
| fill_color | [Color](#dcs-trigger-v0-Color) |  |  |
| line_type | [LineType](#dcs-trigger-v0-LineType) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-MarkupToCoalitionResponse"></a>

### MarkupToCoalitionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-OutTextForCoalitionRequest"></a>

### OutTextForCoalitionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [string](#string) |  |  |
| display_time | [int32](#int32) |  |  |
| clear_view | [bool](#bool) |  |  |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |






<a name="dcs-trigger-v0-OutTextForCoalitionResponse"></a>

### OutTextForCoalitionResponse







<a name="dcs-trigger-v0-OutTextForGroupRequest"></a>

### OutTextForGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [string](#string) |  |  |
| display_time | [int32](#int32) |  |  |
| clear_view | [bool](#bool) |  |  |
| group_id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-OutTextForGroupResponse"></a>

### OutTextForGroupResponse







<a name="dcs-trigger-v0-OutTextForUnitRequest"></a>

### OutTextForUnitRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [string](#string) |  |  |
| display_time | [int32](#int32) |  |  |
| clear_view | [bool](#bool) |  |  |
| unit_id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-OutTextForUnitResponse"></a>

### OutTextForUnitResponse







<a name="dcs-trigger-v0-OutTextRequest"></a>

### OutTextRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [string](#string) |  |  |
| display_time | [int32](#int32) |  |  |
| clear_view | [bool](#bool) |  |  |






<a name="dcs-trigger-v0-OutTextResponse"></a>

### OutTextResponse







<a name="dcs-trigger-v0-PushAITaskRequest"></a>

### PushAITaskRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| task_index | [int32](#int32) |  |  |






<a name="dcs-trigger-v0-PushAITaskResponse"></a>

### PushAITaskResponse







<a name="dcs-trigger-v0-QuadToAllRequest"></a>

### QuadToAllRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| p1 | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| p2 | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| p3 | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| p4 | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| border_color | [Color](#dcs-trigger-v0-Color) |  |  |
| fill_color | [Color](#dcs-trigger-v0-Color) |  |  |
| line_type | [LineType](#dcs-trigger-v0-LineType) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-QuadToAllResponse"></a>

### QuadToAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-RectToAllRequest"></a>

### RectToAllRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| start_point | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| end_point | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| border_color | [Color](#dcs-trigger-v0-Color) |  |  |
| fill_color | [Color](#dcs-trigger-v0-Color) |  |  |
| line_type | [LineType](#dcs-trigger-v0-LineType) |  |  |
| read_only | [bool](#bool) |  |  |
| message | [string](#string) |  |  |






<a name="dcs-trigger-v0-RectToAllResponse"></a>

### RectToAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-RemoveMarkRequest"></a>

### RemoveMarkRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-RemoveMarkResponse"></a>

### RemoveMarkResponse







<a name="dcs-trigger-v0-SetAITaskRequest"></a>

### SetAITaskRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |
| task_index | [int32](#int32) |  |  |






<a name="dcs-trigger-v0-SetAITaskResponse"></a>

### SetAITaskResponse







<a name="dcs-trigger-v0-SetGroupAIOffRequest"></a>

### SetGroupAIOffRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-trigger-v0-SetGroupAIOffResponse"></a>

### SetGroupAIOffResponse







<a name="dcs-trigger-v0-SetGroupAIOnRequest"></a>

### SetGroupAIOnRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group_name | [string](#string) |  |  |






<a name="dcs-trigger-v0-SetGroupAIOnResponse"></a>

### SetGroupAIOnResponse







<a name="dcs-trigger-v0-SetMarkupColorFillRequest"></a>

### SetMarkupColorFillRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| fill_color | [Color](#dcs-trigger-v0-Color) |  |  |






<a name="dcs-trigger-v0-SetMarkupColorFillResponse"></a>

### SetMarkupColorFillResponse







<a name="dcs-trigger-v0-SetMarkupColorRequest"></a>

### SetMarkupColorRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| color | [Color](#dcs-trigger-v0-Color) |  |  |






<a name="dcs-trigger-v0-SetMarkupColorResponse"></a>

### SetMarkupColorResponse







<a name="dcs-trigger-v0-SetMarkupFontSizeRequest"></a>

### SetMarkupFontSizeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| font_size | [int32](#int32) |  |  |






<a name="dcs-trigger-v0-SetMarkupFontSizeResponse"></a>

### SetMarkupFontSizeResponse







<a name="dcs-trigger-v0-SetMarkupPositionEndRequest"></a>

### SetMarkupPositionEndRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |






<a name="dcs-trigger-v0-SetMarkupPositionEndResponse"></a>

### SetMarkupPositionEndResponse







<a name="dcs-trigger-v0-SetMarkupPositionStartRequest"></a>

### SetMarkupPositionStartRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |






<a name="dcs-trigger-v0-SetMarkupPositionStartResponse"></a>

### SetMarkupPositionStartResponse







<a name="dcs-trigger-v0-SetMarkupRadiusRequest"></a>

### SetMarkupRadiusRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| radius | [double](#double) |  |  |






<a name="dcs-trigger-v0-SetMarkupRadiusResponse"></a>

### SetMarkupRadiusResponse







<a name="dcs-trigger-v0-SetMarkupTextRequest"></a>

### SetMarkupTextRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| text | [string](#string) |  |  |






<a name="dcs-trigger-v0-SetMarkupTextResponse"></a>

### SetMarkupTextResponse







<a name="dcs-trigger-v0-SetMarkupTypeLineRequest"></a>

### SetMarkupTypeLineRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| line_type | [LineType](#dcs-trigger-v0-LineType) |  |  |






<a name="dcs-trigger-v0-SetMarkupTypeLineResponse"></a>

### SetMarkupTypeLineResponse







<a name="dcs-trigger-v0-SetUnitInternalCargoRequest"></a>

### SetUnitInternalCargoRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit_name | [string](#string) |  |  |
| mass | [int32](#int32) |  |  |






<a name="dcs-trigger-v0-SetUnitInternalCargoResponse"></a>

### SetUnitInternalCargoResponse







<a name="dcs-trigger-v0-SetUserFlagRequest"></a>

### SetUserFlagRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| flag | [string](#string) |  |  |
| value | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-SetUserFlagResponse"></a>

### SetUserFlagResponse







<a name="dcs-trigger-v0-SignalFlareRequest"></a>

### SignalFlareRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | Altitude parameter will be ignored. Signal flares always fire from ground level which will be calculated server-side |
| color | [SignalFlareRequest.FlareColor](#dcs-trigger-v0-SignalFlareRequest-FlareColor) |  |  |
| azimuth | [uint32](#uint32) |  |  |






<a name="dcs-trigger-v0-SignalFlareResponse"></a>

### SignalFlareResponse







<a name="dcs-trigger-v0-SmokeRequest"></a>

### SmokeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | Altitude parameter will be ignored. Smoke always eminates from ground level which will be calculated server-side |
| color | [SmokeRequest.SmokeColor](#dcs-trigger-v0-SmokeRequest-SmokeColor) |  |  |






<a name="dcs-trigger-v0-SmokeResponse"></a>

### SmokeResponse







<a name="dcs-trigger-v0-TextToAllRequest"></a>

### TextToAllRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |
| start_point | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  |  |
| border_color | [Color](#dcs-trigger-v0-Color) |  |  |
| fill_color | [Color](#dcs-trigger-v0-Color) |  |  |
| font_size | [int32](#int32) |  |  |
| read_only | [bool](#bool) |  |  |
| text | [string](#string) |  |  |






<a name="dcs-trigger-v0-TextToAllResponse"></a>

### TextToAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |





 


<a name="dcs-trigger-v0-EffectSmokeBigRequest-SmokePreset"></a>

### EffectSmokeBigRequest.SmokePreset


| Name | Number | Description |
| ---- | ------ | ----------- |
| SMOKE_PRESET_UNSPECIFIED | 0 |  |
| SMOKE_PRESET_SMALL_SMOKE_AND_FIRE | 1 |  |
| SMOKE_PRESET_MEDIUM_SMOKE_AND_FIRE | 2 |  |
| SMOKE_PRESET_LARGE_SMOKE_AND_FIRE | 3 |  |
| SMOKE_PRESET_HUGE_SMOKE_AND_FIRE | 4 |  |
| SMOKE_PRESET_SMALL_SMOKE | 5 |  |
| SMOKE_PRESET_MEDIUM_SMOKE | 6 |  |
| SMOKE_PRESET_LARGE_SMOKE | 7 |  |
| SMOKE_PRESET_HUGE_SMOKE | 8 |  |



<a name="dcs-trigger-v0-LineType"></a>

### LineType


| Name | Number | Description |
| ---- | ------ | ----------- |
| LINE_TYPE_NO_LINE | 0 | protolint:disable:next ENUM_FIELD_NAMES_ZERO_VALUE_END_WITH |
| LINE_TYPE_SOLID | 1 |  |
| LINE_TYPE_DASHED | 2 |  |
| LINE_TYPE_DOTTED | 3 |  |
| LINE_TYPE_DOT_DASH | 4 |  |
| LINE_TYPE_LONG_DASH | 5 |  |
| LINE_TYPE_TWO_DASH | 6 |  |



<a name="dcs-trigger-v0-Shape"></a>

### Shape


| Name | Number | Description |
| ---- | ------ | ----------- |
| SHAPE_UNSPECIFIED | 0 |  |
| SHAPE_LINE | 1 |  |
| SHAPE_CIRCLE | 2 |  |
| SHAPE_RECT | 3 |  |
| SHAPE_ARROW | 4 |  |
| SHAPE_TEXT | 5 |  |
| SHAPE_QUAD | 6 |  |
| SHAPE_FREEFORM | 7 |  |



<a name="dcs-trigger-v0-SignalFlareRequest-FlareColor"></a>

### SignalFlareRequest.FlareColor


| Name | Number | Description |
| ---- | ------ | ----------- |
| FLARE_COLOR_UNSPECIFIED | 0 |  |
| FLARE_COLOR_GREEN | 1 |  |
| FLARE_COLOR_RED | 2 |  |
| FLARE_COLOR_WHITE | 3 |  |
| FLARE_COLOR_YELLOW | 4 |  |



<a name="dcs-trigger-v0-SmokeRequest-SmokeColor"></a>

### SmokeRequest.SmokeColor


| Name | Number | Description |
| ---- | ------ | ----------- |
| SMOKE_COLOR_UNSPECIFIED | 0 |  |
| SMOKE_COLOR_GREEN | 1 |  |
| SMOKE_COLOR_RED | 2 |  |
| SMOKE_COLOR_WHITE | 3 |  |
| SMOKE_COLOR_ORANGE | 4 |  |
| SMOKE_COLOR_BLUE | 5 |  |


 

 


<a name="dcs-trigger-v0-TriggerService"></a>

### TriggerService
https://wiki.hoggitworld.com/view/DCS_singleton_trigger

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| OutText | [OutTextRequest](#dcs-trigger-v0-OutTextRequest) | [OutTextResponse](#dcs-trigger-v0-OutTextResponse) | https://wiki.hoggitworld.com/view/DCS_func_outText |
| OutTextForCoalition | [OutTextForCoalitionRequest](#dcs-trigger-v0-OutTextForCoalitionRequest) | [OutTextForCoalitionResponse](#dcs-trigger-v0-OutTextForCoalitionResponse) | https://wiki.hoggitworld.com/view/DCS_func_outTextForCoalition |
| OutTextForGroup | [OutTextForGroupRequest](#dcs-trigger-v0-OutTextForGroupRequest) | [OutTextForGroupResponse](#dcs-trigger-v0-OutTextForGroupResponse) | https://wiki.hoggitworld.com/view/DCS_func_outTextForGroup |
| OutTextForUnit | [OutTextForUnitRequest](#dcs-trigger-v0-OutTextForUnitRequest) | [OutTextForUnitResponse](#dcs-trigger-v0-OutTextForUnitResponse) | https://wiki.hoggitworld.com/view/DCS_func_outTextForUnit |
| GetUserFlag | [GetUserFlagRequest](#dcs-trigger-v0-GetUserFlagRequest) | [GetUserFlagResponse](#dcs-trigger-v0-GetUserFlagResponse) | https://wiki.hoggitworld.com/view/DCS_func_getUserFlag |
| SetUserFlag | [SetUserFlagRequest](#dcs-trigger-v0-SetUserFlagRequest) | [SetUserFlagResponse](#dcs-trigger-v0-SetUserFlagResponse) | https://wiki.hoggitworld.com/view/DCS_func_setUserFlag |
| MarkToAll | [MarkToAllRequest](#dcs-trigger-v0-MarkToAllRequest) | [MarkToAllResponse](#dcs-trigger-v0-MarkToAllResponse) | https://wiki.hoggitworld.com/view/DCS_func_markToAll |
| MarkToCoalition | [MarkToCoalitionRequest](#dcs-trigger-v0-MarkToCoalitionRequest) | [MarkToCoalitionResponse](#dcs-trigger-v0-MarkToCoalitionResponse) | https://wiki.hoggitworld.com/view/DCS_func_markToCoalition |
| MarkToGroup | [MarkToGroupRequest](#dcs-trigger-v0-MarkToGroupRequest) | [MarkToGroupResponse](#dcs-trigger-v0-MarkToGroupResponse) | https://wiki.hoggitworld.com/view/DCS_func_markToGroup |
| MarkupToAll | [MarkupToAllRequest](#dcs-trigger-v0-MarkupToAllRequest) | [MarkupToAllResponse](#dcs-trigger-v0-MarkupToAllResponse) | https://wiki.hoggitworld.com/view/DCS_func_markupToAll |
| MarkupToCoalition | [MarkupToCoalitionRequest](#dcs-trigger-v0-MarkupToCoalitionRequest) | [MarkupToCoalitionResponse](#dcs-trigger-v0-MarkupToCoalitionResponse) | Uses markupToAll under the hood but enforces a coalition to be specified https://wiki.hoggitworld.com/view/DCS_func_markupToAll |
| RemoveMark | [RemoveMarkRequest](#dcs-trigger-v0-RemoveMarkRequest) | [RemoveMarkResponse](#dcs-trigger-v0-RemoveMarkResponse) | https://wiki.hoggitworld.com/view/DCS_func_removeMark |
| Explosion | [ExplosionRequest](#dcs-trigger-v0-ExplosionRequest) | [ExplosionResponse](#dcs-trigger-v0-ExplosionResponse) | https://wiki.hoggitworld.com/view/DCS_func_explosion |
| Smoke | [SmokeRequest](#dcs-trigger-v0-SmokeRequest) | [SmokeResponse](#dcs-trigger-v0-SmokeResponse) | https://wiki.hoggitworld.com/view/DCS_func_smoke |
| IlluminationBomb | [IlluminationBombRequest](#dcs-trigger-v0-IlluminationBombRequest) | [IlluminationBombResponse](#dcs-trigger-v0-IlluminationBombResponse) | https://wiki.hoggitworld.com/view/DCS_func_illuminationBomb |
| SignalFlare | [SignalFlareRequest](#dcs-trigger-v0-SignalFlareRequest) | [SignalFlareResponse](#dcs-trigger-v0-SignalFlareResponse) | https://wiki.hoggitworld.com/view/DCS_func_signalFlare |
| GetZone | [GetZoneRequest](#dcs-trigger-v0-GetZoneRequest) | [GetZoneResponse](#dcs-trigger-v0-GetZoneResponse) | https://wiki.hoggitworld.com/view/DCS_func_getZone |
| EffectSmokeBig | [EffectSmokeBigRequest](#dcs-trigger-v0-EffectSmokeBigRequest) | [EffectSmokeBigResponse](#dcs-trigger-v0-EffectSmokeBigResponse) |  |
| EffectSmokeStop | [EffectSmokeStopRequest](#dcs-trigger-v0-EffectSmokeStopRequest) | [EffectSmokeStopResponse](#dcs-trigger-v0-EffectSmokeStopResponse) |  |
| SetUnitInternalCargo | [SetUnitInternalCargoRequest](#dcs-trigger-v0-SetUnitInternalCargoRequest) | [SetUnitInternalCargoResponse](#dcs-trigger-v0-SetUnitInternalCargoResponse) |  |
| ActivateGroup | [ActivateGroupRequest](#dcs-trigger-v0-ActivateGroupRequest) | [ActivateGroupResponse](#dcs-trigger-v0-ActivateGroupResponse) |  |
| DeactivateGroup | [DeactivateGroupRequest](#dcs-trigger-v0-DeactivateGroupRequest) | [DeactivateGroupResponse](#dcs-trigger-v0-DeactivateGroupResponse) |  |
| SetGroupAIOn | [SetGroupAIOnRequest](#dcs-trigger-v0-SetGroupAIOnRequest) | [SetGroupAIOnResponse](#dcs-trigger-v0-SetGroupAIOnResponse) |  |
| SetGroupAIOff | [SetGroupAIOffRequest](#dcs-trigger-v0-SetGroupAIOffRequest) | [SetGroupAIOffResponse](#dcs-trigger-v0-SetGroupAIOffResponse) |  |
| GroupStopMoving | [GroupStopMovingRequest](#dcs-trigger-v0-GroupStopMovingRequest) | [GroupStopMovingResponse](#dcs-trigger-v0-GroupStopMovingResponse) |  |
| GroupContinueMoving | [GroupContinueMovingRequest](#dcs-trigger-v0-GroupContinueMovingRequest) | [GroupContinueMovingResponse](#dcs-trigger-v0-GroupContinueMovingResponse) |  |
| SetAITask | [SetAITaskRequest](#dcs-trigger-v0-SetAITaskRequest) | [SetAITaskResponse](#dcs-trigger-v0-SetAITaskResponse) |  |
| PushAITask | [PushAITaskRequest](#dcs-trigger-v0-PushAITaskRequest) | [PushAITaskResponse](#dcs-trigger-v0-PushAITaskResponse) |  |
| SetMarkupRadius | [SetMarkupRadiusRequest](#dcs-trigger-v0-SetMarkupRadiusRequest) | [SetMarkupRadiusResponse](#dcs-trigger-v0-SetMarkupRadiusResponse) |  |
| SetMarkupText | [SetMarkupTextRequest](#dcs-trigger-v0-SetMarkupTextRequest) | [SetMarkupTextResponse](#dcs-trigger-v0-SetMarkupTextResponse) |  |
| SetMarkupFontSize | [SetMarkupFontSizeRequest](#dcs-trigger-v0-SetMarkupFontSizeRequest) | [SetMarkupFontSizeResponse](#dcs-trigger-v0-SetMarkupFontSizeResponse) |  |
| SetMarkupColor | [SetMarkupColorRequest](#dcs-trigger-v0-SetMarkupColorRequest) | [SetMarkupColorResponse](#dcs-trigger-v0-SetMarkupColorResponse) |  |
| SetMarkupColorFill | [SetMarkupColorFillRequest](#dcs-trigger-v0-SetMarkupColorFillRequest) | [SetMarkupColorFillResponse](#dcs-trigger-v0-SetMarkupColorFillResponse) |  |
| SetMarkupTypeLine | [SetMarkupTypeLineRequest](#dcs-trigger-v0-SetMarkupTypeLineRequest) | [SetMarkupTypeLineResponse](#dcs-trigger-v0-SetMarkupTypeLineResponse) |  |
| SetMarkupPositionEnd | [SetMarkupPositionEndRequest](#dcs-trigger-v0-SetMarkupPositionEndRequest) | [SetMarkupPositionEndResponse](#dcs-trigger-v0-SetMarkupPositionEndResponse) |  |
| SetMarkupPositionStart | [SetMarkupPositionStartRequest](#dcs-trigger-v0-SetMarkupPositionStartRequest) | [SetMarkupPositionStartResponse](#dcs-trigger-v0-SetMarkupPositionStartResponse) |  |
| LineToAll | [LineToAllRequest](#dcs-trigger-v0-LineToAllRequest) | [LineToAllResponse](#dcs-trigger-v0-LineToAllResponse) |  |
| CircleToAll | [CircleToAllRequest](#dcs-trigger-v0-CircleToAllRequest) | [CircleToAllResponse](#dcs-trigger-v0-CircleToAllResponse) |  |
| RectToAll | [RectToAllRequest](#dcs-trigger-v0-RectToAllRequest) | [RectToAllResponse](#dcs-trigger-v0-RectToAllResponse) |  |
| QuadToAll | [QuadToAllRequest](#dcs-trigger-v0-QuadToAllRequest) | [QuadToAllResponse](#dcs-trigger-v0-QuadToAllResponse) |  |
| TextToAll | [TextToAllRequest](#dcs-trigger-v0-TextToAllRequest) | [TextToAllResponse](#dcs-trigger-v0-TextToAllResponse) |  |
| ArrowToAll | [ArrowToAllRequest](#dcs-trigger-v0-ArrowToAllRequest) | [ArrowToAllResponse](#dcs-trigger-v0-ArrowToAllResponse) |  |

 



<a name="dcs_unit_v0_unit-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/unit/v0/unit.proto



<a name="dcs-unit-v0-AmmoItem"></a>

### AmmoItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| count | [uint32](#uint32) |  |  |
| type_name | [string](#string) |  |  |
| display_name | [string](#string) |  |  |
| category | [uint32](#uint32) |  |  |
| missile_category | [uint32](#uint32) | optional |  |
| guidance | [uint32](#uint32) | optional |  |






<a name="dcs-unit-v0-DestroyRequest"></a>

### DestroyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-DestroyResponse"></a>

### DestroyResponse







<a name="dcs-unit-v0-DetectionDistanceAir"></a>

### DetectionDistanceAir



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| upper_hemisphere | [Hemisphere](#dcs-unit-v0-Hemisphere) |  |  |
| lower_hemisphere | [Hemisphere](#dcs-unit-v0-Hemisphere) |  |  |






<a name="dcs-unit-v0-GetAmmoRequest"></a>

### GetAmmoRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetAmmoResponse"></a>

### GetAmmoResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ammo | [AmmoItem](#dcs-unit-v0-AmmoItem) | repeated |  |






<a name="dcs-unit-v0-GetCountryRequest"></a>

### GetCountryRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetCountryResponse"></a>

### GetCountryResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| country | [dcs.common.v0.Country](#dcs-common-v0-Country) |  |  |






<a name="dcs-unit-v0-GetDescByNameRequest"></a>

### GetDescByNameRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type_name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetDescByNameResponse"></a>

### GetDescByNameResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| desc_json | [string](#string) |  |  |






<a name="dcs-unit-v0-GetDescentCapacityRequest"></a>

### GetDescentCapacityRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetDescentCapacityResponse"></a>

### GetDescentCapacityResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| capacity | [uint32](#uint32) |  |  |






<a name="dcs-unit-v0-GetDescriptorRequest"></a>

### GetDescriptorRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetDescriptorResponse"></a>

### GetDescriptorResponse
TODO fill these in as and when we need em


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| attributes | [string](#string) | repeated |  |






<a name="dcs-unit-v0-GetDrawArgumentValueRequest"></a>

### GetDrawArgumentValueRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| argument | [uint32](#uint32) |  |  |






<a name="dcs-unit-v0-GetDrawArgumentValueResponse"></a>

### GetDrawArgumentValueResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [double](#double) |  |  |






<a name="dcs-unit-v0-GetFuelRequest"></a>

### GetFuelRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetFuelResponse"></a>

### GetFuelResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fuel | [float](#float) |  |  |






<a name="dcs-unit-v0-GetGroupRequest"></a>

### GetGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetGroupResponse"></a>

### GetGroupResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| group | [dcs.common.v0.Group](#dcs-common-v0-Group) |  |  |






<a name="dcs-unit-v0-GetLife0Request"></a>

### GetLife0Request



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetLife0Response"></a>

### GetLife0Response



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| life0 | [int32](#int32) |  |  |






<a name="dcs-unit-v0-GetLifeRequest"></a>

### GetLifeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetLifeResponse"></a>

### GetLifeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| life | [float](#float) |  |  |
| life0 | [float](#float) |  |  |






<a name="dcs-unit-v0-GetNearestCargosRequest"></a>

### GetNearestCargosRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetNearestCargosResponse"></a>

### GetNearestCargosResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cargos | [dcs.common.v0.Cargo](#dcs-common-v0-Cargo) | repeated |  |






<a name="dcs-unit-v0-GetNumberRequest"></a>

### GetNumberRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetNumberResponse"></a>

### GetNumberResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| number | [uint32](#uint32) |  |  |






<a name="dcs-unit-v0-GetPlayerNameRequest"></a>

### GetPlayerNameRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetPlayerNameResponse"></a>

### GetPlayerNameResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| player_name | [string](#string) | optional |  |






<a name="dcs-unit-v0-GetPositionRequest"></a>

### GetPositionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetPositionResponse"></a>

### GetPositionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |






<a name="dcs-unit-v0-GetRadarRequest"></a>

### GetRadarRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetRadarResponse"></a>

### GetRadarResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| active | [bool](#bool) |  |  |
| target | [dcs.common.v0.Target](#dcs-common-v0-Target) |  |  |






<a name="dcs-unit-v0-GetRequest"></a>

### GetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetResponse"></a>

### GetResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unit | [dcs.common.v0.Unit](#dcs-common-v0-Unit) |  |  |






<a name="dcs-unit-v0-GetSensorsRequest"></a>

### GetSensorsRequest
Sensors


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetSensorsResponse"></a>

### GetSensorsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sensors | [SensorCategory](#dcs-unit-v0-SensorCategory) | repeated |  |






<a name="dcs-unit-v0-GetTransformRequest"></a>

### GetTransformRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-GetTransformResponse"></a>

### GetTransformResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  | Time in seconds since the scenario started. |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  | The position of the unit |
| orientation | [dcs.common.v0.Orientation](#dcs-common-v0-Orientation) |  | The orientation of the unit in both 2D and 3D space |
| velocity | [dcs.common.v0.Velocity](#dcs-common-v0-Velocity) |  | The velocity of the unit in both 2D and 3D space |






<a name="dcs-unit-v0-HasSensorsRequest"></a>

### HasSensorsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-HasSensorsResponse"></a>

### HasSensorsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| has_sensors | [bool](#bool) |  |  |






<a name="dcs-unit-v0-Hemisphere"></a>

### Hemisphere



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tail_on | [double](#double) |  |  |
| head_on | [double](#double) |  |  |






<a name="dcs-unit-v0-InAirRequest"></a>

### InAirRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-InAirResponse"></a>

### InAirResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| in_air | [bool](#bool) |  |  |






<a name="dcs-unit-v0-IrstSensor"></a>

### IrstSensor



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| detection_distance_idle | [double](#double) |  |  |
| detection_distance_afterburner | [double](#double) |  |  |
| detection_distance_maximal | [double](#double) |  |  |






<a name="dcs-unit-v0-IsActiveRequest"></a>

### IsActiveRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-unit-v0-IsActiveResponse"></a>

### IsActiveResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_active | [bool](#bool) |  |  |






<a name="dcs-unit-v0-OpticalSensor"></a>

### OpticalSensor







<a name="dcs-unit-v0-RadarSensor"></a>

### RadarSensor



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| detection_distance_air | [DetectionDistanceAir](#dcs-unit-v0-DetectionDistanceAir) |  |  |






<a name="dcs-unit-v0-RwrSensor"></a>

### RwrSensor







<a name="dcs-unit-v0-Sensor"></a>

### Sensor



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [uint32](#uint32) |  |  |
| type_name | [string](#string) |  |  |
| radar | [RadarSensor](#dcs-unit-v0-RadarSensor) |  |  |
| irst | [IrstSensor](#dcs-unit-v0-IrstSensor) |  |  |
| rwr | [RwrSensor](#dcs-unit-v0-RwrSensor) |  |  |
| optical | [OpticalSensor](#dcs-unit-v0-OpticalSensor) |  |  |






<a name="dcs-unit-v0-SensorCategory"></a>

### SensorCategory



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| category | [uint32](#uint32) |  | DCS’ outer sensor array index (1..n) |
| sensors | [Sensor](#dcs-unit-v0-Sensor) | repeated |  |






<a name="dcs-unit-v0-SetEmissionRequest"></a>

### SetEmissionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| emitting | [bool](#bool) |  |  |






<a name="dcs-unit-v0-SetEmissionResponse"></a>

### SetEmissionResponse






 

 

 


<a name="dcs-unit-v0-UnitService"></a>

### UnitService
https://wiki.hoggitworld.com/view/DCS_Class_Unit

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetRadar | [GetRadarRequest](#dcs-unit-v0-GetRadarRequest) | [GetRadarResponse](#dcs-unit-v0-GetRadarResponse) | https://wiki.hoggitworld.com/view/DCS_func_getRadar |
| GetPosition | [GetPositionRequest](#dcs-unit-v0-GetPositionRequest) | [GetPositionResponse](#dcs-unit-v0-GetPositionResponse) | https://wiki.hoggitworld.com/view/DCS_func_getPoint |
| GetPlayerName | [GetPlayerNameRequest](#dcs-unit-v0-GetPlayerNameRequest) | [GetPlayerNameResponse](#dcs-unit-v0-GetPlayerNameResponse) | https://wiki.hoggitworld.com/view/DCS_func_getPlayerName |
| GetDescriptor | [GetDescriptorRequest](#dcs-unit-v0-GetDescriptorRequest) | [GetDescriptorResponse](#dcs-unit-v0-GetDescriptorResponse) |  |
| SetEmission | [SetEmissionRequest](#dcs-unit-v0-SetEmissionRequest) | [SetEmissionResponse](#dcs-unit-v0-SetEmissionResponse) | https://wiki.hoggitworld.com/view/DCS_func_enableEmission |
| Get | [GetRequest](#dcs-unit-v0-GetRequest) | [GetResponse](#dcs-unit-v0-GetResponse) | https://wiki.hoggitworld.com/view/DCS_func_getByName |
| GetTransform | [GetTransformRequest](#dcs-unit-v0-GetTransformRequest) | [GetTransformResponse](#dcs-unit-v0-GetTransformResponse) | Get information about the unit in 3D space, including its position, orientation and velocity. |
| Destroy | [DestroyRequest](#dcs-unit-v0-DestroyRequest) | [DestroyResponse](#dcs-unit-v0-DestroyResponse) | https://wiki.hoggitworld.com/view/DCS_func_destroy |
| GetDrawArgumentValue | [GetDrawArgumentValueRequest](#dcs-unit-v0-GetDrawArgumentValueRequest) | [GetDrawArgumentValueResponse](#dcs-unit-v0-GetDrawArgumentValueResponse) | https://wiki.hoggitworld.com/view/DCS_func_getDrawArgumentValue |
| GetSensors | [GetSensorsRequest](#dcs-unit-v0-GetSensorsRequest) | [GetSensorsResponse](#dcs-unit-v0-GetSensorsResponse) | https://wiki.hoggitworld.com/view/DCS_func_getSensors |
| GetLife | [GetLifeRequest](#dcs-unit-v0-GetLifeRequest) | [GetLifeResponse](#dcs-unit-v0-GetLifeResponse) | https://wiki.hoggitworld.com/view/DCS_func_getLife |
| GetFuel | [GetFuelRequest](#dcs-unit-v0-GetFuelRequest) | [GetFuelResponse](#dcs-unit-v0-GetFuelResponse) | https://wiki.hoggitworld.com/view/DCS_func_getFuel |
| GetAmmo | [GetAmmoRequest](#dcs-unit-v0-GetAmmoRequest) | [GetAmmoResponse](#dcs-unit-v0-GetAmmoResponse) | https://wiki.hoggitworld.com/view/DCS_func_getAmmo |
| InAir | [InAirRequest](#dcs-unit-v0-InAirRequest) | [InAirResponse](#dcs-unit-v0-InAirResponse) | https://wiki.hoggitworld.com/view/DCS_func_inAir |
| IsActive | [IsActiveRequest](#dcs-unit-v0-IsActiveRequest) | [IsActiveResponse](#dcs-unit-v0-IsActiveResponse) | https://wiki.hoggitworld.com/view/DCS_func_isActive |
| GetCountry | [GetCountryRequest](#dcs-unit-v0-GetCountryRequest) | [GetCountryResponse](#dcs-unit-v0-GetCountryResponse) | https://wiki.hoggitworld.com/view/DCS_func_getCountry |
| GetNumber | [GetNumberRequest](#dcs-unit-v0-GetNumberRequest) | [GetNumberResponse](#dcs-unit-v0-GetNumberResponse) |  |
| GetGroup | [GetGroupRequest](#dcs-unit-v0-GetGroupRequest) | [GetGroupResponse](#dcs-unit-v0-GetGroupResponse) |  |
| GetLife0 | [GetLife0Request](#dcs-unit-v0-GetLife0Request) | [GetLife0Response](#dcs-unit-v0-GetLife0Response) |  |
| HasSensors | [HasSensorsRequest](#dcs-unit-v0-HasSensorsRequest) | [HasSensorsResponse](#dcs-unit-v0-HasSensorsResponse) |  |
| GetNearestCargos | [GetNearestCargosRequest](#dcs-unit-v0-GetNearestCargosRequest) | [GetNearestCargosResponse](#dcs-unit-v0-GetNearestCargosResponse) |  |
| GetDescentCapacity | [GetDescentCapacityRequest](#dcs-unit-v0-GetDescentCapacityRequest) | [GetDescentCapacityResponse](#dcs-unit-v0-GetDescentCapacityResponse) |  |
| GetDescByName | [GetDescByNameRequest](#dcs-unit-v0-GetDescByNameRequest) | [GetDescByNameResponse](#dcs-unit-v0-GetDescByNameResponse) |  |

 



<a name="dcs_warehouse_v0_warehouse-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/warehouse/v0/warehouse.proto



<a name="dcs-warehouse-v0-AddItemRequest"></a>

### AddItemRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |
| item_name | [string](#string) |  |  |
| count | [int32](#int32) |  |  |






<a name="dcs-warehouse-v0-AddItemResponse"></a>

### AddItemResponse







<a name="dcs-warehouse-v0-AddLiquidRequest"></a>

### AddLiquidRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |
| liquid_type | [int32](#int32) |  |  |
| amount | [double](#double) |  |  |






<a name="dcs-warehouse-v0-AddLiquidResponse"></a>

### AddLiquidResponse







<a name="dcs-warehouse-v0-GetInventoryRequest"></a>

### GetInventoryRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |






<a name="dcs-warehouse-v0-GetInventoryResponse"></a>

### GetInventoryResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| inventory_json | [string](#string) |  |  |






<a name="dcs-warehouse-v0-GetItemCountRequest"></a>

### GetItemCountRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |
| item_name | [string](#string) |  |  |






<a name="dcs-warehouse-v0-GetItemCountResponse"></a>

### GetItemCountResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| count | [int32](#int32) |  |  |






<a name="dcs-warehouse-v0-GetLiquidAmountRequest"></a>

### GetLiquidAmountRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |
| liquid_type | [int32](#int32) |  |  |






<a name="dcs-warehouse-v0-GetLiquidAmountResponse"></a>

### GetLiquidAmountResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [double](#double) |  |  |






<a name="dcs-warehouse-v0-GetOwnerRequest"></a>

### GetOwnerRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |






<a name="dcs-warehouse-v0-GetOwnerResponse"></a>

### GetOwnerResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |






<a name="dcs-warehouse-v0-RemoveItemRequest"></a>

### RemoveItemRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |
| item_name | [string](#string) |  |  |
| count | [int32](#int32) |  |  |






<a name="dcs-warehouse-v0-RemoveItemResponse"></a>

### RemoveItemResponse







<a name="dcs-warehouse-v0-SetItemRequest"></a>

### SetItemRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |
| item_name | [string](#string) |  |  |
| count | [int32](#int32) |  |  |






<a name="dcs-warehouse-v0-SetItemResponse"></a>

### SetItemResponse







<a name="dcs-warehouse-v0-SetLiquidAmountRequest"></a>

### SetLiquidAmountRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbase_name | [string](#string) |  |  |
| static_name | [string](#string) |  |  |
| liquid_type | [int32](#int32) |  |  |
| amount | [double](#double) |  |  |






<a name="dcs-warehouse-v0-SetLiquidAmountResponse"></a>

### SetLiquidAmountResponse






 

 

 


<a name="dcs-warehouse-v0-WarehouseService"></a>

### WarehouseService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetInventory | [GetInventoryRequest](#dcs-warehouse-v0-GetInventoryRequest) | [GetInventoryResponse](#dcs-warehouse-v0-GetInventoryResponse) |  |
| GetItemCount | [GetItemCountRequest](#dcs-warehouse-v0-GetItemCountRequest) | [GetItemCountResponse](#dcs-warehouse-v0-GetItemCountResponse) |  |
| AddItem | [AddItemRequest](#dcs-warehouse-v0-AddItemRequest) | [AddItemResponse](#dcs-warehouse-v0-AddItemResponse) |  |
| RemoveItem | [RemoveItemRequest](#dcs-warehouse-v0-RemoveItemRequest) | [RemoveItemResponse](#dcs-warehouse-v0-RemoveItemResponse) |  |
| SetItem | [SetItemRequest](#dcs-warehouse-v0-SetItemRequest) | [SetItemResponse](#dcs-warehouse-v0-SetItemResponse) |  |
| GetLiquidAmount | [GetLiquidAmountRequest](#dcs-warehouse-v0-GetLiquidAmountRequest) | [GetLiquidAmountResponse](#dcs-warehouse-v0-GetLiquidAmountResponse) |  |
| AddLiquid | [AddLiquidRequest](#dcs-warehouse-v0-AddLiquidRequest) | [AddLiquidResponse](#dcs-warehouse-v0-AddLiquidResponse) |  |
| SetLiquidAmount | [SetLiquidAmountRequest](#dcs-warehouse-v0-SetLiquidAmountRequest) | [SetLiquidAmountResponse](#dcs-warehouse-v0-SetLiquidAmountResponse) |  |
| GetOwner | [GetOwnerRequest](#dcs-warehouse-v0-GetOwnerRequest) | [GetOwnerResponse](#dcs-warehouse-v0-GetOwnerResponse) |  |

 



<a name="dcs_weapon_v0_weapon-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/weapon/v0/weapon.proto



<a name="dcs-weapon-v0-DestroyRequest"></a>

### DestroyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-DestroyResponse"></a>

### DestroyResponse







<a name="dcs-weapon-v0-GetCategoryRequest"></a>

### GetCategoryRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetCategoryResponse"></a>

### GetCategoryResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| category | [dcs.common.v0.ObjectCategory](#dcs-common-v0-ObjectCategory) |  |  |






<a name="dcs-weapon-v0-GetCoalitionRequest"></a>

### GetCoalitionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetCoalitionResponse"></a>

### GetCoalitionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |






<a name="dcs-weapon-v0-GetCountryRequest"></a>

### GetCountryRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetCountryResponse"></a>

### GetCountryResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| country | [dcs.common.v0.Country](#dcs-common-v0-Country) |  |  |






<a name="dcs-weapon-v0-GetDescRequest"></a>

### GetDescRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetDescResponse"></a>

### GetDescResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| desc_json | [string](#string) |  | Serialized json of the weapon description table |






<a name="dcs-weapon-v0-GetLauncherRequest"></a>

### GetLauncherRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetLauncherResponse"></a>

### GetLauncherResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| launcher_name | [string](#string) |  | Name of the unit that launched it |






<a name="dcs-weapon-v0-GetNameRequest"></a>

### GetNameRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetNameResponse"></a>

### GetNameResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetPointRequest"></a>

### GetPointRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetPointResponse"></a>

### GetPointResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| point | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |






<a name="dcs-weapon-v0-GetPositionRequest"></a>

### GetPositionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetPositionResponse"></a>

### GetPositionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [dcs.common.v0.Orientation](#dcs-common-v0-Orientation) |  |  |






<a name="dcs-weapon-v0-GetTargetRequest"></a>

### GetTargetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetTargetResponse"></a>

### GetTargetResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| target_name | [string](#string) |  | Name of the unit targeted (if any) |






<a name="dcs-weapon-v0-GetTypeNameRequest"></a>

### GetTypeNameRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetTypeNameResponse"></a>

### GetTypeNameResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type_name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetVelocityRequest"></a>

### GetVelocityRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-GetVelocityResponse"></a>

### GetVelocityResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| velocity | [dcs.common.v0.Velocity](#dcs-common-v0-Velocity) |  |  |






<a name="dcs-weapon-v0-InAirRequest"></a>

### InAirRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-InAirResponse"></a>

### InAirResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| in_air | [bool](#bool) |  |  |






<a name="dcs-weapon-v0-IsExistRequest"></a>

### IsExistRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-weapon-v0-IsExistResponse"></a>

### IsExistResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_exist | [bool](#bool) |  |  |





 

 

 


<a name="dcs-weapon-v0-WeaponService"></a>

### WeaponService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetLauncher | [GetLauncherRequest](#dcs-weapon-v0-GetLauncherRequest) | [GetLauncherResponse](#dcs-weapon-v0-GetLauncherResponse) |  |
| GetTarget | [GetTargetRequest](#dcs-weapon-v0-GetTargetRequest) | [GetTargetResponse](#dcs-weapon-v0-GetTargetResponse) |  |
| GetCategory | [GetCategoryRequest](#dcs-weapon-v0-GetCategoryRequest) | [GetCategoryResponse](#dcs-weapon-v0-GetCategoryResponse) |  |
| GetDesc | [GetDescRequest](#dcs-weapon-v0-GetDescRequest) | [GetDescResponse](#dcs-weapon-v0-GetDescResponse) |  |
| GetPosition | [GetPositionRequest](#dcs-weapon-v0-GetPositionRequest) | [GetPositionResponse](#dcs-weapon-v0-GetPositionResponse) |  |
| GetVelocity | [GetVelocityRequest](#dcs-weapon-v0-GetVelocityRequest) | [GetVelocityResponse](#dcs-weapon-v0-GetVelocityResponse) |  |
| InAir | [InAirRequest](#dcs-weapon-v0-InAirRequest) | [InAirResponse](#dcs-weapon-v0-InAirResponse) |  |
| IsExist | [IsExistRequest](#dcs-weapon-v0-IsExistRequest) | [IsExistResponse](#dcs-weapon-v0-IsExistResponse) |  |
| Destroy | [DestroyRequest](#dcs-weapon-v0-DestroyRequest) | [DestroyResponse](#dcs-weapon-v0-DestroyResponse) |  |
| GetCoalition | [GetCoalitionRequest](#dcs-weapon-v0-GetCoalitionRequest) | [GetCoalitionResponse](#dcs-weapon-v0-GetCoalitionResponse) |  |
| GetCountry | [GetCountryRequest](#dcs-weapon-v0-GetCountryRequest) | [GetCountryResponse](#dcs-weapon-v0-GetCountryResponse) |  |
| GetName | [GetNameRequest](#dcs-weapon-v0-GetNameRequest) | [GetNameResponse](#dcs-weapon-v0-GetNameResponse) |  |
| GetTypeName | [GetTypeNameRequest](#dcs-weapon-v0-GetTypeNameRequest) | [GetTypeNameResponse](#dcs-weapon-v0-GetTypeNameResponse) |  |
| GetPoint | [GetPointRequest](#dcs-weapon-v0-GetPointRequest) | [GetPointResponse](#dcs-weapon-v0-GetPointResponse) |  |

 



<a name="dcs_world_v0_world-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## dcs/world/v0/world.proto



<a name="dcs-world-v0-AirbaseParking"></a>

### AirbaseParking



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| term_index | [uint32](#uint32) |  |  |
| term_type | [uint32](#uint32) |  |  |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |
| distance_to_runway | [double](#double) |  |  |
| to_ac | [bool](#bool) |  |  |






<a name="dcs-world-v0-AirbaseRunway"></a>

### AirbaseRunway



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| course | [double](#double) |  |  |
| length | [double](#double) |  |  |
| width | [double](#double) |  |  |
| position | [dcs.common.v0.Position](#dcs-common-v0-Position) |  |  |






<a name="dcs-world-v0-BoxVolume"></a>

### BoxVolume



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | Minimum corner in geographic coordinates. |
| max | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | Maximum corner in geographic coordinates. |






<a name="dcs-world-v0-GetAirbaseIDRequest"></a>

### GetAirbaseIDRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-world-v0-GetAirbaseIDResponse"></a>

### GetAirbaseIDResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |






<a name="dcs-world-v0-GetAirbaseParkingRequest"></a>

### GetAirbaseParkingRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| available | [bool](#bool) | optional |  |






<a name="dcs-world-v0-GetAirbaseParkingResponse"></a>

### GetAirbaseParkingResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| parking | [AirbaseParking](#dcs-world-v0-AirbaseParking) | repeated | The singular field name is retained for API compatibility. protolint:disable:next REPEATED_FIELD_NAMES_PLURALIZED |






<a name="dcs-world-v0-GetAirbaseRadioSilentModeRequest"></a>

### GetAirbaseRadioSilentModeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-world-v0-GetAirbaseRadioSilentModeResponse"></a>

### GetAirbaseRadioSilentModeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| silent | [bool](#bool) |  |  |






<a name="dcs-world-v0-GetAirbaseRunwaysRequest"></a>

### GetAirbaseRunwaysRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="dcs-world-v0-GetAirbaseRunwaysResponse"></a>

### GetAirbaseRunwaysResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| runways | [AirbaseRunway](#dcs-world-v0-AirbaseRunway) | repeated |  |






<a name="dcs-world-v0-GetAirbasesRequest"></a>

### GetAirbasesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |






<a name="dcs-world-v0-GetAirbasesResponse"></a>

### GetAirbasesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| airbases | [dcs.common.v0.Airbase](#dcs-common-v0-Airbase) | repeated |  |






<a name="dcs-world-v0-GetMarkPanelsRequest"></a>

### GetMarkPanelsRequest







<a name="dcs-world-v0-GetMarkPanelsResponse"></a>

### GetMarkPanelsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| mark_panels | [dcs.common.v0.MarkPanel](#dcs-common-v0-MarkPanel) | repeated |  |






<a name="dcs-world-v0-GetTheatreRequest"></a>

### GetTheatreRequest







<a name="dcs-world-v0-GetTheatreResponse"></a>

### GetTheatreResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| theatre | [string](#string) |  |  |






<a name="dcs-world-v0-PyramidVolume"></a>

### PyramidVolume



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| center | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | Center point of the pyramid vertex in geographic coordinates. |
| forward | [dcs.common.v0.Vector](#dcs-common-v0-Vector) |  | Orientation unit vectors defining the pyramid. Should be normalized. |
| right | [dcs.common.v0.Vector](#dcs-common-v0-Vector) |  |  |
| up | [dcs.common.v0.Vector](#dcs-common-v0-Vector) |  |  |
| length | [double](#double) |  | Max distance from vertex to objects considered inside the pyramid. |
| half_angle_horizontal | [double](#double) |  | Horizontal and vertical half-angles in radians. |
| half_angle_vertical | [double](#double) |  |  |






<a name="dcs-world-v0-SearchObjectsRequest"></a>

### SearchObjectsRequest
Search objects inside a 3D volume by category.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| categories | [dcs.common.v0.ObjectCategory](#dcs-common-v0-ObjectCategory) | repeated | Object categories to search for. If empty, no objects will be returned. |
| volume | [SearchVolume](#dcs-world-v0-SearchVolume) |  | The search volume. |






<a name="dcs-world-v0-SearchObjectsResponse"></a>

### SearchObjectsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| objects | [dcs.common.v0.Target](#dcs-common-v0-Target) | repeated | Objects found inside the volume. |






<a name="dcs-world-v0-SearchVolume"></a>

### SearchVolume
Volume used by world.searchObjects.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sphere | [SphereVolume](#dcs-world-v0-SphereVolume) |  |  |
| box | [BoxVolume](#dcs-world-v0-BoxVolume) |  |  |
| segment | [SegmentVolume](#dcs-world-v0-SegmentVolume) |  |  |
| pyramid | [PyramidVolume](#dcs-world-v0-PyramidVolume) |  |  |






<a name="dcs-world-v0-SegmentVolume"></a>

### SegmentVolume



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| from | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | Start point in geographic coordinates. |
| to | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | End point in geographic coordinates. |






<a name="dcs-world-v0-SetAirbaseCoalitionRequest"></a>

### SetAirbaseCoalitionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| coalition | [dcs.common.v0.Coalition](#dcs-common-v0-Coalition) |  |  |






<a name="dcs-world-v0-SetAirbaseCoalitionResponse"></a>

### SetAirbaseCoalitionResponse







<a name="dcs-world-v0-SetAirbaseRadioSilentModeRequest"></a>

### SetAirbaseRadioSilentModeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| silent | [bool](#bool) |  |  |






<a name="dcs-world-v0-SetAirbaseRadioSilentModeResponse"></a>

### SetAirbaseRadioSilentModeResponse







<a name="dcs-world-v0-SphereVolume"></a>

### SphereVolume



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| center | [dcs.common.v0.InputPosition](#dcs-common-v0-InputPosition) |  | Center point in geographic coordinates. |
| radius | [double](#double) |  | Radius in meters. |





 

 

 


<a name="dcs-world-v0-WorldService"></a>

### WorldService
https://wiki.hoggitworld.com/view/DCS_singleton_world

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetAirbases | [GetAirbasesRequest](#dcs-world-v0-GetAirbasesRequest) | [GetAirbasesResponse](#dcs-world-v0-GetAirbasesResponse) | https://wiki.hoggitworld.com/view/DCS_func_getAirbases |
| GetMarkPanels | [GetMarkPanelsRequest](#dcs-world-v0-GetMarkPanelsRequest) | [GetMarkPanelsResponse](#dcs-world-v0-GetMarkPanelsResponse) | https://wiki.hoggitworld.com/view/DCS_func_getMarkPanels |
| GetTheatre | [GetTheatreRequest](#dcs-world-v0-GetTheatreRequest) | [GetTheatreResponse](#dcs-world-v0-GetTheatreResponse) | Returns the theatre (Map name) of the mission |
| SearchObjects | [SearchObjectsRequest](#dcs-world-v0-SearchObjectsRequest) | [SearchObjectsResponse](#dcs-world-v0-SearchObjectsResponse) | https://wiki.hoggitworld.com/view/DCS_func_searchObjects |
| GetAirbaseParking | [GetAirbaseParkingRequest](#dcs-world-v0-GetAirbaseParkingRequest) | [GetAirbaseParkingResponse](#dcs-world-v0-GetAirbaseParkingResponse) | https://wiki.hoggitworld.com/view/DCS_func_getParking |
| GetAirbaseRunways | [GetAirbaseRunwaysRequest](#dcs-world-v0-GetAirbaseRunwaysRequest) | [GetAirbaseRunwaysResponse](#dcs-world-v0-GetAirbaseRunwaysResponse) | https://wiki.hoggitworld.com/view/DCS_func_getRunways |
| GetAirbaseID | [GetAirbaseIDRequest](#dcs-world-v0-GetAirbaseIDRequest) | [GetAirbaseIDResponse](#dcs-world-v0-GetAirbaseIDResponse) |  |
| GetAirbaseRadioSilentMode | [GetAirbaseRadioSilentModeRequest](#dcs-world-v0-GetAirbaseRadioSilentModeRequest) | [GetAirbaseRadioSilentModeResponse](#dcs-world-v0-GetAirbaseRadioSilentModeResponse) |  |
| SetAirbaseRadioSilentMode | [SetAirbaseRadioSilentModeRequest](#dcs-world-v0-SetAirbaseRadioSilentModeRequest) | [SetAirbaseRadioSilentModeResponse](#dcs-world-v0-SetAirbaseRadioSilentModeResponse) |  |
| SetAirbaseCoalition | [SetAirbaseCoalitionRequest](#dcs-world-v0-SetAirbaseCoalitionRequest) | [SetAirbaseCoalitionResponse](#dcs-world-v0-SetAirbaseCoalitionResponse) |  |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

