#include <Arduino.h>

#include "esp32-hal-gpio.h"
#include "hal/gpio_types.h"
#include "io_pin_remap.h"
#include "pins_arduino.h"

void blink_da_led(void) {
  digitalWrite(LED_BUILTIN, HIGH);
  delay(200 / portTICK_PERIOD_MS);

  digitalWrite(LED_BUILTIN, LOW);
  delay(200 / portTICK_PERIOD_MS);
}

void configure_led(void) { pinMode(LED_BUILTIN, GPIO_MODE_OUTPUT); }
