//! JSON/TypeScript representations of the shared telemetry types.
//
//! `ort_types` stores fixed-size ASCII fields as byte arrays, which is appropriate
//! for the wire and event log but serializes to JSON as `[71, 80]`. These DTOs
//! provide the frontend-facing shape: Strings converted once at the IPC
//! boundary. Keeping them separate ensures the shared type remains `no_std` and
//! free of codegen concerns, and the generated TypeScript describes the JSON
//! actually sent.

use ort_types::{ImuData, NmeaMessage};
use serde::Serialize;
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum TalkerIdentifier {
    #[default]
    /// GPS
    GP,
    /// GLONASS
    GL,
    /// Galileo
    GA,
    /// Multi-constellation GNSS
    GN,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum PositionFixIndicator {
    #[default]
    NotAvailableOrInvalid = 0,
    GpsSPSModeFixValid = 1,
    DifferentialGpsFixValid = 2,
    DeadReckoningMode = 6,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum LatSign {
    #[default]
    North = 1,
    South = -1,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum LongSign {
    #[default]
    East = 1,
    West = -1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GPGGASentenceDto {
    pub talker: String,
    /// hhmmss.sss
    pub utc_time: i32,
    pub latitude: f32,

    /// "N"=1, "S"=-1
    pub lat_sign: LatSign,

    pub longitude: f32,

    /// E=1, W=-1
    pub long_sign: LongSign,

    pub position_fix_indicator: PositionFixIndicator,

    /// apprently range is 0 - 12
    pub satellites_used: u8,

    /// Horizontal Dilution of Precision
    pub hdop: f32,

    pub msl_altitude_meters: f32,

    /// Meters
    pub units: u8,
}

impl From<NmeaMessage> for GPGGASentenceDto {
    /// NMEA prefixes are printable ASCII by spec, anything else is a corrupt
    /// frame, so lossy conversion (U+FFFD) is preferable to failing the IPC
    /// call.
    fn from(msg: NmeaMessage) -> Self {
        Self::default()
    }
}

/// An NMEA prefix as the frontend sees it — `{ talker: "GP", messageType: "GGA" }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct NmeaMessageDto {
    pub talker: String,
    pub message_type: String,
}

impl From<NmeaMessage> for NmeaMessageDto {
    /// NMEA prefixes are printable ASCII by spec, anything else is a corrupt
    /// frame, so lossy conversion (U+FFFD) is preferable to failing the IPC
    /// call.
    fn from(msg: NmeaMessage) -> Self {
        Self {
            talker: String::from_utf8_lossy(&msg.talker).into_owned(),
            message_type: String::from_utf8_lossy(&msg.message_type).into_owned(),
        }
    }
}

/// One IMU sample, in raw register units.
///
/// The fields need no conversion (they are already `i16`).
/// DTO keeps `specta` out of `ort_types`, and it gives the
/// frontend camelCase names to match `NmeaMessageDto`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImuDataDto {
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub temp_raw: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
}

impl From<ImuData> for ImuDataDto {
    fn from(d: ImuData) -> Self {
        Self {
            accel_x: d.accel_x,
            accel_y: d.accel_y,
            accel_z: d.accel_z,
            temp_raw: d.temp_raw,
            gyro_x: d.gyro_x,
            gyro_y: d.gyro_y,
            gyro_z: d.gyro_z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SENTENCE: &[u8; 86] =
        b"$GPGGA,115739.00,4158.8441367,N,09147.4416929,W,4,13,0.9,255.747,M,-32.00,M,01,0000*6E";
    /// The reason these DTOs exist: the shared type serializes its ASCII
    /// fields as byte arrays, and the frontend expects strings.
    #[test]
    fn nmea_dto_serializes_prefix_fields_as_strings() {
        let msg = NmeaMessage::from_bytes(SENTENCE).expect("sentence parses");
        println!("{msg:#?}");

        let json = serde_json::to_string(&NmeaMessageDto::from(msg)).expect("dto serializes");
        println!("{json}");

        assert_eq!(r#"{"talker":"GP","messageType":"GGA"}"#, json);
    }

    /// A corrupt frame should still render rather than fail the IPC call.
    #[test]
    fn nmea_dto_replaces_non_utf8_bytes() {
        let msg = NmeaMessage {
            talker: [0xFF, 0xFE],
            message_type: *b"GGA",
            ..Default::default()
        };

        let dto = NmeaMessageDto::from(msg);

        assert_eq!("\u{FFFD}\u{FFFD}", dto.talker);
        assert_eq!("GGA", dto.message_type);
    }

    #[test]
    fn imu_dto_preserves_raw_register_values() {
        let data = ImuData {
            accel_x: -1,
            accel_y: 2,
            accel_z: 3,
            temp_raw: 4,
            gyro_x: 5,
            gyro_y: 6,
            gyro_z: 7,
        };

        let json = serde_json::to_string(&ImuDataDto::from(data)).expect("dto serializes");

        assert_eq!(
            r#"{"accelX":-1,"accelY":2,"accelZ":3,"tempRaw":4,"gyroX":5,"gyroY":6,"gyroZ":7}"#,
            json
        );
    }
}
