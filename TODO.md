# TODO: STM32Cube HAL Migration

## Current Status

The environment `[env:genericSTM32F411CE]` uses `framework = stm32cube`.
Before this change, the environment used `framework = arduino`.
The environment name did not change. The board did not change.
There is no separate `_hal` environment.

The HAL package `framework-stm32cubef4` is installed.

The build fails at `imu.cpp`.
The build will also fail at `blink.cpp` and `main.cpp` after the `imu.cpp` fix.
The failures occur because these files call Arduino APIs.
The Arduino APIs include `pinMode`, `Wire`, and `Serial`.
These APIs do not exist under `framework = stm32cube`.
This failure is expected.
The failure matches the migration order in this document.

The environments `esp32dev` and `native` are not changed.

### Compiler Database

The file `compile_commands.json` supplies data to clangd for code completion.
This file now works correctly for the HAL headers.

Follow this procedure to regenerate the file for the STM32 environment.

1. Run the command `just compiledb-stm32`.

Follow this procedure to regenerate the file for another environment.

1. Run the command `just compiledb-esp32` for the ESP32 environment.
2. Run the command `just compiledb-native` for the native environment.

Note: PlatformIO overwrites the compiler database file. PlatformIO does not
merge the compiler database file across environments. Regenerate the file
after you switch environments.

The `.clangd` configuration file no longer sets a fixed xtensa `Compiler:`
override. This override previously forced every file in the database
through the ESP32 toolchain. This was incorrect for files compiled under
other environments. This was the cause of unresolved STM32 and HAL headers.

### Purpose of This Migration

The Arduino layer supplies `Wire`, `Serial`, `pinMode`, `digitalWrite`, and
`TaskScheduler`. This layer is a compatibility shim over the ST HAL.
The shim is useful to start a project quickly.
The shim hides register-level control of the hardware.
Register-level control includes clock trees, low-power modes, DMA, and
interrupt priorities.
Register-level control is the goal of this migration away from the ESP32
platform.

This migration does not need CubeIDE or CubeMX.
The PlatformIO `stm32cube` framework supplies the HAL and LL driver source
files directly.
CubeMX is a pin-mux graphical tool.
You may add CubeMX later as an optional tool to view pin assignments.
CubeMX is not required to write or build HAL code.

## Reasons for This Migration

- **Power efficiency**: The Arduino function `delay()` blocks the processor
  in a wait loop. Low-power operation requires the functions
  `HAL_PWR_EnterSTOPMode` and `HAL_PWR_EnterSLEEPMode`. Low-power operation
  also requires RTC and EXTI wakeup sources. The Arduino core does not
  expose these functions.
- **Direct clock control**: The function `SystemClock_Config()` lets you set
  exact PLL and prescaler values. The STM32duino default clock tree does not
  allow this control. Exact clock values are important for I2C and UART
  timing precision. Exact clock values are also important for power draw.
- **Learning value**: This project has a goal to teach hardware-level
  programming.

## Migration Order

Do not rewrite all files at one time.
Port one file at a time.
Keep the build working after each step.

- [x] **Change the framework setting.**
  Change `[env:genericSTM32F411CE]` from `framework = arduino` to
  `framework = stm32cube` in the existing environment.

  Note: An alternative approach was to create a separate `_hal` environment.
  This would keep the Arduino build working as a fallback during the port.
  This approach was rejected. The direct change is simpler.
  You can restore the Arduino environment from git history if needed.
  The build is expected to fail until `blink.cpp`, `main.cpp`, and
  `imu.cpp` are ported. See the steps below.

