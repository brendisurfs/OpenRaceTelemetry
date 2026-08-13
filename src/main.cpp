#include <stm32f411xe.h>

#include "stm32f4xx_hal.h"
#include "stm32f4xx_hal_rcc.h"

#include "blink.h"

static void SystemClock_Config(void) {
  RCC_OscInitTypeDef osc_init{};
  osc_init.HSIState = RCC_HSI_ON;
  osc_init.PLL.PLLState = RCC_PLL_NONE;
  osc_init.OscillatorType = RCC_OSCILLATORTYPE_HSI;
  osc_init.HSICalibrationValue = RCC_HSICALIBRATION_DEFAULT;

  HAL_RCC_OscConfig(&osc_init);

  RCC_ClkInitTypeDef clk_config{};
  clk_config.ClockType = RCC_CLOCKTYPE_SYSCLK | RCC_CLOCKTYPE_HCLK |
                         RCC_CLOCKTYPE_PCLK1 | RCC_CLOCKTYPE_PCLK2;

  clk_config.SYSCLKSource = RCC_SYSCLKSOURCE_HSI;
  clk_config.AHBCLKDivider = RCC_SYSCLK_DIV1;
  clk_config.APB1CLKDivider = RCC_HCLK_DIV1;
  clk_config.APB2CLKDivider = RCC_HCLK_DIV1;

  HAL_RCC_ClockConfig(&clk_config, FLASH_LATENCY_0);
}

/**
 * The vector table's SysTick entry falls back to a weak, empty handler unless
 * we provide this. CubeMX normally generates this in stm32f4xx_it.c;
 * without it, SysTick fires every 1ms but never calls HAL_IncTick(),
 * so uwTick never advances and HAL_Delay() spins forever.
 */
extern "C" void SysTick_Handler(void) {
  HAL_IncTick();
}

int main(void) {
  HAL_Init();
  SystemClock_Config();
  led_gpio_init();
  // When button pressed, change state
  while (1) {
    gps_warmup_blink();
  }
}
