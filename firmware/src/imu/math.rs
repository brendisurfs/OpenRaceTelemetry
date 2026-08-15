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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_within(tolerance: f32, expected: f32, actual: f32, message: &str) {
        assert!(
            libm::fabsf(expected - actual) <= tolerance,
            "{message}: expected {expected}, got {actual}"
        );
    }

    #[test]
    fn converts_temp() {
        let expected = 29.33;
        let actual = convert_temp(-2448);

        assert_within(0.01, expected, actual, "Temp should be equal");
    }

    // --- calculate_roll -----------------------------------------------------

    #[test]
    fn roll_level_is_zero() {
        // accel_y = 0, accel_z = +1g -> level, no roll
        let actual = calculate_roll(0, 16384);
        assert_within(
            0.001,
            0.0,
            actual,
            "Level orientation should be 0 degrees roll",
        );
    }

    #[test]
    fn roll_90_degrees_right() {
        // accel_y = +1g, accel_z = 0 -> rolled 90 degrees right
        let actual = calculate_roll(16384, 0);
        assert_within(0.001, 90.0, actual, "Should be 90 degrees roll right");
    }

    #[test]
    fn roll_90_degrees_left() {
        // accel_y = -1g, accel_z = 0 -> rolled 90 degrees left
        let actual = calculate_roll(-16384, 0);
        assert_within(0.001, -90.0, actual, "Should be 90 degrees roll left");
    }

    #[test]
    fn roll_inverted_is_180_degrees() {
        // accel_z = -1g -> upside down
        let actual = calculate_roll(0, -16384);
        assert_within(
            0.001,
            180.0,
            actual,
            "Inverted orientation should be 180 degrees roll",
        );
    }

    #[test]
    fn roll_from_real_sensor_reading() {
        // Regression check against a real raw MPU6050 reading.
        let accel_y: i16 = -224;
        let accel_z: i16 = 15820;

        let expected = -0.8112135;
        let actual = calculate_roll(accel_y, accel_z);

        assert_within(
            0.001,
            expected,
            actual,
            "Roll should match atan2(accel_y, accel_z) in degrees",
        );
    }

    #[test]
    fn roll_is_never_nan() {
        let accel_y: i16 = 324;
        let accel_z: i16 = 12752;

        let actual = calculate_roll(accel_y, accel_z);
        assert!(!actual.is_nan(), "Roll should not be NaN");
    }

    // --- calculate_pitch -----------------------------------------------------

    #[test]
    fn pitch_flat_is_zero() {
        let actual = calculate_pitch(0);
        assert_within(
            0.001,
            0.0,
            actual,
            "Flat orientation should be 0 degrees pitch",
        );
    }

    #[test]
    fn pitch_nose_up_90_degrees() {
        // accel_x = +1g -> pitched straight up
        let actual = calculate_pitch(16384);
        assert_within(
            0.001,
            90.0,
            actual,
            "Nose-up 1g on X should be 90 degrees pitch",
        );
    }

    #[test]
    fn pitch_nose_down_90_degrees() {
        // accel_x = -1g -> pitched straight down
        let actual = calculate_pitch(-16384);
        assert_within(
            0.001,
            -90.0,
            actual,
            "Nose-down 1g on X should be -90 degrees pitch",
        );
    }

    #[test]
    fn pitch_30_degrees() {
        // accel_x = 0.5g -> asin(0.5) = 30 degrees
        let actual = calculate_pitch(8192);
        assert_within(0.001, 30.0, actual, "0.5g on X should be 30 degrees pitch");
    }

    #[test]
    fn pitch_clamps_above_range() {
        // accel_x beyond +1g (sensor noise/overshoot) should clamp instead of NaN
        let actual = calculate_pitch(20000);
        assert_within(
            0.001,
            90.0,
            actual,
            "Over-range accel_x should clamp to 90 degrees",
        );
    }

    #[test]
    fn pitch_clamps_below_range() {
        let actual = calculate_pitch(-20000);
        assert_within(
            0.001,
            -90.0,
            actual,
            "Under-range accel_x should clamp to -90 degrees",
        );
    }

    // --- complementary_filter --------------------------------------------------

    #[test]
    fn complementary_filter_single_step() {
        // alpha = 0.98: new_angle = alpha*(prev + gyro_rate*dt) +
        // (1-alpha)*accel_angle
        //             = 0.98*(5 + 20*0.02) + 0.02*8 = 5.452
        let actual = complementary_filter(&CompFilterData {
            accel_angle: 8.0,
            gyro_rate: 20.0,
            time_delta: 0.02,
            prev_angle: 5.0,
        });

        assert_within(
            0.001,
            5.452,
            actual,
            "Filter output should match the weighted blend formula",
        );
    }

    #[test]
    fn complementary_filter_trusts_gyro_short_term() {
        // With prev_angle=0 and a large single-step accel disagreement, the filter
        // should barely move off the gyro-predicted angle (heavily weighted at
        // alpha=0.98), demonstrating short-term gyro trust / accel noise rejection.
        let predicted_from_gyro = 0.0; // prev_angle + gyro_rate*dt, gyro_rate=0
        let actual = complementary_filter(&CompFilterData {
            accel_angle: 100.0,
            gyro_rate: 0.0,
            time_delta: 0.01,
            prev_angle: 0.0,
        });

        let delta = actual - predicted_from_gyro;
        assert!(
            delta < 5.0,
            "Single noisy accel reading should not swing the estimate far"
        );
    }

    #[test]
    fn complementary_filter_converges_to_accel_angle_when_stationary() {
        // With gyro_rate = 0 held constant (no rotation), repeated filtering should
        // converge the estimate toward the accelerometer's angle over time.
        let mut angle = 0.0;
        for _ in 0..2000 {
            angle = complementary_filter(&CompFilterData {
                accel_angle: 10.0,
                gyro_rate: 0.0,
                time_delta: 0.01,
                prev_angle: angle,
            });
        }

        assert_within(
            0.001,
            10.0,
            angle,
            "Filter should converge to the accel angle when stationary",
        );
    }

    #[test]
    fn complementary_filter_matches_gyro_prediction_when_accel_agrees() {
        // If the accelerometer reading exactly matches where the gyro predicts we
        // are, the filter output should equal that predicted angle regardless of
        // alpha weighting.
        let prev_angle = 5.0;
        let gyro_rate = 20.0;
        let time_delta = 0.02;
        let predicted = prev_angle + gyro_rate * time_delta;

        let actual = complementary_filter(&CompFilterData {
            accel_angle: predicted,
            gyro_rate,
            time_delta,
            prev_angle,
        });

        assert_within(
            0.001,
            predicted,
            actual,
            "Filter should match gyro prediction when accel agrees",
        );
    }

    #[test]
    fn complementary_filter_with_real_sensor_reading() {
        // Regression check against a real raw MPU6050 reading, starting from a
        // fresh (zeroed) filter state. Expected value is near zero, so this uses
        // an absolute tolerance rather than a relative one.
        let accel_y: i16 = 636;
        let accel_z: i16 = 9300;
        let gyro_x: i16 = -1107;

        let roll = calculate_roll(accel_y, accel_z);
        let gyro_rate = gyro_x as f32 / 131.0; // Gyroscope sensitivity (LSB/deg/s at +-250dps)

        let actual = complementary_filter(&CompFilterData {
            accel_angle: roll,
            gyro_rate,
            time_delta: 0.01,
            prev_angle: 0.0,
        });

        assert_within(
            0.001,
            -0.0045697,
            actual,
            "Expected complementary filter to match expected",
        );
    }
}