- [x] **Prove the HAL toolchain with a blink test.**
  Write a minimal blink program directly in `main.cpp`.
  The test uses pin PC13, the onboard LED on the WeAct BlackPill board.
  The test uses a real `main()` function, `HAL_Init()`, a hand-written
  `SystemClock_Config()` function, and the functions `HAL_GPIO_Init`,
  `HAL_GPIO_WritePin`, and `HAL_Delay`.
  The current `SystemClock_Config()` function uses the HSI clock source.
  The current `SystemClock_Config()` function does not use the PLL.
  See the follow-up task below for the PLL configuration.

  This test was flashed to the board through USB DFU.
  The test confirmed the LED blinks correctly.
  SWD wiring is not soldered yet.

  The files `blink.cpp` and `blink.h` are not ported yet.
  The build excludes these files.
  These files still call the Arduino functions `pinMode`, `digitalWrite`,
  and `delay`, and the Arduino constant `LOW`.

  Two decisions remain open:
  1. Decide how to port `blink.cpp`.
  2. Decide if `main.cpp` calls `blink.cpp`, or if `main.cpp` controls the
     LED directly.

- [ ] **Port `main.cpp`.**
  The current minimal version runs the LED loop directly inside `main()`.

  Two decisions remain open:
  1. Decide whether to use `TaskScheduler`. `TaskScheduler` is a pure C++
     library. `TaskScheduler` does not depend on Arduino. `TaskScheduler`
     may work without changes.
  2. Alternatively, write a scheduler based on the function `HAL_GetTick()`
     to replace the cooperative loop.

  Also complete the real `SystemClock_Config()` function with PLL settings.
  The board's rated clock speed is 100MHz.
  The current configuration uses the HSI clock at 16MHz.
  This is a deliberate, temporary choice. See the Gotchas section below.

- [ ] **Port `imu.cpp`.**
  Replace the `Wire` library calls with the functions `HAL_I2C_Init` and
  `HAL_I2C_Master_Transmit`, or the function `HAL_I2C_Master_Receive`.

  Alternatively, use the functions `HAL_I2C_Mem_Write` and
  `HAL_I2C_Mem_Read`. These functions match the existing MPU6050 register
  access pattern more closely.

  This step requires a configured `I2C_HandleTypeDef` structure for the
  chosen I2C peripheral. Most F411 boards use the I2C1 peripheral.
  This step also requires GPIO alternate function setup for the SDA and
  SCL pins.

- [ ] **Port `gps.cpp`.**
  This file currently contains only NMEA string parsing logic.
  This file has no Arduino dependencies.
  Port this file with minimal changes.

  If you add direct UART reads later, use the function `HAL_UART_Init`.
  Also use the function `HAL_UART_Receive`, or use interrupt mode, or use
  DMA mode.

- [ ] **Replace `Serial.printf` debug output.**
  Choose one of two methods.
  1. Use HAL UART through a dedicated debug port.
  2. Use ITM or SWO trace through the ST-Link debugger. This method needs
     no extra UART wire. Retarget `printf` output through a `_write()`
     function that calls `ITM_SendChar`.

  This task has high learning value.

- [ ] **Finalize the environment structure.**
  After the `stm32cube` environment is stable, decide whether to remove
  the Arduino environment or keep both environments.
  If the project keeps ESP32 and STM32 support together, keep both
  environments.
  If you keep both environments, also decide whether the ESP32 environment
  should stop depending on Arduino, or whether the ESP32 board stays on
  Arduino only.

- [ ] **Close this migration.**
  Delete this document after the HAL environment is the default and stable
  environment.
  Move any long-term relevant notes from the Gotchas section into
  `CLAUDE.md`.

## Gotchas

- **Name collisions with CMSIS headers.**
  The header file `stm32f4xx.h` defines the symbol `SUCCESS` as part of the
  `ErrorStatus` enum. A local enum or macro with the name `SUCCESS` will
  cause a naming collision. The file `blink.cpp` had this collision.
  The symbol `FlashDelay::SUCCESS` was renamed to `FLASH_SUCCESS` to fix
  this. The header files also claim the symbols `ERROR`, `OK`, `RESET`, and
  `SET`. Check new code against `stm32cube` for the same type of collision.

- **`Wire.setPins()` is not portable.**
  The function `Wire.setPins(sda, scl)` is an ESP32-Arduino-core extension.
  This function is not part of the standard Arduino API.
  STM32duino uses the functions `Wire.setSDA()` and `Wire.setSCL()` before
  the function `Wire.begin()`. Alternatively, call `Wire.begin()` alone to
  use the board variant's default pins.
  This issue is fixed in `imu.cpp`. The ESP32-specific pin override was
  removed.

