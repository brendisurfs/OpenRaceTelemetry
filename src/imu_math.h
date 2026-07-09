#pragma once
#include <cstdint>

/*
 * Converts raw MPU6050 temp register reading to Celsius
 */
float convert_temp(int16_t temp_raw);

/*
 * Combines a high and low byte read off the I2C bus
 */
int16_t combine_bytes(uint8_t high, uint8_t low);
