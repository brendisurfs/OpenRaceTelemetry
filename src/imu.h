#include <cstdint>

/**
 * 20ms interval
 * ms = 1000 / Hz
 */
static const unsigned long IMU_READ_INTERVAL = 20;

typedef struct ImuData {
  int16_t accel_x;
  int16_t accel_y;
  int16_t accel_z;
  int16_t temp_raw;
  int16_t gyro_x;
  int16_t gyro_y;
  int16_t gyro_z;
} imu_data_t;

void configure_i2c_wire_interface();

/*
 * Probes every 7-bit I2C address (0x08-0x77) on bus_handle and logs which
 * ones ACK. Use this to sanity-check wiring before trusting a fixed
 * device address like 0x68/0x69.
 */
void scan_i2c_bus();

// Sets up the IMU interface
void setup_imu();

void read_imu_accel_data(void);