- **ESP32-only headers caused build failures.**
  The files `blink.cpp` and `imu.cpp` included the headers
  `esp32-hal-gpio.h`, `io_pin_remap.h`, `pins_arduino.h`, and
  `hal/gpio_types.h` without a guard condition.
  These headers caused no error under `framework = arduino` on the ESP32
  target.
  These headers caused a compile failure on the STM32 target.
  These headers were removed. The code did not use any symbols from these
  headers. The standard `Arduino.h` header supplies the function `pinMode`
  and similar functions.

- **`SysTick_Handler` is not supplied by the HAL package.**
  The CubeMX tool normally generates the function `SysTick_Handler` inside
  the file `stm32f4xx_it.c`. This project does not have that file.

  The startup assembly file `startup_stm32f411xe.s` declares
  `SysTick_Handler` as a weak symbol.
  Without an override, the linker uses an empty default function.
  The interrupt fires every 1 millisecond. The empty default function
  performs no action.

  The function `HAL_Delay()` reads the global counter variable `uwTick`.
  Only the function `HAL_IncTick()` updates this counter.
  Normally, `SysTick_Handler` calls `HAL_IncTick()`.

  Effect: The functions `HAL_Init()` and `HAL_GPIO_WritePin()` work
  correctly. The first call to `HAL_Delay()` stops and does not return.
  Symptom: the LED turns on, then stays on with no further change. The
  program does not crash.

  Fix: Add this function definition to the build. The current location is
  `main.cpp`.
  ```
  extern "C" void SysTick_Handler(void) { HAL_IncTick(); }
  ```
  The declaration `extern "C"` is required. The vector table links against
  the unmangled C symbol name.

  Expect the same type of gap for other weak default handlers.
  Examples include `HardFault_Handler` and other peripheral IRQ handlers.
  CubeMX would normally generate real handler functions for these.
  Without CubeMX, you must write these handler functions by hand.
  Alternatively, accept the weak default handler where safe.
  A weak default handler is acceptable for `HardFault_Handler` during early
  bring-up.
  A weak default handler is not acceptable for a handler you expect to fire
  regularly, for example a UART or I2C interrupt handler.

- **Board flashing currently uses USB DFU.**
  The board bring-up procedure used USB DFU. USB DFU uses the tool
  `dfu-util` and the ROM bootloader.
  SWD and ST-Link are not used yet. SWD pins are not soldered yet.

  Follow this procedure to enter DFU mode.
  1. Hold the BOOT0 button.
  2. Tap the reset button.

  The device reports the USB VID:PID `0483:df11`.
  The internal flash memory uses DFU alternate setting 0.
  The internal flash memory base address is `0x08000000`.

  The command `dfu-util -l` may list more than one DFU device.
  This occurs when other USB DFU peripherals are connected, for example an
  audio interface.
  Use the `-S <serial>` flag with the serial number from `dfu-util -l` to
  select the correct device.

  Use this command to flash the firmware.
  ```
  dfu-util -d 0483:df11 -S <serial> -a 0 -s 0x08000000:leave -D firmware.bin
  ```

  The file `platformio.ini` still sets `upload_protocol = stlink` for the
  planned workflow after SWD wiring is complete.
  The DFU flashing procedure is a manual, one-time step.
  The DFU flashing procedure is not connected to the command
  `pio run -t upload`.

- **The onboard LED uses active-low logic.**
  The WeAct BlackPill board connects the onboard LED cathode to pin PC13.
  This is an active-low connection.
  The value `GPIO_PIN_RESET` turns the LED on.
  The value `GPIO_PIN_SET` turns the LED off.

  This is the opposite of the Arduino convention. The Arduino convention
  uses the constant `LED_BUILTIN` with the value `HIGH` to turn a LED on.
  The file `blink.cpp` currently assumes the Arduino convention.
  Check the LED polarity when you port `blink.cpp`.
