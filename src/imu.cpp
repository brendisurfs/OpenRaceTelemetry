#include "imu.h"
#include "blink.h"

#include <cstddef>
#include <cstdio>
#include <cstdlib>

#include <stm32f4xx_hal.h>

#include "imu_math.h"
#include "stm32f411xe.h"
#include "stm32f4xx_hal_cortex.h"
#include "stm32f4xx_hal_def.h"
#include "stm32f4xx_hal_gpio.h"
#include "stm32f4xx_hal_gpio_ex.h"
#include "stm32f4xx_hal_i2c.h"
#include "stm32f4xx_hal_rcc.h"
#include "stm32f4xx_hal_uart.h"

#define MPU_ADDR 0x68
#define PWR_MGMT_REG 0x6B
#define ACCEL_CONFIG 0x1C
#define GYRO_CONFIG 0x1B
#define ACCEL_OUT_H 0x3B

#define READ_BUF_SIZE 14

UART_HandleTypeDef huart1;

void I2C_Error_Handler() {
  while (1) {
    i2c_error_blink();
  }
}

/*
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
  I2C_HandleTypeDef handle{};
  handle.Instance = I2C1;
  handle.Init = init_config;

  HAL_StatusTypeDef status = HAL_I2C_Init(&handle);
  if (status != HAL_OK) {
    I2C_Error_Handler();
  }
}

/*
 * Probes every 7-bit I2C address (0x08-0x77) on bus_handle and logs which
 * ones ACK. Use this to sanity-check wiring before trusting a fixed
 * device address like 0x68/0x69.
 */
void scan_i2c_bus(void) {
  // TODO: implement scan.
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
  MPU_Region_InitTypeDef config{};
  config.Enable = 1;

  // HAL_MPU_ConfigRegion( *MPU_Init)
  Wire.beginTransmission(MPU_ADDR);
  Wire.write(GYRO_CONFIG);  // GYRO_CONFIG register
  Wire.write(0x00);         // ±250dps
  Wire.endTransmission(true);

  uint8_t uart_data[10] = {0};
  uint16_t len_of_msg = sprintf(uart_data, "hey %s\n", "cuh");
  HAL_UART_Transmit(&huart1, uart_data, len_of_msg, 100);
}

void setup_imu() {
  configure_i2c_gpio();
  configure_i2c_wire_interface();

  // wake_mpu();
  // configure_gyro();
  // configure_accel_range();
  // Serial.println("MPU6050 initialized");
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
  Wire.beginTransmission(MPU_ADDR);
  Wire.write(ACCEL_OUT_H);
  Wire.endTransmission(false);

  size_t written_bytes = Wire.requestFrom(MPU_ADDR, READ_BUF_SIZE);
  if (written_bytes == 0) {
    // Serial.println("Read bytes error");
  }

  imu_data_t data = collect_imu_data();
  print_roll_pitch(data);
}
