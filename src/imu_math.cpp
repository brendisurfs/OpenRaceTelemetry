#include "imu_math.h"

#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <string>

// LSB per g at +- 2g range
static const float ACCEL_SCALE_2G = 16384.0f;

void todo(std::string message) {
  printf("%s\n", message.c_str());
  exit(1);
};

float convert_temp(int16_t temp_raw) {
  return temp_raw / 340.0f + 36.53f;
}

int16_t combine_bytes(uint8_t high, uint8_t low) {
  return (int16_t)((high << 8 | low));
}

/**
 * Roll angle is equal to arctan(Ay/Az)
 */
float calculate_roll(int16_t accel_y, int16_t accel_z) {
  return std::atan2(accel_y, accel_z) * (180.0f / M_PI);
}

float calculate_pitch(int16_t accel_x) {
  float accel_x_g = (float)accel_x / ACCEL_SCALE_2G;
  float clamped_accel_x_g = std::fmax(-1.0f, std::fmin(1.0f, accel_x_g));
  return std::asin(clamped_accel_x_g) * (180.0f / M_PI);
}

float complemetary_filter(float accel_angle,
                          float gyro_rate,
                          float time_delta) {
  static float prev_angle = 0.0;
  static const float alpha = 0.98;

  float new_angle =
      alpha * (prev_angle + gyro_rate * time_delta) + (1 - alpha) * accel_angle;

  prev_angle = new_angle;

  return roundf(new_angle);
}
