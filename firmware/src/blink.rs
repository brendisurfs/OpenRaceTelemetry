//! Status LED.
//!
//! On the STM32F411CE "Black Pill" the onboard LED sits on PC13 and is wired
//! active-low: driving the pin low sinks current through the LED and turns it
//! on. [`Led`] hides that inversion so callers can talk in terms of on/off.

use embassy_stm32::gpio::{Output, Pin};
use embassy_time::{Duration, Timer};

/// Blink cadence for the different states of the system.
pub enum FlashDelay {
    /// Slow blink while the GPS is acquiring a fix.
    Warmup,
    /// Short flash used for the "GPS connected" burst.
    Success,
}

impl FlashDelay {
    /// Warmup is 200ms, Success is 20ms.
    pub const fn duration(&self) -> Duration {
        todo!()
    }
}

/// The onboard status LED.
pub struct Led<'d> {
    output: Output<'d>,
}

impl<'d> Led<'d> {
    /// Configures `pin` as a push-pull output with the LED initially off.
    ///
    /// Active-low, so the initial [`Level`](embassy_stm32::gpio::Level) is the
    /// opposite of what "off" intuitively suggests.
    pub fn new(pin: impl Pin + 'd) -> Self {
        todo!()
    }

    /// Active-low: this drives the pin *low*.
    pub fn on(&mut self) {
        todo!()
    }

    /// Active-low: this drives the pin *high*.
    pub fn off(&mut self) {
        todo!()
    }

    pub fn toggle(&mut self) {
        todo!()
    }

    pub fn is_on(&self) -> bool {
        todo!()
    }
}

/// Blinks at a slower, consistent rate to show that the GPS is warming up.
///
/// One toggle plus one [`FlashDelay::Warmup`] wait per call — the caller drives
/// this in a loop, so it must `.await` rather than block.
pub async fn gps_warmup_blink(led: &mut Led<'_>) {
    todo!()
}

/// Flashes the LED to signal that the GPS has warmed up and acquired a fix.
///
/// Ten short flashes at [`FlashDelay::Success`]. Note the C++ version only
/// delayed between on and off, never after off, so the flashes ran together;
/// you probably want a delay on both edges here.
pub async fn gps_connected_blink(led: &mut Led<'_>) {
    todo!()
}

/// Heartbeat task: toggles the LED forever at a fixed interval.
///
/// Replaces the `TaskScheduler` entry from the Arduino build — Embassy runs
/// this as its own task instead of polling it from a scheduler in `loop()`.
#[embassy_executor::task]
pub async fn blink_task(mut led: Led<'static>, interval: Duration) -> ! {
    todo!()
}
