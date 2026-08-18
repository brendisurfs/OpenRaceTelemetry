pkg := "firmware"
target := "thumbv7em-none-eabi"
bin := "target/" + target + "/release/ort_firmware"
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
run: build-firmware
    probe-rs run {{ probe_opts }} {{ bin }}

# Build, flash via ST-Link/SWD, reset, and detach (no RTT session)
flash: build-firmware
    probe-rs download {{ probe_opts }} {{ bin }}
    probe-rs reset {{ probe_opts }}

# Reset the target without reflashing
reset:
    probe-rs reset {{ probe_opts }}

# If a flash/attach fails, the target is usually stuck in WFI and ignoring SWD.
# Enter the ROM bootloader (hold BOOT0, tap NRST, release BOOT0) so your
# firmware isn't running, then re-run `just flash`.
unbrick:
    @echo "Hold BOOT0, tap NRST, release BOOT0 (LED should stop blinking), then:"
    @echo "  just flash"

# Flash via USB DFU bootloader (put chip in DFU mode first: hold BOOT0, tap NRST, release BOOT0)
dfu: build-firmware
    cargo objcopy -p {{ pkg }} --target {{ target }} --release -- -O binary {{ bin }}.bin
    dfu-util -a 0 -s 0x08000000:leave -D {{ bin }}.bin -d 0483:df11

probes:
    probe-rs list

clean:
    cargo clean

coverage:
    cargo llvm-cov

test-types:
    cargo test -p ort_types -- --no-capture


build-firmware:
    cargo build -p {{ pkg }} --target {{ target }} --release

check-firmware:
    cargo check -p firmware --target thumbv7em-none-eabi

# --- DESKTOP ----------
#
build-desktop:
    cargo build -p desktop --release

generate-bindings:
    cargo test -p desktop export_typescript_bindings
