#include <Arduino.h>
#include <Wire.h>

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

/*
 * Converts the temperature to Celsius
 */
static float convert_temp(int16_t temp_raw) {
  return temp_raw / 340.0f + 36.53f;
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

void read_imu_accel_data(void) {
  int16_t accel_x, accel_y, accel_z, temp_raw, gyro_x, gyro_y, gyro_z;

  Wire.beginTransmission(MPU_ADDR);
  Wire.write(ACCEL_OUT_H);
  Wire.endTransmission(false);

  size_t written_bytes = Wire.requestFrom(MPU_ADDR, READ_BUF_SIZE);
  if (written_bytes == 0) {
    Serial.println("Read bytes error");
  }

  // Wire.readBytes(read_buffer, READ_BUF_SIZE);

  // accel_x = (read_buffer[0] << 8) | read_buffer[1];
  // accel_y = (read_buffer[2] << 8) | read_buffer[3];
  // accel_z = (read_buffer[4] << 8) | read_buffer[5];
  // temp_raw = (read_buffer[6] << 8) | read_buffer[7];
  // gyro_x = (read_buffer[8] << 8) | read_buffer[9];
  // gyro_y = (read_buffer[10] << 8) | read_buffer[11];
  // gyro_z = (read_buffer[12] << 8) | read_buffer[13];

  accel_x = (Wire.read() << 8) | Wire.read();
  accel_y = (Wire.read() << 8) | Wire.read();
  accel_z = (Wire.read() << 8) | Wire.read();
  temp_raw = (Wire.read() << 8) | Wire.read();
  gyro_x = (Wire.read() << 8) | Wire.read();
  gyro_y = (Wire.read() << 8) | Wire.read();
  gyro_z = (Wire.read() << 8) | Wire.read();
  float temp_c = convert_temp(temp_raw);

  Serial.printf("accel: [%d %d %d] gyro: [%d %d %d] temp: %.2fC\n", accel_x,
                accel_y, accel_z, gyro_x, gyro_y, gyro_z, temp_c);
}

// /*
//  * Reads accel/gyro/temp data from the imu
//  */
// static void imu_read_accel_data(void) {
//   uint8_t reg_addr = 0x3B; /* ACCEL_XOUT_H */
//   uint8_t read_buffer[14];

//   esp_err_t err = i2c_master_transmit_receive(
//       s_imu_dev_handle, &reg_addr, 1, read_buffer, sizeof(read_buffer), 500);

//   ESP_ERROR_CHECK(err);

//   // This is just a fancy way of combining the bits sent over I2C.
//   // Shift the high bit left by 8 bits, append the low bit to create bytes.
//   int16_t accel_x = (read_buffer[0] << 8) | read_buffer[1];
//   int16_t accel_y = (read_buffer[2] << 8) | read_buffer[3];
//   int16_t accel_z = (read_buffer[4] << 8) | read_buffer[5];
//   int16_t temp_raw = (read_buffer[6] << 8) | read_buffer[7];
//   int16_t gyro_x = (read_buffer[8] << 8) | read_buffer[9];
//   int16_t gyro_y = (read_buffer[10] << 8) | read_buffer[11];
//   int16_t gyro_z = (read_buffer[12] << 8) | read_buffer[13];

//   float temp_c = convert_temp(temp_raw);

//   ESP_LOGI(IMU_TAG, "accel[%d %d %d] gyro[%d %d %d] temp=%.2fC", accel_x,
//            accel_y, accel_z, gyro_x, gyro_y, gyro_z, temp_c);
// }
