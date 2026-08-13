bin := "target/thumbv7em-none-eabi/release/embassy-fun"
chip := "STM32F411CE"

# NOTE: --connect-under-reset is deliberately NOT used here. It requires NRST to
# be wired from the ST-Link to the board; with the common 4-wire hookup
# (SWDIO/SWCLK/GND/3V3) it just times out waiting for a reset line that isn't
# there. If you ever wire NRST, add it to probe_opts to make attaching to a
# sleeping (WFI) target reliable.
probe_opts := "--chip " + chip

# Default: build, flash, reset, and stream defmt logs over RTT
default: run

# Flash + RTT log via ST-Link/SWD (stays attached; ctrl-C to detach)
run:
  cargo run --release

# Build, flash via ST-Link/SWD, reset, and detach (no RTT session)
flash:
  cargo build --release
  probe-rs download {{probe_opts}} {{bin}}
  probe-rs reset {{probe_opts}}

# Reset the target without reflashing
reset:
  probe-rs reset {{probe_opts}}

# If a flash/attach fails, the target is usually stuck in WFI and ignoring SWD.
# Enter the ROM bootloader (hold BOOT0, tap NRST, release BOOT0) so your
# firmware isn't running, then re-run `just flash`.
unbrick:
  @echo "Hold BOOT0, tap NRST, release BOOT0 (LED should stop blinking), then:"
  @echo "  just flash"

# Flash via USB DFU bootloader (put chip in DFU mode first: hold BOOT0, tap NRST, release BOOT0)
dfu:
  cargo build --release
  cargo objcopy --release -- -O binary {{bin}}.bin
  dfu-util -a 0 -s 0x08000000:leave -D {{bin}}.bin -d 0483:df11

probes:
  probe-rs list

build:
  cargo build --release

clean:
  cargo clean

# build:
#     pio run -e esp32dev
#
# upload:
#     pio run -t upload
#
# upload-and-monitor:
#     pio run -e esp32dev -t upload -t monitor
#
#
# monitor: 
#   pio run -t monitor
#
# # Builds and links to the hardware.
# test-esp32:
#     pio test -e esp32dev
#
# # Tests native utility functions
# test-native:
#     pio test -e native
