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

static const char* MAIN_TAG = "MAIN";

// Global State for leds
static uint8_t s_led_state = 0;

static void blink_led(void) {
  gpio_set_level(BLINK_GPIO, s_led_state);
  ESP_LOGI(MAIN_TAG, "Setting level: %d\n", s_led_state);
}

/*
 * sets up gpio led
 */
static void configure_led(void) {
  gpio_reset_pin(BLINK_GPIO);
  gpio_set_direction(BLINK_GPIO, GPIO_MODE_OUTPUT);
}

/*
 * ENTRYPOINT
 */
void app_main(void) {
  configure_led();
  setup_imu();

  do {
    ESP_LOGI(MAIN_TAG, "Hello, Nerds");
    blink_led();
    s_led_state = !s_led_state;

    // Read imu data and print
    imu_read_accel_data();

    vTaskDelay(100 / portTICK_PERIOD_MS);
  } while (true);
}
