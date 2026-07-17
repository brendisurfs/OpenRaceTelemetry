#include <unity.h>

#include <cstdint>

#include "imu_math.h"

void setUp(void) {}
void tearDown(void) {}

void test_converts_temp() {
  float expected = 29.33;
  float actual = convert_temp(-2448);

  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(0.01f, expected, actual,
                                   "Temp should be equal");
}

// --- calculate_roll -----------------------------------------------------

void test_roll_level_is_zero() {
  // accel_y = 0, accel_z = +1g -> level, no roll
  float actual = calculate_roll(0, 16384);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, 0.0f, actual, "Level orientation should be 0 degrees roll");
}

void test_roll_90_degrees_right() {
  // accel_y = +1g, accel_z = 0 -> rolled 90 degrees right
  float actual = calculate_roll(16384, 0);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(0.001f, 90.0f, actual,
                                   "Should be 90 degrees roll right");
}

void test_roll_90_degrees_left() {
  // accel_y = -1g, accel_z = 0 -> rolled 90 degrees left
  float actual = calculate_roll(-16384, 0);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(0.001f, -90.0f, actual,
                                   "Should be 90 degrees roll left");
}

void test_roll_inverted_is_180_degrees() {
  // accel_z = -1g -> upside down
  float actual = calculate_roll(0, -16384);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, 180.0f, actual,
      "Inverted orientation should be 180 degrees roll");
}

void test_roll_from_real_sensor_reading() {
  // Regression check against a real raw MPU6050 reading.
  int16_t accel_y = -224;
  int16_t accel_z = 15820;

  float expected = -0.8112135f;
  float actual = calculate_roll(accel_y, accel_z);

  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, expected, actual,
      "Roll should match atan2(accel_y, accel_z) in degrees");
}

void test_roll_is_never_nan() {
  int16_t accel_y = 324;
  int16_t accel_z = 12752;

  float actual = calculate_roll(accel_y, accel_z);
  TEST_ASSERT_FALSE_MESSAGE(isnan(actual), "Roll should not be NaN");
}

// --- calculate_pitch -----------------------------------------------------

void test_pitch_flat_is_zero() {
  float actual = calculate_pitch(0);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, 0.0f, actual, "Flat orientation should be 0 degrees pitch");
}

void test_pitch_nose_up_90_degrees() {
  // accel_x = +1g -> pitched straight up
  float actual = calculate_pitch(16384);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, 90.0f, actual, "Nose-up 1g on X should be 90 degrees pitch");
}

void test_pitch_nose_down_90_degrees() {
  // accel_x = -1g -> pitched straight down
  float actual = calculate_pitch(-16384);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, -90.0f, actual, "Nose-down 1g on X should be -90 degrees pitch");
}

void test_pitch_30_degrees() {
  // accel_x = 0.5g -> asin(0.5) = 30 degrees
  float actual = calculate_pitch(8192);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(0.001f, 30.0f, actual,
                                   "0.5g on X should be 30 degrees pitch");
}

void test_pitch_clamps_above_range() {
  // accel_x beyond +1g (sensor noise/overshoot) should clamp instead of NaN
  float actual = calculate_pitch(20000);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, 90.0f, actual, "Over-range accel_x should clamp to 90 degrees");
}

void test_pitch_clamps_below_range() {
  float actual = calculate_pitch(-20000);
  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, -90.0f, actual,
      "Under-range accel_x should clamp to -90 degrees");
}

// --- complemetary_filter --------------------------------------------------

void test_complementary_filter_single_step() {
  // alpha = 0.98: new_angle = alpha*(prev + gyro_rate*dt) +
  // (1-alpha)*accel_angle
  //             = 0.98*(5 + 20*0.02) + 0.02*8 = 5.452
  float actual = complemetary_filter({.accel_angle = 8.0f,
                                      .gyro_rate = 20.0f,
                                      .time_delta = 0.02f,
                                      .prev_angle = 5.0f});

  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, 5.452f, actual,
      "Filter output should match the weighted blend formula");
}

