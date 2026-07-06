#include <Arduino.h>

#include "blink.h"

// Setup function for anything we need
void setup() {
  Serial.begin(115200);
  configure_led();
}

void loop() { blink_da_led(); }
