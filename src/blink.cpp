#include <Arduino.h>

#include "hal/gpio_types.h"
#include "io_pin_remap.h"
#include "pins_arduino.h"

void configure_led(void) { pinMode(LED_BUILTIN, GPIO_MODE_OUTPUT); }

void blink_led(void) { digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN)); }
