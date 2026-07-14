#include <Arduino.h>
#include <TaskScheduler.h>

#include "Wire.h"
#include "blink.h"
#include "imu.h"

Scheduler runner;

Task task_read_imu(100, TASK_FOREVER, &read_imu_accel_data);
Task task_blink_led(500, TASK_FOREVER, &blink_led);

void setup() {
  Wire.begin();
  Serial.begin(9600);

  configure_led();
  setup_imu();

  // add our tasks here
  runner.addTask(task_read_imu);
  runner.addTask(task_blink_led);

  // Make sure to enable all the tasks we want.
  task_blink_led.enable();
  task_read_imu.enable();
}

void loop() { runner.execute(); }
