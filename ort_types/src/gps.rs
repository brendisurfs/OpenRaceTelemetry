//! GPS/NMEA types, shared between the firmware and the desktop app.

/// NMEA sentences are printable ASCII, at most 82 bytes including the
/// leading `$` and the trailing CRLF.
pub const NMEA_MAX_LEN: usize = 82;

/// Result of parsing an NMEA sentence's talker/message-type prefix.
///
/// For example `"$GPGGA,..."` has talker `"GP"` and message type `"GGA"`.
/// The C++ version used NUL-terminated `char` buffers; fixed-size byte
/// arrays carry the same data without needing the terminator.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NmeaMessage {
    #[cfg_attr(feature= "specta", specta(type = str))]
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
    pub talker: [u8; 2],

    #[cfg_attr(feature= "specta", specta(type = str))]
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
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
mod tests {
    use crate::NmeaMessage;

    #[test]
    fn test_parses_talker_and_message_type() {
        let sentence = "$GPGGA,123519,4807.038,N,01131.000,E*6A";

        let actual = NmeaMessage::from_bytes(sentence.as_bytes()).expect("nmea message to parse");
        let talker_str = String::from_utf8_lossy(&actual.talker);
        let msg_type_str = String::from_utf8_lossy(&actual.message_type);

        assert_eq!("GP", talker_str);
        assert_eq!("GGA", msg_type_str);
    }
}
