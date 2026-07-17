#include <Arduino.h>
#include <TaskScheduler.h>

#include "Wire.h"
#include "blink.h"
#include "imu.h"

Scheduler runner;

Task task_blink_led(400, TASK_FOREVER, &blink_led);

Task task_read_imu(IMU_READ_INTERVAL, TASK_FOREVER, &read_imu_accel_data);

void setup() {
  Wire.begin();

  // A baud rate of 115200 seems to work well.
  Serial.begin(115200);

  // Set up our peripherals and LED to blink
  configure_led();
  setup_imu();

  // add our tasks here
  runner.addTask(task_read_imu);
  runner.addTask(task_blink_led);

  // Make sure to enable all the tasks we want.
  task_blink_led.enable();
  task_read_imu.enable();
}

void loop() {
  runner.execute();
}
