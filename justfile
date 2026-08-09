# Regenerate compile_commands.json for the STM32 env (clangd completions
# for HAL headers). PlatformIO overwrites this file per invocation rather
# than merging across envs, so re-run the matching recipe when you switch
# which board you're actively editing.
compiledb-stm32:
    pio run -e genericSTM32F411CE -t compiledb

compiledb-esp32:
    pio run -e esp32dev -t compiledb

compiledb-native:
    pio run -e native -t compiledb

build:
    pio run -e esp32dev

upload:
    pio run -t upload

upload-and-monitor:
    pio run -e esp32dev -t upload -t monitor

monitor:
    pio run -t monitor

# Builds and links to the hardware.
test-esp32:
    pio test -e esp32dev

# Tests native utility functions
test-native:
    pio test -e native


bin := ".pio/build/genericSTM32F411CE/firmware.bin"

upload-stm32: 
  dfu-util -a 0 -s 0x08000000:leave -D {{bin}} -d 0483:df11

