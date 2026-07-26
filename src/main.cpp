#include <Arduino.h>
#include <TaskScheduler.h>

#include "TaskSchedulerDeclarations.h"
#include "Wire.h"
#include "blink.h"
#include "gps.h"
#include "imu.h"

Scheduler runner;

Task task_blink_led(400, TASK_FOREVER, &blink_led);

// Task task_read_imu(IMU_READ_INTERVAL, TASK_FOREVER, &read_imu_accel_data);
Task task_read_gps(400, TASK_FOREVER, &read_gps_message);

void setup() {
  Wire.begin();

  // A baud rate of 115200 seems to work well.
  Serial.begin(115200);

  // Set up our peripherals and LED to blink
  configure_led();
  setup_imu();
  setup_gps();

  // add our tasks here
  // runner.addTask(task_read_imu);
  runner.addTask(task_blink_led);
  runner.addTask(task_read_gps);

  // Make sure to enable all the tasks we want.
  task_blink_led.enable();
  // task_read_imu.enable();
  task_read_gps.enable();
}

void loop() {
  runner.execute();
}
