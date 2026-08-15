//! MPU6050 6-axis IMU over I2C.
//!
//! On the STM32F411CE the sensor hangs off I2C1: PB6 = SCL, PB7 = SDA.
//! Unlike the ESP32 build, which fell back to the MCU's weak internal
//! pull-ups, this expects external 4.7k pull-ups to 3V3 on both lines —
//! `embassy-stm32` configures I2C pins as open-drain without them.

mod math;

use defmt::{info, warn};
use embassy_stm32::i2c::{self, I2c, Master};
use embassy_stm32::mode::Async;
use embassy_time::Duration;

use crate::imu::math::{calculate_pitch, calculate_roll, convert_temp};

/// `ImuData` is shared with the desktop app; the decoding of a burst read into
/// one lives alongside it in [`ort_types`].
pub use ort_types::imu::{ImuData, READ_BUF_SIZE};

/// Sample period for the IMU read task: 20ms, i.e. 50Hz (`ms = 1000 / Hz`).
pub const IMU_READ_INTERVAL: Duration = Duration::from_millis(20);

/// 7-bit address of the MPU6050 with AD0 tied low. Tie AD0 high for 0x69.
const MPU_ADDR: u8 = 0x68;

const PWR_MGMT_REG: u8 = 0x6B;
const GYRO_CONFIG: u8 = 0x1B;
const ACCEL_CONFIG: u8 = 0x1C;
const ACCEL_OUT_H: u8 = 0x3B;

/// The MPU6050, bound to an I2C bus.
pub struct Imu<'d> {
    i2c: I2c<'d, Async, Master>,
}

impl<'d> Imu<'d> {
    /// Wraps an already-configured I2C bus. Call [`Imu::setup`] before reading.
    pub fn new(i2c: I2c<'d, Async, Master>) -> Self {
        todo!()
    }

    /// Wakes the device and configures the accel and gyro ranges.
    pub async fn setup(&mut self) -> Result<(), i2c::Error> {
        todo!()
    }

    /// Clears the sleep bit in PWR_MGMT_1 by writing 0x00 — the MPU6050 boots
    /// asleep and NAKs data reads until this lands.
    async fn wake(&mut self) -> Result<(), i2c::Error> {
        todo!()
    }

    /// Configures the gyroscope range to the +/- 250 deg/s default (0x00).
    async fn configure_gyro(&mut self) -> Result<(), i2c::Error> {
        todo!()
    }

    /// Configures the accelerometer range to the +/- 2g default (0x00), which
    /// is the range `ACCEL_SCALE_2G` in [`crate::imu::math`] assumes.
    async fn configure_accel_range(&mut self) -> Result<(), i2c::Error> {
        todo!()
    }

    /// Writes one config byte: a single transaction of `[register, value]`.
    async fn write_register(&mut self, register: u8, value: u8) -> Result<(), i2c::Error> {
        todo!()
    }

    /// Burst-reads accel, temperature, and gyro in a single transaction.
    ///
    /// `I2c::write_read` is the equivalent of the C++ pair of
    /// `endTransmission(false)` + `requestFrom`: it writes the start register,
    /// then issues a repeated START and clocks out [`READ_BUF_SIZE`] bytes
    /// without releasing the bus.
    pub async fn read(&mut self) -> Result<ImuData, i2c::Error> {
        todo!()
    }
}

/// Probes every 7-bit I2C address (0x08-0x77) and logs which ones ACK.
///
/// Use this to sanity-check wiring before trusting a fixed device address
/// like 0x68/0x69. A zero-length write ACKs on the address alone, so it
/// detects a device without touching its registers.
pub async fn scan_i2c_bus(i2c: &mut I2c<'_, Async, Master>) {
    todo!()
}

/// Logs orientation derived from the accelerometer.
pub fn log_roll_pitch(data: &ImuData) {
    todo!()
}

/// Logs the raw sample set alongside the decoded temperature.
pub fn log_raw_imu_data(data: &ImuData) {
    todo!()
}

/// Samples the IMU at [`IMU_READ_INTERVAL`] and logs orientation.
///
/// Use an [`embassy_time::Ticker`] rather than `Timer::after`: a ticker holds
/// the sample rate steady instead of letting the read time drift the period.
/// A failed read should warn and keep the loop alive, not panic the task.
#[embassy_executor::task]
pub async fn imu_task(mut imu: Imu<'static>) -> ! {
    todo!()
}

#[cfg(test)]
mod imu_tests {
    #[test]
    fn does_log() {}
}
