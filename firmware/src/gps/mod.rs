//! GPS receiver over UART, speaking NMEA 0183.
//!
//! On the STM32F411CE this is USART1: PA9 = TX (MCU -> GPS), PA10 = RX
//! (GPS -> MCU). Most modules default to 9600 baud.

use embassy_stm32::mode::Async;
use embassy_stm32::usart::{self, Uart};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use panic_probe as _;

static SHARED_GPS_CHAN: Channel<ThreadModeRawMutex, NmeaMessage, 16> = Channel::new();

/// `NmeaMessage` and its parser are shared with the desktop app, which decodes
/// the same sentences off recorded logs.
pub use ort_types::gps::{NMEA_MAX_LEN, NmeaMessage};

/// Default baud rate for a stock NMEA module.
pub const GPS_BAUD: u32 = 9600;

/// The GPS receiver, bound to a UART.
pub struct Gps<'d> {
    uart: Uart<'d, Async>,
}

impl<'d> Gps<'d> {
    /// Wraps an already-configured UART.
    pub fn new(uart: Uart<'d, Async>) -> Self {
        Self { uart }
    }

    /// Reads bytes until a full sentence (`$` through the line terminator) has
    /// been collected, writing to a buffer and parsing that buffer to an NmeaMessage,
    /// Returning the parsed NmeaMessage.
    ///
    /// Sentences can arrive mid-stream, so discard bytes until a `$` is seen
    /// before starting to accumulate, and drop anything longer than
    /// [`NMEA_MAX_LEN`] rather than overflowing `buf`.
    pub async fn read_sentence<'b>(&mut self) -> Result<NmeaMessage, usart::Error> {
        let mut buf_two = [0u8; NMEA_MAX_LEN];
        self.uart.read(&mut buf_two).await?;

        let Some(msg) = NmeaMessage::from_bytes(&buf_two) else {
            return Err(usart::Error::BufferTooLong);
        };

        Ok(msg)
    }
}

/// Initializes GPS resources and waits for the module to start streaming.
#[embassy_executor::task]
pub async fn run_gps_loop(mut uart: Uart<'static, Async>) -> ! {
    let mut buf = [0u8; NMEA_MAX_LEN];
    loop {
        uart.read(&mut buf).await.unwrap();
        let m = NmeaMessage::from_bytes(&buf).expect("error parsing nmea message from bytes");
        SHARED_GPS_CHAN.send(m).await;
    }
}
