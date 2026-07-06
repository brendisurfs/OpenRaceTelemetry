#include <Arduino.h>

#include "USBCDC.h"
#include "blink.h"
#include "imu.h"

void setup() {
  Serial.begin(115200);
  configure_led();
  configure_i2c_wire_interface();
}

void loop() {
  blink_led();
  read_imu_accel_data();
}
