#include <Arduino.h>

/**
 * Different flashes for different states of the system.
 */
enum FlashDelay {
  //  200ms for warming up
  WARMUP = 200,
  // 20 ms flash timeout
  FLASH_SUCCESS = 20,
};

void configure_led(void) {
  pinMode(LED_BUILTIN, OUTPUT);
}

void blink_led(void) {
  digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
}

/**
 * Blinks at a slower, consistent rate
 * to show that the GPS is warming up.
 */
void gps_warmup_blink(void) {
  blink_led();
  delay(WARMUP);
}

/**
 * Flashes the LED when the GPS has successfully
 * warmed up and connected.
 */
void gps_connected_blink(void) {
  int max_flashes = 10;
  for (int i = 0; i < max_flashes; i++) {
    digitalWrite(LED_BUILTIN, HIGH);
    delay(FLASH_SUCCESS);
    digitalWrite(LED_BUILTIN, LOW);
  }
}

void imu_setup_success(void) {}
