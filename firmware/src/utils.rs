use alloc::vec::{self, Vec};
use defmt::todo;
use ort_types::MAX_NMEA_LEN;

// The maximum size a field can have in an NMEA message
const MAX_ITEM_LEN: usize = 12;

fn parse_nmea_sentence(sentence: &[u8; MAX_NMEA_LEN]) -> Vec<u8> {
    let mut buffer = Vec::<u8>::with_capacity(MAX_ITEM_LEN);
    let mut res = Vec::<u8>::with_capacity(MAX_NMEA_LEN);
    let mut i = 0;

    while i < sentence.len() {
        let current_char = sentence[i];

        if current_char == 44 {
            // Buffer is drained, so no need to call clear
            res.append(&mut buffer);
            i += 1;
        } else {
            buffer.push(current_char);
            i += 1;
        }

        let next_char = if i + 1 < sentence.len() {
            Some(sentence[i + 1])
        } else {
            None
        };

        if next_char.is_none() {
            res.append(&mut buffer);
            i += 1;
            continue;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use ort_types::MAX_NMEA_LEN;

    use crate::utils::parse_nmea_sentence;

    const SENTENCE: &'static [u8; MAX_NMEA_LEN] =
        b"$GPGGA,115739.00,4158.8441367,N,09147.4416929,W,4,13,0.9,255.747,M,-32.00,M,01,0000*6E";

    #[test]
    fn it_parses() {
        parse_nmea_sentence(SENTENCE);
        assert!(true);
    }
}