void test_complementary_filter_trusts_gyro_short_term() {
  // With prev_angle=0 and a large single-step accel disagreement, the filter
  // should barely move off the gyro-predicted angle (heavily weighted at
  // alpha=0.98), demonstrating short-term gyro trust / accel noise rejection.
  float predicted_from_gyro = 0.0f;  // prev_angle + gyro_rate*dt, gyro_rate=0
  float actual = complemetary_filter({.accel_angle = 100.0f,
                                      .gyro_rate = 0.0f,
                                      .time_delta = 0.01f,
                                      .prev_angle = 0.0f});

  float delta = actual - predicted_from_gyro;
  TEST_ASSERT_TRUE_MESSAGE(
      delta < 5.0f,
      "Single noisy accel reading should not swing the estimate far");
}

void test_complementary_filter_converges_to_accel_angle_when_stationary() {
  // With gyro_rate = 0 held constant (no rotation), repeated filtering should
  // converge the estimate toward the accelerometer's angle over time.
  float angle = 0.0f;
  for (int i = 0; i < 2000; i++) {
    angle = complemetary_filter({.accel_angle = 10.0f,
                                 .gyro_rate = 0.0f,
                                 .time_delta = 0.01f,
                                 .prev_angle = angle});
  }

  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, 10.0f, angle,
      "Filter should converge to the accel angle when stationary");
}

void test_complementary_filter_matches_gyro_prediction_when_accel_agrees() {
  // If the accelerometer reading exactly matches where the gyro predicts we
  // are, the filter output should equal that predicted angle regardless of
  // alpha weighting.
  float prev_angle = 5.0f;
  float gyro_rate = 20.0f;
  float time_delta = 0.02f;
  float predicted = prev_angle + gyro_rate * time_delta;

  float actual = complemetary_filter({.accel_angle = predicted,
                                      .gyro_rate = gyro_rate,
                                      .time_delta = time_delta,
                                      .prev_angle = prev_angle});

  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, predicted, actual,
      "Filter should match gyro prediction when accel agrees");
}

void test_complementary_filter_with_real_sensor_reading() {
  // Regression check against a real raw MPU6050 reading, starting from a
  // fresh (zeroed) filter state. Expected value is near zero, so this uses
  // an absolute tolerance rather than a relative one.
  int16_t accel_y = 636;
  int16_t accel_z = 9300;
  int16_t gyro_x = -1107;

  float roll = calculate_roll(accel_y, accel_z);
  float gyro_rate =
      gyro_x / 131.0f;  // Gyroscope sensitivity (LSB/deg/s at +-250dps)

  float actual = complemetary_filter({.accel_angle = roll,
                                      .gyro_rate = gyro_rate,
                                      .time_delta = 0.01f,
                                      .prev_angle = 0.0f});

  TEST_ASSERT_FLOAT_WITHIN_MESSAGE(
      0.001f, -0.0045697f, actual,
      "Expected complementary filter to match expected");
}

int main(int argc, char** argv) {
  UNITY_BEGIN();

  RUN_TEST(test_converts_temp);

  RUN_TEST(test_roll_level_is_zero);
  RUN_TEST(test_roll_90_degrees_right);
  RUN_TEST(test_roll_90_degrees_left);
  RUN_TEST(test_roll_inverted_is_180_degrees);
  RUN_TEST(test_roll_from_real_sensor_reading);
  RUN_TEST(test_roll_is_never_nan);

  RUN_TEST(test_pitch_flat_is_zero);
  RUN_TEST(test_pitch_nose_up_90_degrees);
  RUN_TEST(test_pitch_nose_down_90_degrees);
  RUN_TEST(test_pitch_30_degrees);
  RUN_TEST(test_pitch_clamps_above_range);
  RUN_TEST(test_pitch_clamps_below_range);

  RUN_TEST(test_complementary_filter_single_step);
  RUN_TEST(test_complementary_filter_trusts_gyro_short_term);
  RUN_TEST(test_complementary_filter_converges_to_accel_angle_when_stationary);
  RUN_TEST(test_complementary_filter_matches_gyro_prediction_when_accel_agrees);
  RUN_TEST(test_complementary_filter_with_real_sensor_reading);

  UNITY_END();
}
