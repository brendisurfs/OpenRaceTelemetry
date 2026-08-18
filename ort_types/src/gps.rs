//! GPS/NMEA types, shared between the firmware and the desktop app.
//! NMEA sentences are printable ASCII, at most 86 bytes including the
//! leading `$` and the trailing CRLF.
//! Reference for sentence parsing: https://www.nautixia.com/blog/nmea-0183-sentences-explained

pub const MAX_NMEA_LEN: usize = 86;

struct ByteCursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ByteCursor<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    pub fn take<const N: usize>(&mut self) -> &'a [u8; N] {
        let slice = &self.buf[self.pos..self.pos + N];
        self.pos += N;

        slice.try_into().expect("slice length matches N")
    }

    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }
}

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

pub struct GPGGASentence {
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

mod sizing {
    pub const TALKER: usize = 2;
    pub const MESSAGE_TYPE: usize = 3;
    pub const UTC_TIME: usize = 9;
    pub const LATITUDE: usize = 12;
    pub const LAT_SIGN: usize = 1;
    pub const LONGITUDE: usize = 13;
    pub const LONG_SIGN: usize = 1;
    pub const POSITION_FIX_INDICATOR: usize = 1;
    pub const SATELLITES_USED: usize = 2;
    pub const HDOP: usize = 3;
    pub const MSL_ALTITUDE_METERS: usize = 7;
    pub const UNITS: usize = 1;
    pub const GEOIDAL_SEPARATION: usize = 6;
    pub const GEOIDAL_UNITS: usize = 1;
    pub const AGE_OF_DIFF: usize = 2;
    pub const CHECKSUM: usize = 7;
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
    pub talker: [u8; sizing::TALKER],
    pub message_type: [u8; sizing::MESSAGE_TYPE],
    pub utc_time: [u8; sizing::UTC_TIME],
    pub latitude: [u8; sizing::LATITUDE],
    lat_sign: [u8; sizing::LAT_SIGN],
    longitude: [u8; sizing::LONGITUDE],
    /// E=1, W=-1
    long_sign: [u8; sizing::LONG_SIGN],

    position_fix_indicator: [u8; sizing::POSITION_FIX_INDICATOR],
    /// apprently range is 0 - 12
    satellites_used: [u8; sizing::SATELLITES_USED],
    /// Horizontal Dilution of Precision
    hdop: [u8; sizing::HDOP],
    msl_altitude_meters: [u8; sizing::MSL_ALTITUDE_METERS],
    /// Meters
    units: [u8; sizing::UNITS],
    geoidal_separation: [u8; sizing::GEOIDAL_SEPARATION],
    /// Meters
    geoidal_units: [u8; sizing::GEOIDAL_UNITS],
    age_of_diff: [u8; sizing::AGE_OF_DIFF],
    checksum: [u8; sizing::CHECKSUM],
}

impl NmeaMessage {
    /// Parses the talker and message type off the front of an NMEA sentence.
    ///
    /// Returns `None` when the sentence is too short to hold a prefix (under 6
    /// bytes) or doesn't start with `$`. The C++ version signalled this with a
    /// `valid` flag, which `Option` expresses directly.
    pub fn from_bytes(nmea_message: &[u8]) -> Option<NmeaMessage> {
        if nmea_message.len() < 6 || nmea_message[0] != b'$' {
            return None;
        }

        // We skip 1 after every call in order to skip commas.
        let mut cursor = ByteCursor::new(nmea_message);
        let mut msg = NmeaMessage::default();
        cursor.skip(1);
        msg.talker = *cursor.take::<{ sizing::TALKER }>();
        msg.message_type = *cursor.take::<{ sizing::MESSAGE_TYPE }>();
        cursor.skip(1);
        msg.utc_time = *cursor.take::<{ sizing::UTC_TIME }>();
        cursor.skip(1);
        msg.latitude = *cursor.take::<{ sizing::LATITUDE }>();
        cursor.skip(1);
        msg.lat_sign = *cursor.take::<{ sizing::LAT_SIGN }>();
        cursor.skip(1);
        msg.longitude = *cursor.take::<{ sizing::LONGITUDE }>();
        cursor.skip(1);
        msg.long_sign = *cursor.take::<{ sizing::LONG_SIGN }>();
        cursor.skip(1);
        msg.position_fix_indicator = *cursor.take::<{ sizing::POSITION_FIX_INDICATOR }>();
        cursor.skip(1);
        msg.satellites_used = *cursor.take::<{ sizing::SATELLITES_USED }>();
        cursor.skip(1);
        msg.hdop = *cursor.take::<{ sizing::HDOP }>();
        cursor.skip(1);
        msg.msl_altitude_meters = *cursor.take::<{ sizing::MSL_ALTITUDE_METERS }>();
        cursor.skip(1);
        msg.units = *cursor.take::<{ sizing::UNITS }>();
        cursor.skip(1);
        msg.geoidal_separation = *cursor.take::<{ sizing::GEOIDAL_SEPARATION }>();
        cursor.skip(1);
        msg.geoidal_units = *cursor.take::<{ sizing::GEOIDAL_UNITS }>();
        cursor.skip(1);
        msg.age_of_diff = *cursor.take::<{ sizing::AGE_OF_DIFF }>();
        cursor.skip(1);
        msg.checksum = *cursor.take::<{ sizing::CHECKSUM }>();

        Some(msg)
    }
}

#[cfg(test)]
mod gps_tests {

    use crate::NmeaMessage;

    const SENTENCE: &[u8; 86] =
        b"$GPGGA,115739.00,4158.8441367,N,09147.4416929,W,4,13,0.9,255.747,M,-32.00,M,01,0000*6E";

    #[test]
    fn test_parses_talker_and_message_type() {
        let actual = NmeaMessage::from_bytes(SENTENCE).expect("nmea message to parse");
        let talker_str = String::from_utf8_lossy(&actual.talker);
        let msg_type_str = String::from_utf8_lossy(&actual.message_type);
        let utc_time_str = String::from_utf8_lossy(&actual.utc_time);

        assert_eq!("GP", talker_str);
        assert_eq!("GGA", msg_type_str);
        assert_eq!("115739.00", utc_time_str);
    }

    #[test]
    fn test_parses_different_talker_and_message_type() {
        // todo!("Change sentence type to test this");
        // let actual = NmeaMessage::from_bytes(SENTENCE).expect("nmea message to parse");
        // let talker_str = String::from_utf8_lossy(&actual.talker);
        // let msg_type_str = String::from_utf8_lossy(&actual.message_type);
        //
        // assert_eq!("GP", talker_str);
        // assert_eq!("GGA", msg_type_str);
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
