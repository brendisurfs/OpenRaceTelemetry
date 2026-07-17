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
