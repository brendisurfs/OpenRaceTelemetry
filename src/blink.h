/**
 * Different flashes for different states of the system.
 */
enum FlashDelay {
  //  200ms for warming up
  BLINK_WARMUP = 200,
  // 20 ms flash timeout
  BLINK_SUCCESS = 20,
  BLINK_ERROR = 100,
  BLINK_WAIT = 1000,
};

/*
 * sets up gpio led
 */
void led_gpio_init(void);

// /**
//  * Blinks at a slower, consistent rate
//  * to show that the GPS is warming up.
//  */
void gps_warmup_blink(void);

// /**
//  * Flashes the LED when the GPS has successfully
//  * warmed up and connected.
//  */
void gps_connected_blink(void);

void i2c_error_blink(void);
