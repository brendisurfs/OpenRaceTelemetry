#include <Wire.h>

void setup_wire_protocol() {}
// #include "driver/i2c_master.h"
// #include "driver/i2c_types.h"
// #include "esp_err.h"
// #include "esp_log.h"
// #include "soc/clk_tree_defs.h"
// #include "soc/gpio_num.h"

// static const char* IMU_TAG = "IMU";

// /* Persisted so imu_read_accel_data() can reuse it. Revisit the API shape
//  * (avoid the file-scope global) tomorrow. */
// static i2c_master_dev_handle_t s_imu_dev_handle;

// /*
//  * Probes every 7-bit I2C address (0x08-0x77) on bus_handle and logs which
//  * ones ACK. Use this to sanity-check wiring before trusting a fixed
//  * device address like 0x68/0x69.
//  */
// static void imu_i2c_scan(i2c_master_bus_handle_t bus_handle) {
//   ESP_LOGI(IMU_TAG, "Scanning I2C bus...");
//   int found = 0;
//   for (uint8_t addr = 0x08; addr <= 0x77; addr++) {
//     esp_err_t err = i2c_master_probe(bus_handle, addr, 50);
//     if (err == ESP_OK) {
//       ESP_LOGI(IMU_TAG, "Found device at 0x%02X", addr);
//       found++;
//     }
//   }
//   if (found == 0) {
//     ESP_LOGW(IMU_TAG, "No I2C devices found on the bus");
//   }
// }

// /*
//  * Sets up the IMU to be read by creating a new master bus,
//  * scanning to make sure our device is properly wired,
//  * waking it up
//  */
// static void setup_imu(void) {
//   i2c_master_bus_config_t config = {.i2c_port = -1,
//                                     .sda_io_num = GPIO_NUM_11,
//                                     .scl_io_num = GPIO_NUM_12,
//                                     .clk_source = I2C_CLK_SRC_DEFAULT,
//                                     .glitch_ignore_cnt = 7,
//                                     .intr_priority = 0,
//                                     .trans_queue_depth = 0,
//                                     .flags.enable_internal_pullup = true};

//   i2c_master_bus_handle_t bus_handle;

//   esp_err_t err_code = i2c_new_master_bus(&config, &bus_handle);
//   if (err_code) {
//     ESP_LOGE(IMU_TAG, "Error setting master bus: %s\n",
//              esp_err_to_name(err_code));
//     return;
//   }

//   imu_i2c_scan(bus_handle);

//   /*Add the IMU device to the master bus handle */
//   i2c_device_config_t dev_config = {
//       .device_address = 0x68,
//       .scl_speed_hz = 400000,
//   };

//   esp_err_t add_device_err =
//       i2c_master_bus_add_device(bus_handle, &dev_config, &s_imu_dev_handle);

//   if (add_device_err) {
//     ESP_LOGE(IMU_TAG, "Error adding device: %s\n",
//              esp_err_to_name(add_device_err));
//     return;
//   }

//   /*
//    * MPU6050 boots into sleep mode; wake it by clearing PWR_MGMT_1
//    */
//   uint8_t wake_buffer[] = {0x6B, 0x00};
//   esp_err_t wake_err = i2c_master_transmit(s_imu_dev_handle, wake_buffer,
//                                            sizeof(wake_buffer), 500);

//   if (wake_err) {
//     ESP_LOGE(IMU_TAG, "Wake error: %s\n", esp_err_to_name(wake_err));
//   }

//   ESP_ERROR_CHECK(wake_err);
// }

// /*
//  * Converts the temperature to Celsius
//  */
// static float convert_temp(int16_t temp_raw) {
//   return temp_raw / 340.0f + 36.53f;
// }

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
