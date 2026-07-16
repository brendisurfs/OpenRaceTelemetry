#include "imu.h"

#include <Arduino.h>
#include <Wire.h>

#include <cstddef>
#include <cstdint>
#include <cstdlib>

#include "imu_math.h"
#include "pins_arduino.h"

#define MPU_ADDR 0x68
#define PWR_MGMT_REG 0x6B
#define ACCEL_CONFIG 0x1C
#define GYRO_CONFIG 0x1B
#define ACCEL_OUT_H 0x3B

#define READ_BUF_SIZE 14

void configure_i2c_wire_interface() {
  Wire.setPins(A4, A5);
  Wire.begin();

  // No external pull-up resistors on SDA/SCL — fall back to the ESP32's
  // weak internal pull-ups so the bus doesn't float.
  pinMode(A4, INPUT_PULLUP);
  pinMode(A5, INPUT_PULLUP);
}

/*
 * Probes every 7-bit I2C address (0x08-0x77) on bus_handle and logs which
 * ones ACK. Use this to sanity-check wiring before trusting a fixed
 * device address like 0x68/0x69.
 */
void scan_i2c_bus(void) {
  Serial.printf("Scanning I2C bus (SDA=%d, SCL=%d)...\n", A4, A5);

  int found = 0;
  for (uint8_t addr = 0x08; addr <= 0x77; addr++) {
    Wire.beginTransmission(addr);
    uint8_t err = Wire.endTransmission();
    if (err == 0) {
      Serial.printf("Found I2C device at 0x%02X\n", addr);
      found++;
    }
  }

  if (found == 0) {
    Serial.printf("No I2C devices found on the bus\n");
  }
}

void wake_mpu() {
  Wire.beginTransmission(MPU_ADDR);
  Wire.write(PWR_MGMT_REG);
  Wire.write(0x00);
  Wire.endTransmission(true);
}

void configure_accel_range() {
  Wire.beginTransmission(MPU_ADDR);
  Wire.write(ACCEL_CONFIG);
  Wire.write(0x00);  // set to default range.
  Wire.endTransmission(true);
}

// Configure gyroscope range to ±250 °/s
void configure_gyro() {
  Wire.beginTransmission(MPU_ADDR);
  Wire.write(GYRO_CONFIG);  // GYRO_CONFIG register
  Wire.write(0x00);         // ±250dps
  Wire.endTransmission(true);
}

void setup_imu() {
  wake_mpu();
  configure_gyro();
  configure_accel_range();
  Serial.println("MPU6050 initialized");
}

imu_data_t collect_imu_data(void) {
  imu_data_t data;

  data.accel_x = combine_bytes(Wire.read(), Wire.read());
  data.accel_y = combine_bytes(Wire.read(), Wire.read());
  data.accel_z = combine_bytes(Wire.read(), Wire.read());
  data.temp_raw = combine_bytes(Wire.read(), Wire.read());
  data.gyro_x = combine_bytes(Wire.read(), Wire.read());
  data.gyro_y = combine_bytes(Wire.read(), Wire.read());
  data.gyro_z = combine_bytes(Wire.read(), Wire.read());

  return data;
}

void print_imu_data(imu_data_t data) {
  float temp_celsius = convert_temp(data.temp_raw);

  float pitch = calculate_pitch(data.accel_x);
  float roll = calculate_roll(data.accel_y, data.accel_z);

  Serial.printf("Pitch: %.2f Roll: %.2f temp_c: %.2fC\n", pitch, roll,
                temp_celsius);

  // Serial.printf(
  //       "accel: [%d %d %d] gyro: [%d %d %d] temp_raw: %d temp_c: %.2fC\n",
  //       data.accel_x, data.accel_y, data.accel_z, data.gyro_x, data.gyro_y,
  //       data.gyro_z, data.temp_raw, temp_celsius);
}

void read_imu_accel_data(void) {
  Wire.beginTransmission(MPU_ADDR);
  Wire.write(ACCEL_OUT_H);
  Wire.endTransmission(false);

  size_t written_bytes = Wire.requestFrom(MPU_ADDR, READ_BUF_SIZE);
  if (written_bytes == 0) {
    Serial.println("Read bytes error");
  }

  imu_data_t data = collect_imu_data();

  float roll = calculate_roll(data.accel_y, data.accel_z);
  float gyro_rate = data.gyro_x / 131.0;  // Gyroscope Sensitivity
  print_imu_data(data);
}
