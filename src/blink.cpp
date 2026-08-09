
#include <stm32f411xe.h>
#include <stm32f4xx_hal.h>

#include "stm32f4xx_hal_gpio.h"

#include "blink.h"

#define LED_PIN GPIO_PIN_13
#define LED_PORT GPIOC

void led_gpio_init(void) {
  __HAL_RCC_GPIOC_CLK_ENABLE();

  GPIO_InitTypeDef gpio_config{};
  gpio_config.Pin = LED_PIN;
  gpio_config.Pull = GPIO_NOPULL;
  gpio_config.Mode = GPIO_MODE_OUTPUT_PP;
  gpio_config.Speed = GPIO_SPEED_FREQ_LOW;

  HAL_GPIO_Init(LED_PORT, &gpio_config);
}

void gps_warmup_blink(void) {
  HAL_GPIO_WritePin(LED_PORT, LED_PIN, GPIO_PIN_RESET);
  HAL_Delay(BLINK_WARMUP);
  HAL_GPIO_WritePin(LED_PORT, LED_PIN, GPIO_PIN_SET);
  HAL_Delay(BLINK_WARMUP);
}

void gps_connected_blink(void) {
  int max_flashes = 10;
  for (int i = 0; i < max_flashes; i++) {
    HAL_GPIO_WritePin(LED_PORT, LED_PIN, GPIO_PIN_RESET);
    HAL_Delay(BLINK_SUCCESS);
    HAL_GPIO_WritePin(LED_PORT, LED_PIN, GPIO_PIN_SET);
    HAL_Delay(BLINK_SUCCESS);
  }
}
