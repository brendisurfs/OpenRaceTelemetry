build:
  idf.py build

upload:
  pio run -t upload

upload-and-monitor:
    pio run -t upload -t monitor

test-native: 
  pio test -e native
