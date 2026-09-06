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

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct RecoveryUnitObservationIntermediate {
        status: i32,
        expected_name: String,
        expected_id: u32,
        resolved_id: Option<u32>,
        raw_transform: Option<RawTransform>,
        read_started_offset_us: Option<u64>,
        read_finished_offset_us: Option<u64>,
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct RecoveryTelemetrySnapshotIntermediate {
        recovery_handle: String,
        source_epoch: String,
        sequence: u64,
        capture_tick: u64,
        capture_time: f64,
        aircraft: Option<RecoveryUnitObservationIntermediate>,
        carrier: Option<RecoveryUnitObservationIntermediate>,
        capture_duration_us: Option<u64>,
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ReadRecoveryTelemetryResponseIntermediate {
        source_epoch: String,
        recovery_handle: String,
        lifecycle_status: i32,
        oldest_available_sequence: u64,
        newest_available_sequence: u64,
        next_after_sequence: u64,
        loss_reason: i32,
        overflow_count: u64,
        configured_period: f64,
        capacity: u32,
        retention_seconds: f64,
        #[serde(default)]
        snapshots: Vec<RecoveryTelemetrySnapshotIntermediate>,
        diagnostics: Option<RecoveryTelemetryDiagnostics>,
        lease_expires_at: f64,
        read_time: f64,
    }

    impl From<RecoveryUnitObservationIntermediate> for RecoveryUnitObservation {
        fn from(value: RecoveryUnitObservationIntermediate) -> Self {
            Self {
                status: value.status,
                expected_name: value.expected_name,
                expected_id: value.expected_id,
                resolved_id: value.resolved_id,
                transform: value.raw_transform.map(recovery_transform),
                read_started_offset_us: value.read_started_offset_us,
                read_finished_offset_us: value.read_finished_offset_us,
            }
        }
    }

    impl TryFrom<RecoveryTelemetrySnapshotIntermediate> for RecoveryTelemetrySnapshot {
        type Error = String;

        fn try_from(value: RecoveryTelemetrySnapshotIntermediate) -> Result<Self, Self::Error> {
            Ok(Self {
                recovery_handle: value.recovery_handle,
                source_epoch: value.source_epoch,
                sequence: value.sequence,
                capture_tick: value.capture_tick,
                capture_time: value.capture_time,
                aircraft: Some(value.aircraft.ok_or("snapshot is missing aircraft")?.into()),
                carrier: Some(value.carrier.ok_or("snapshot is missing carrier")?.into()),
                capture_duration_us: value.capture_duration_us,
            })
        }
    }

    impl TryFrom<ReadRecoveryTelemetryResponseIntermediate> for ReadRecoveryTelemetryResponse {
        type Error = String;

        fn try_from(value: ReadRecoveryTelemetryResponseIntermediate) -> Result<Self, Self::Error> {
            Ok(Self {
                source_epoch: value.source_epoch,
                recovery_handle: value.recovery_handle,
                lifecycle_status: value.lifecycle_status,
                oldest_available_sequence: value.oldest_available_sequence,
                newest_available_sequence: value.newest_available_sequence,
                next_after_sequence: value.next_after_sequence,
                loss_reason: value.loss_reason,
                overflow_count: value.overflow_count,
                configured_period: value.configured_period,
                capacity: value.capacity,
                retention_seconds: value.retention_seconds,
                snapshots: value
                    .snapshots
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                diagnostics: value.diagnostics,
                lease_expires_at: value.lease_expires_at,
                read_time: value.read_time,
            })
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

        #[test]
        fn telemetry_batch_converts_nested_raw_transforms_and_metadata() {
            let raw = r#"{
                "position": {"lat":1.0,"lon":2.0,"alt":3.0,"u":4.0,"v":5.0},
                "positionNorth":{"x":4.0,"y":0.0,"z":6.0},
                "forward":{"x":0.0,"y":0.0,"z":1.0},
                "right":{"x":1.0,"y":0.0,"z":0.0},
                "up":{"x":0.0,"y":1.0,"z":0.0},
                "velocity":{"x":3.0,"y":0.0,"z":4.0}
            }"#;
            let json = format!(
                r#"{{
                    "sourceEpoch":"epoch","recoveryHandle":"r1","lifecycleStatus":1,
                    "oldestAvailableSequence":1,"newestAvailableSequence":1,
                    "nextAfterSequence":1,"lossReason":4,"overflowCount":0,
                    "configuredPeriod":0.05,"capacity":600,"retentionSeconds":30.0,
                    "snapshots":[{{
                        "recoveryHandle":"r1","sourceEpoch":"epoch","sequence":1,
                        "captureTick":9,"captureTime":42.0,
                        "aircraft":{{"status":1,"expectedName":"jet","expectedId":10,
                            "resolvedId":10,"rawTransform":{raw}}},
                        "carrier":{{"status":1,"expectedName":"ship","expectedId":20,
                            "resolvedId":20,"rawTransform":{raw}}},
                        "captureDurationUs":12
                    }}],
                    "diagnostics":{{
                        "lastCaptureTime":42.0,"configuredPeriod":0.05,"observedGap":0.05,
                        "missedCaptureIntervals":0,"snapshotsProduced":1,
                        "invalidSnapshotsProduced":0,"snapshotsServed":1,
                        "retentionExpirationCount":0,"capacityOverflowCount":0,
                        "highWaterMark":1,"activeRecoveries":1,"activeCarriers":1,
                        "lastCaptureDurationUs":12,"readBatches":1,"lastBatchSize":1,
                        "sourceAge":0.0
                    }},
                    "leaseExpiresAt":102.0,"readTime":42.0
                }}"#
            );
            let batch: ReadRecoveryTelemetryResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(batch.snapshots.len(), 1);
            assert_eq!(
                batch.loss_reason,
                i32::from(RecoveryTelemetryLossReason::Mixed)
            );
            assert_eq!(batch.snapshots[0].capture_tick, 9);
            assert_eq!(
                batch.snapshots[0]
                    .carrier
                    .as_ref()
                    .unwrap()
                    .transform
                    .as_ref()
                    .unwrap()
                    .velocity
                    .as_ref()
                    .unwrap()
                    .speed,
                5.0
            );
            assert_eq!(batch.diagnostics.unwrap().active_carriers, 1);
        }
    }
}
