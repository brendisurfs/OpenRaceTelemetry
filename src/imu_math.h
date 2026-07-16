#pragma once
#include <cstdint>

/**
 * Converts raw MPU6050 temp register reading to Celsius
 */
float convert_temp(int16_t temp_raw);

/**
 * Combines a high and low byte read off the I2C bus
 */
int16_t combine_bytes(uint8_t high, uint8_t low);

/**
 * Calculates roll by taking the
 * arctan of accel_y over accel_z
 */
float calculate_roll(int16_t accel_y, int16_t accel_z);

/**
 * Calculates the pitch angle by taking the arcsin of
  accel_x / gravity (9.8m/s^2)
 */
float calculate_pitch(int16_t accel_x);

struct CompFilterData {
  float accel_angle;
  float gyro_rate;
  float time_delta;
};

float complemetary_filter(struct CompFilterData data);
