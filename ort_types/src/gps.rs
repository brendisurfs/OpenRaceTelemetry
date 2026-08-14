//! GPS/NMEA types, shared between the firmware and the desktop app.

/// NMEA sentences are printable ASCII, at most 82 bytes including the
/// leading `$` and the trailing CRLF.
pub const NMEA_MAX_LEN: usize = 82;

/// Result of parsing an NMEA sentence's talker/message-type prefix.
///
/// For example `"$GPGGA,..."` has talker `"GP"` and message type `"GGA"`.
/// The C++ version used NUL-terminated `char` buffers; fixed-size byte
/// arrays carry the same data without needing the terminator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NmeaMessage {
    pub talker: [u8; 2],
    pub message_type: [u8; 3],
}

impl NmeaMessage {
    /// Parses the talker and message type off the front of an NMEA sentence.
    ///
    /// Returns `None` when the sentence is too short to hold a prefix (under 6
    /// bytes) or doesn't start with `$` — the C++ version signalled this with a
    /// `valid` flag, which `Option` expresses directly.
    ///
    /// Layout: `$` at index 0, talker at 1..3, message type at 3..6.
    pub fn from_bytes(nmea_message: &[u8]) -> Option<NmeaMessage> {
        todo!()
    }
}
