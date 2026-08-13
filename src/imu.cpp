#include "imu.h"
#include "blink.h"

#include <cstdio>
#include <cstdlib>

#include <stm32f4xx_hal.h>

#include "imu_math.h"
#include "stm32f411xe.h"
#include "stm32f4xx_hal_def.h"
#include "stm32f4xx_hal_gpio.h"
#include "stm32f4xx_hal_gpio_ex.h"
#include "stm32f4xx_hal_i2c.h"
#include "stm32f4xx_hal_rcc.h"

#define MPU_ADDR 0x68
#define PWR_MGMT_REG 0x6B
#define ACCEL_CONFIG 0x1C
#define GYRO_CONFIG 0x1B
#define ACCEL_OUT_H 0x3B

#define READ_BUF_SIZE 14

// HAL takes the 8-bit address, so the 7-bit device address is shifted up one.
#define MPU_ADDR_8BIT (MPU_ADDR << 1)
#define I2C_TIMEOUT_MS 100

/**
 * HAL stores per-bus state in this handle, so every HAL_I2C_* call after init
 * has to reference the same object. it lives at file scope for that reason.
 */
static I2C_HandleTypeDef i2c_handle_config;

/**
 * A generic error handler for i2c config/setup.
 * As of writing this, its just an error blink, but later would be nice to
 * either log it or do something more productive, if possible.
 */
void handle_i2c_error() {
  while (1) {
    i2c_error_blink();
  }
}

/**
 * Writes a single byte to one MPU6050 register.
 * HAL_I2C_Mem_Write handles the sequence that the Arduino Wire implementation
 * spread across four calls.
 */
static HAL_StatusTypeDef mpu_write_reg(uint8_t reg, uint8_t value) {
  return HAL_I2C_Mem_Write(&i2c_handle_config, MPU_ADDR_8BIT, reg,
                           I2C_MEMADD_SIZE_8BIT, &value, 1, I2C_TIMEOUT_MS);
}

/**
 * Enable our GPIO for the I2C bus
 */
void configure_i2c_gpio(void) {
  __HAL_RCC_GPIOB_CLK_ENABLE();

  GPIO_InitTypeDef gpio_config{
      .Pin = GPIO_PIN_6 | GPIO_PIN_7,
      .Mode = GPIO_MODE_AF_OD,
      .Pull = GPIO_NOPULL,
      .Speed = GPIO_SPEED_FREQ_HIGH,
      .Alternate = GPIO_AF4_I2C1,
  };

  HAL_GPIO_Init(GPIOB, &gpio_config);
}

void configure_i2c_wire_interface() {
  __HAL_RCC_I2C1_CLK_ENABLE();

  I2C_InitTypeDef init_config{
      .ClockSpeed = 100000,
      .DutyCycle = I2C_DUTYCYCLE_2,

      // Only if this module were a slave module would we change this
      .OwnAddress1 = 0,

      // MPU6050 uses 7-bit addressing
      .AddressingMode = I2C_ADDRESSINGMODE_7BIT,

      // Only relevant for slave mode
      .DualAddressMode = I2C_DUALADDRESS_DISABLE,
      .OwnAddress2 = 0,
      .GeneralCallMode = I2C_GENERALCALL_DISABLE,
      .NoStretchMode = I2C_NOSTRETCH_DISABLE,

  };

  i2c_handle_config.Instance = I2C1;
  i2c_handle_config.Init = init_config;

  HAL_StatusTypeDef status = HAL_I2C_Init(&i2c_handle_config);

  if (status != HAL_OK) {
    handle_i2c_error();
  }
}

/**
 * Probes every 7-bit I2C address (0x08-0x77) on bus_handle and logs which
 * ones ACK. Use this to sanity-check wiring before trusting a fixed device
 * address.
 */
void scan_i2c_bus(void) {
  // TODO: implement scan.
}

void wake_mpu() {
  if (mpu_write_reg(PWR_MGMT_REG, 0x00) != HAL_OK) {
    handle_i2c_error();
  }
}

void configure_accel_range() {
  if (mpu_write_reg(ACCEL_CONFIG, 0x00) != HAL_OK) {
    handle_i2c_error();
  }
}

// Configure gyroscope range to ±250 °/s
void configure_gyro() {
  if (mpu_write_reg(GYRO_CONFIG, 0x00) != HAL_OK) {
    handle_i2c_error();
  }
}

void setup_imu() {
  configure_i2c_gpio();
  configure_i2c_wire_interface();

  wake_mpu();
  configure_gyro();
  configure_accel_range();
}

imu_data_t collect_imu_data(const uint8_t* buf) {
  imu_data_t data;

  data.accel_x = combine_bytes(buf[0], buf[1]);
  data.accel_y = combine_bytes(buf[2], buf[3]);
  data.accel_z = combine_bytes(buf[4], buf[5]);
  data.temp_raw = combine_bytes(buf[6], buf[7]);
  data.gyro_x = combine_bytes(buf[8], buf[9]);
  data.gyro_y = combine_bytes(buf[10], buf[11]);
  data.gyro_z = combine_bytes(buf[12], buf[13]);

  return data;
}

void print_roll_pitch(imu_data_t data) {
  float pitch = calculate_pitch(data.accel_x);
  float roll = calculate_roll(data.accel_y, data.accel_z);

  // Serial.printf("Pitch: %.2f Roll: %.2f\n", pitch, roll);
}

void print_raw_imu_data(imu_data_t data) {
  float temp_celsius = convert_temp(data.temp_raw);

  // Serial.printf(
  //     "accel: [%d %d %d] gyro: [%d %d %d] temp_raw: %d temp_c: %.2fC\n",
  //     data.accel_x, data.accel_y, data.accel_z, data.gyro_x, data.gyro_y,
  //     data.gyro_z, data.temp_raw, temp_celsius);
}

void read_imu_accel_data(void) {
  uint8_t buf[READ_BUF_SIZE] = {0};

  // burst read the mpu6050 data.
  HAL_StatusTypeDef status = HAL_I2C_Mem_Read(
      &i2c_handle_config, MPU_ADDR_8BIT, ACCEL_OUT_H, I2C_MEMADD_SIZE_8BIT, buf,
      READ_BUF_SIZE, I2C_TIMEOUT_MS);

  if (status != HAL_OK) {
    // we drop the current sample if the read fails.
    // the last thing we want to do is hang on reading.
    return;
  }

  imu_data_t data = collect_imu_data(buf);
  print_roll_pitch(data);
}
