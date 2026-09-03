pub mod v0 {
    use crate::common::v0::{RawTransform, Transform};

    tonic::include_proto!("dcs.recovery.v0");

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct GetRecoverySnapshotResponseIntermediate {
        time: f64,
        carrier_raw_transform: Option<RawTransform>,
        aircraft_raw_transform: Option<RawTransform>,
        aircraft_draw_argument: Option<DrawArgumentObservation>,
        sequence: u64,
        queue_wait_ms: Option<f64>,
        lua_exec_ms: Option<f64>,
        queue_depth: Option<u32>,
        dequeued_model_time: Option<f64>,
    }

    impl TryFrom<GetRecoverySnapshotResponseIntermediate> for GetRecoverySnapshotResponse {
        type Error = String;

        fn try_from(value: GetRecoverySnapshotResponseIntermediate) -> Result<Self, Self::Error> {
            let carrier = value
                .carrier_raw_transform
                .ok_or_else(|| "recovery snapshot is missing carrierRawTransform".to_string())?;
            let aircraft = value
                .aircraft_raw_transform
                .ok_or_else(|| "recovery snapshot is missing aircraftRawTransform".to_string())?;

            Ok(Self {
                time: value.time,
                carrier: Some(recovery_transform(carrier)),
                aircraft: Some(recovery_transform(aircraft)),
                aircraft_draw_argument: value.aircraft_draw_argument,
                sequence: value.sequence,
                queue_wait_ms: value.queue_wait_ms,
                lua_exec_ms: value.lua_exec_ms,
                queue_depth: value.queue_depth,
                dequeued_model_time: value.dequeued_model_time,
            })
        }
    }

    fn recovery_transform(raw: RawTransform) -> RecoveryTransform {
        let transform = Transform::from(raw);
        RecoveryTransform {
            position: Some(transform.position),
            orientation: Some(transform.orientation),
            velocity: Some(transform.velocity),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn response_json(draw_argument: &str) -> String {
            response_json_with_extra(draw_argument, "")
        }

        /// `extra` is spliced verbatim after `sequence`; pass e.g. `, "queueWaitMs": 1.5`.
        fn response_json_with_extra(draw_argument: &str, extra: &str) -> String {
            format!(
                r#"{{
                    "time": 42.125,
                    "carrierRawTransform": {{
                        "position": {{"lat": 1.0, "lon": 2.0, "alt": 3.0, "u": 4.0, "v": 5.0}},
                        "positionNorth": {{"x": 4.0, "y": 0.0, "z": 6.0}},
                        "forward": {{"x": 0.0, "y": 0.0, "z": 1.0}},
                        "right": {{"x": 1.0, "y": 0.0, "z": 0.0}},
                        "up": {{"x": 0.0, "y": 1.0, "z": 0.0}},
                        "velocity": {{"x": 3.0, "y": 0.0, "z": 4.0}}
                    }},
                    "aircraftRawTransform": {{
                        "position": {{"lat": 6.0, "lon": 7.0, "alt": 8.0, "u": 9.0, "v": 10.0}},
                        "positionNorth": {{"x": 9.0, "y": 0.0, "z": 11.0}},
                        "forward": {{"x": 1.0, "y": 0.0, "z": 0.0}},
                        "right": {{"x": 0.0, "y": 0.0, "z": 1.0}},
                        "up": {{"x": 0.0, "y": 1.0, "z": 0.0}},
                        "velocity": {{"x": 0.0, "y": 0.0, "z": 2.0}}
                    }},
                    "aircraftDrawArgument": {draw_argument},
                    "sequence": 17{extra}
                }}"#
            )
        }

        #[test]
        fn snapshot_deserializes_two_transforms_and_observed_zero() {
            let response: GetRecoverySnapshotResponse =
                serde_json::from_str(&response_json(r#"{"status": 2, "value": 0.0}"#)).unwrap();

            assert_eq!(response.time, 42.125);
            assert_eq!(response.sequence, 17);
            assert_eq!(response.carrier.unwrap().velocity.unwrap().speed, 5.0);
            assert_eq!(response.aircraft.unwrap().velocity.unwrap().speed, 2.0);
            assert_eq!(
                response.aircraft_draw_argument.unwrap(),
                DrawArgumentObservation {
                    status: DrawArgumentStatus::Observed.into(),
                    value: Some(0.0),
                    detail: None,
                }
            );
        }

        #[test]
        fn snapshot_diagnostics_are_absent_when_lua_omits_them() {
            let response: GetRecoverySnapshotResponse =
                serde_json::from_str(&response_json(r#"{"status": 1}"#)).unwrap();

            assert_eq!(response.queue_wait_ms, None);
            assert_eq!(response.lua_exec_ms, None);
            assert_eq!(response.queue_depth, None);
            assert_eq!(response.dequeued_model_time, None);
            assert_eq!(response.aircraft_draw_argument.unwrap().detail, None);
        }

        #[test]
        fn snapshot_diagnostics_are_carried_when_lua_provides_them() {
            let response: GetRecoverySnapshotResponse =
                serde_json::from_str(&response_json_with_extra(
                    r#"{"status": 3, "detail": "getDrawArgumentValue returned nil"}"#,
                    r#", "queueWaitMs": 12.5, "luaExecMs": 0.75, "queueDepth": 3,
                        "dequeuedModelTime": 42.125"#,
                ))
                .unwrap();

            assert_eq!(response.queue_wait_ms, Some(12.5));
            assert_eq!(response.lua_exec_ms, Some(0.75));
            assert_eq!(response.queue_depth, Some(3));
            assert_eq!(response.dequeued_model_time, Some(42.125));
            assert_eq!(
                response.aircraft_draw_argument.unwrap(),
                DrawArgumentObservation {
                    status: DrawArgumentStatus::Unavailable.into(),
                    value: None,
                    detail: Some("getDrawArgumentValue returned nil".to_string()),
                }
            );
        }

        #[test]
        fn snapshot_preserves_not_requested_draw_argument_status() {
            let response: GetRecoverySnapshotResponse =
                serde_json::from_str(&response_json(r#"{"status": 1}"#)).unwrap();
            let observation = response.aircraft_draw_argument.unwrap();
            assert_eq!(
                observation.status,
                i32::from(DrawArgumentStatus::NotRequested)
            );
            assert_eq!(observation.value, None);
        }

        #[test]
        fn snapshot_preserves_unavailable_draw_argument_status() {
            let response: GetRecoverySnapshotResponse =
                serde_json::from_str(&response_json(r#"{"status": 3}"#)).unwrap();
            let observation = response.aircraft_draw_argument.unwrap();
            assert_eq!(
                observation.status,
                i32::from(DrawArgumentStatus::Unavailable)
            );
            assert_eq!(observation.value, None);
        }

        #[test]
        fn snapshot_rejects_a_missing_raw_transform() {
            let error = serde_json::from_str::<GetRecoverySnapshotResponse>(
                r#"{"time": 1.0, "sequence": 1}"#,
            )
            .unwrap_err();
            assert!(error.to_string().contains("carrierRawTransform"));
        }
    }
}
