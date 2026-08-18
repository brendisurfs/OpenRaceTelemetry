//! GPS/NMEA types, shared between the firmware and the desktop app.
//! NMEA sentences are printable ASCII, at most 86 bytes including the
//! leading `$` and the trailing CRLF.
//! Reference for sentence parsing: https://www.nautixia.com/blog/nmea-0183-sentences-explained

pub const MAX_NMEA_LEN: usize = 86;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum TalkerIdentifier {
    /// GPS
    #[default]
    GP,
    /// GLONASS
    GL,
    /// Galileo
    GA,
    /// Multi-constellation GNSS
    GN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum PositionFixIndicator {
    NotAvailableOrInvalid = 0,
    GpsSPSModeFixValid = 1,
    DifferentialGpsFixValid = 2,
    DeadReckoningMode = 6,
}

enum LatSign {
    North = 1,
    South = -1,
}
enum LongSign {
    East = 1,
    West = -1,
}

struct GPGGASentence {
    talker: TalkerIdentifier,
    /// hhmmss.sss
    utc_time: i32,
    latitude: f32,
    /// "N"=1, "S"=-1
    lat_sign: LatSign,
    longitude: f32,
    /// E=1, W=-1
    long_sign: LongSign,

    position_fix_indicator: PositionFixIndicator,
    /// apprently range is 0 - 12
    satellites_used: u8,
    /// Horizontal Dilution of Precision
    hdop: f32,
    msl_altitude_meters: f32,
    /// Meters
    units: u8,
}

/// Result of parsing an NMEA sentence's talker/message-type prefix.
///
/// For example `"$GPGGA,..."` has talker `"GP"` and message type `"GGA"`.
/// The C++ version used NUL-terminated `char` buffers; fixed-size byte
/// arrays carry the same data without needing the terminator.
/// These are raw bytes, not strings. This is the wire and event-log shape, so
/// it stays `Copy` and allocation-free. The frontend-facing string form lives
/// in `desktop/src-tauri/src/dto.rs`, converted once at the IPC boundary.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NmeaMessage {
    pub talker: [u8; 2],
    pub message_type: [u8; 3],
}

impl NmeaMessage {
    /// Parses the talker and message type off the front of an NMEA sentence.
    ///
    /// Returns `None` when the sentence is too short to hold a prefix (under 6
    /// bytes) or doesn't start with `$`. The C++ version signalled this with a
    /// `valid` flag, which `Option` expresses directly.
    ///
    /// Layout: `$` at index 0, talker at 1..3, message type at 3..6.
    pub fn from_bytes(nmea_message: &[u8]) -> Option<NmeaMessage> {
        let mut msg = NmeaMessage::default();

        if nmea_message.len() < 6 || nmea_message[0] != b'$' {
            return None;
        }

        msg.talker.copy_from_slice(&nmea_message[1..3]);
        msg.message_type.copy_from_slice(&nmea_message[3..6]);

        Some(msg)
    }
}

#[cfg(test)]
mod gps_tests {

    use crate::NmeaMessage;

    #[test]
    fn test_parses_talker_and_message_type() {
        let sentence = "$GPGGA,123519,4807.038,N,01131.000,E*6A";

        let buf = sentence.as_bytes();
        let actual = NmeaMessage::from_bytes(buf).expect("nmea message to parse");
        let talker_str = String::from_utf8_lossy(&actual.talker);
        let msg_type_str = String::from_utf8_lossy(&actual.message_type);

        assert_eq!("GP", talker_str);
        assert_eq!("GGA", msg_type_str);
    }

    #[test]
    fn test_parses_different_talker_and_message_type() {
        let sentence = "$GNRMC,123519.000,A,4807.038,N,01131.000,E*12";

        let byte_sentence = sentence.as_bytes();
        let cuh = &byte_sentence[13..20];
        println!("{:?}", cuh);

        let actual = NmeaMessage::from_bytes(byte_sentence).expect("nmea message to parse");
        let talker_str = String::from_utf8_lossy(&actual.talker);
        let msg_type_str = String::from_utf8_lossy(&actual.message_type);

        assert_eq!("GN", talker_str);
        assert_eq!("RMC", msg_type_str);
    }

    #[test]
    fn test_rejects_message_missing_dollar_prefix() {
        let sentence = "GPGGA,123519.000,4807.038,N,01131.000,E*6A";

        let actual = NmeaMessage::from_bytes(sentence.as_bytes());

        assert_eq!(None, actual);
    }

    #[test]
    fn test_rejects_message_too_short_to_contain_a_type() {
        let sentence = "$GPG";

        let actual = NmeaMessage::from_bytes(sentence.as_bytes());

        assert_eq!(None, actual);
    }

    #[test]
    fn test_rejects_empty_message() {
        let sentence = "";

        let actual = NmeaMessage::from_bytes(sentence.as_bytes());

        assert_eq!(None, actual);
    }
}
