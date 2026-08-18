use alloc::vec::{self, Vec};
use ort_types::MAX_NMEA_LEN;

// The maximum size a field can have in an NMEA message
const MAX_ITEM_LEN: usize = 12;
const CHAR_COMMA: u8 = 0x2C;

/// Parses a raw GPGGA NMEA sentence, splitting out each word from raw bytes.
/// Returns a Vec<Vec<u8>>, representing split words
fn parse_nmea_sentence(sentence: &[u8; MAX_NMEA_LEN]) -> Vec<Vec<u8>> {
    let mut i = 0;
    let mut buffer = Vec::<u8>::with_capacity(MAX_ITEM_LEN);
    let mut res = Vec::<Vec<u8>>::with_capacity(MAX_NMEA_LEN);

    while i < sentence.len() {
        let current_char = sentence[i];

        if current_char == CHAR_COMMA {
            res.push(buffer.clone());
            buffer.clear();
        } else {
            buffer.push(current_char);
        }

        i += 1;

        if sentence.get(i + 1).is_none() {
            res.push(buffer.clone());
            buffer.clear();
            i += 1;
            continue;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;
    use ort_types::MAX_NMEA_LEN;

    use crate::utils::parse_nmea_sentence;

    const SENTENCE: &'static [u8; MAX_NMEA_LEN] =
        b"$GPGGA,115739.00,4158.8441367,N,09147.4416929,W,4,13,0.9,255.747,M,-32.00,M,01,0000*6E";

    #[test]
    fn sentence_parses_from_raw_bytes() {
        let res = parse_nmea_sentence(SENTENCE);
        let expected = [
            "$GPGGA",
            "115739.00",
            "4158.8441367",
            "N",
            "09147.4416929",
            "W",
            "4",
            "13",
            "0.9",
            "255.747",
            "M",
            "-32.00",
            "M",
            "01",
            "0000*6",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

        let v = res
            .iter()
            .map(|v| String::from_utf8_lossy(v))
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        assert_eq!(expected, v);
    }
}
