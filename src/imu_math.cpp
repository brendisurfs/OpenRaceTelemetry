#include "imu_math.h"

#include <cstdint>

float convert_temp(int16_t temp_raw) { return temp_raw / 340.0f + 36.53f; }

int16_t combine_bytes(uint8_t high, uint8_t low) {
  return (int16_t)((high << 8 | low));
}
