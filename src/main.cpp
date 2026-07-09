#include <Arduino.h>
#include <sys/unistd.h>

#include "Wire.h"
#include "blink.h"
#include "esp32-hal-gpio.h"
#include "esp32-hal.h"
#include "imu.h"

void setup() {
  Wire.begin();
  Serial.begin(9600);

  Serial.println("Starting up config sequence");
  configure_led();
  setup_imu();
}

void loop() {
  digitalWrite(LED_BUILTIN, LOW);
  read_imu_accel_data();
  digitalWrite(LED_BUILTIN, HIGH);
  delay(100);
}
