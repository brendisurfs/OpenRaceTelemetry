#include "driver/gpio.h"
#include "esp_log.h"
#include "freertos/idf_additions.h"
#include "hal/gpio_types.h"
#include "imu.c"
#include "portmacro.h"

#ifdef CONFIG_BLINK_GPIO
#define BLINK_GPIO CONFIG_BLINK_GPIO
#else
#define BLINK_GPIO 48
#endif

static const char* BLINK_TAG = "BLINK";

// Global State for leds
static uint8_t s_led_state = 0;

static void blink_led(void) {
  gpio_set_level(BLINK_GPIO, s_led_state);
  ESP_LOGI(BLINK_TAG, "Setting level: %d\n", s_led_state);
}

/*
 * sets up gpio led
 */
static void configure_led(void) {
  gpio_reset_pin(BLINK_GPIO);
  gpio_set_direction(BLINK_GPIO, GPIO_MODE_OUTPUT);
}

void run_blink_loop(void) {
  configure_led();

  do {
    ESP_LOGI(BLINK_TAG, "Hello, Nerds");
    blink_led();
    s_led_state = !s_led_state;

    // Read imu data and print
    imu_read_accel_data();

    vTaskDelay(100 / portTICK_PERIOD_MS);
  } while (true);
}
