//! GPS receiver over UART, speaking NMEA 0183.
//!
//! On the STM32F411CE this is USART1: PA9 = TX (MCU -> GPS), PA10 = RX
//! (GPS -> MCU). Most modules default to 9600 baud.

use embassy_stm32::mode::Async;
use embassy_stm32::usart::{self, Uart};

/// `NmeaMessage` and its parser are shared with the desktop app, which decodes
/// the same sentences off recorded logs.
pub use ort_types::gps::{MAX_NMEA_LEN, NmeaMessage};

/// Default baud rate for a stock NMEA module.
pub const GPS_BAUD: u32 = 9600;

const GPGGA_ID_REPR: &'static [u8; 5] = &[71u8, 80u8, 71u8, 71u8, 65u8];

/// The GPS receiver, bound to a UART.
pub struct Gps<'d> {
    uart: Uart<'d, Async>,
}

impl<'d> Gps<'d> {
    /// Wraps an already-configured UART.
    pub fn new(uart: Uart<'d, Async>) -> Self {
        defmt::todo!()
    }

    /// Reads bytes until a full sentence (`$` through the line terminator) has
    /// been collected, writing it into `buf` and returning the filled slice.
    ///
    /// Sentences can arrive mid-stream, so discard bytes until a `$` is seen
    /// before starting to accumulate, and drop anything longer than
    /// [`NMEA_MAX_LEN`] rather than overflowing `buf`.
    pub async fn read_sentence<'b>(
        &mut self,
        buf: &'b mut [u8; MAX_NMEA_LEN],
    ) -> Result<&'b [u8], usart::Error> {
        let sentence_identifier = &buf[1..5];
        // if our sentence is not a GPGGA sentence, skip it for now.
        match sentence_identifier {
            x if sentence_identifier == GPGGA_ID_REPR => {
                defmt::todo!("Handle GPGGA sentence");
            }
            _ => defmt::todo!("Handle any other sentence"),
        };
    }
}

/// Initializes GPS resources and waits for the module to start streaming.
pub async fn setup_gps(gps: &mut Gps<'_>) -> Result<(), usart::Error> {
    defmt::todo!()
}

#[cfg(test)]
mod gps_tests {
    #[test]
    fn gps_builds() {}
}
