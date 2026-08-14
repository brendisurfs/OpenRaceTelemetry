//! Pure math helpers for turning raw MPU6050 register values into
//! physical units. No hardware access here, so this module is unit-testable
//! on the host.

use core::f32::consts::PI;

/// LSB per g at the +/- 2g range.
const ACCEL_SCALE_2G: f32 = 16384.0;

/// Weight given to the gyro-integrated angle in [`complementary_filter`].
const COMP_FILTER_ALPHA: f32 = 0.98;

const RAD_TO_DEG: f32 = 180.0 / PI;

/// Converts a raw MPU6050 temperature register reading to Celsius.
pub fn convert_temp(temp_raw: i16) -> f32 {
    temp_raw as f32 / 340.0 + 36.53
}

/// Combines a high and low byte read off the I2C bus into a signed sample.
pub fn combine_bytes(high: u8, low: u8) -> i16 {
    i16::from_be_bytes([high, low])
}

/// Roll angle, in degrees, as `arctan(accel_y / accel_z)`.
pub fn calculate_roll(accel_y: i16, accel_z: i16) -> f32 {
    libm::atan2f(accel_y as f32, accel_z as f32) * RAD_TO_DEG
}

/// Pitch angle, in degrees, as `arcsin(accel_x / 1g)`.
///
/// The ratio is clamped to [-1, 1] so that samples beyond 1g (from shock or
/// vibration rather than tilt) don't produce a NaN.
pub fn calculate_pitch(accel_x: i16) -> f32 {
    let accel_x_g = (accel_x as f32 / ACCEL_SCALE_2G).clamp(-1.0, 1.0);
    libm::asinf(accel_x_g) * RAD_TO_DEG
}

/// Inputs for one step of the complementary filter.
pub struct CompFilterData {
    /// Angle derived from the accelerometer, in degrees.
    pub accel_angle: f32,
    /// Angular rate from the gyro, in degrees per second.
    pub gyro_rate: f32,
    /// Time since the previous sample, in seconds.
    pub time_delta: f32,
    /// Filter output from the previous iteration, in degrees.
    pub prev_angle: f32,
}

/// Blends an accelerometer-derived angle with an integrated gyro rate.
///
/// Callers own `prev_angle` across iterations: store the return value and
/// feed it back in as `data.prev_angle` on the next call.
pub fn complementary_filter(data: &CompFilterData) -> f32 {
    COMP_FILTER_ALPHA * (data.prev_angle + data.gyro_rate * data.time_delta)
        + (1.0 - COMP_FILTER_ALPHA) * data.accel_angle
}
