<img width="1280" height="320" alt="OrtGithubImage" src="https://github.com/user-attachments/assets/187a4ebe-e4b7-43f4-907a-d34e5ed48dbb" />


# Open Race Telemetry
Open Race Telemetry is an open-source telemetry system for motorcycle racers.
It lets you build your own telemetry system at a low cost, compared to
commercial systems.

### Status

🚧 In progress. This project is under active development. It is not ready
for use.

The project moved from C++ and PlatformIO on an ESP32 to Rust and Embassy on
an STM32F411CE. The shared types crate and the desktop app build. Their tests
pass. The firmware crate builds too, but many function bodies are not
complete yet. See [Current State](#current-state) for the full detail.

### Overview

Telemetry is important in racing. It lets you map your racing lines and your
speed in corners. It shows you the places where you can push harder. This
data helps you improve your lap times.

The goal of this project is to give racers a telemetry system they can
customize, extend, and adapt for their own needs (an alternative to closed
commercial systems).

This repository holds the software for the Open Race Telemetry project. The
software reads data from an IMU and a GPS receiver on a microcontroller. It
sends the telemetry to a desktop application for display.

### Project Structure

The repository is a Cargo workspace. It has three crates.

| Path | Crate | Purpose |
| --- | --- | --- |
| `ort_types/` | `ort_types` | Data types shared by the firmware and the desktop app. It is `no_std`. It has no hardware access and no sensor math. |
| `firmware/` | `firmware` | Embassy firmware for the STM32F411CE. It builds for the `thumbv7em-none-eabi` target. It produces the `ort_firmware` binary. |
| `desktop/` | `desktop` | Tauri 2 desktop application. The Rust backend is in `desktop/src-tauri/`. The React and TypeScript frontend is in `desktop/src/`. |

The firmware and the desktop app build for different targets. They cannot
build together in one command. A bare `cargo build` command builds only
`ort_types`, because the workspace sets `default-members` to that crate.

#### Shared types

`ort_types` holds the data that crosses the boundary between the device and
the desktop app.

- `ImuData` is one sample set from the MPU6050, in raw register units.
  `ImuData::from_bytes` decodes the 14-byte burst read.
- `NmeaMessage` is the talker code and the message type from an NMEA
  sentence. For example, `$GPGGA,...` gives the talker code `GP` and the
  message type `GGA`. `NmeaMessage::from_bytes` parses the prefix.

Two optional features control the derives. This way, neither consumer pays
for the other. The `defmt` feature is for the firmware and its RTT logging.
The `serde` feature is for the desktop app and its JSON.

The crate stores fixed-size ASCII fields as byte arrays. This format matches
the wire format and the event log. It also keeps the types `Copy` and free
of allocation.

#### Desktop data flow

The Rust backend gives the frontend a different data shape than the wire
format. The data transfer objects (DTOs) in `desktop/src-tauri/src/dto.rs`
convert the byte arrays to strings. This conversion happens one time, at the
IPC boundary. This method keeps `ort_types` `no_std` and free of codegen
concerns. It also makes sure the generated TypeScript matches the JSON that
the backend sends.

- `NmeaMessageDto` converts the ASCII byte arrays to `String`. A corrupt
  frame becomes the character U+FFFD. The IPC call does not fail.
- `ImuDataDto` keeps the `i16` register values. It gives the frontend
  camelCase field names.

`specta` and `tauri-specta` generate the file `desktop/src/bindings.ts` from
the Tauri commands. This generation runs as a test. It does not run at
application start. A test makes CI fail on a stale checked-in file. If the
generation ran at start instead, the application could ship bindings that
disagree with the Rust commands.

A type appears in `bindings.ts` only when a command uses it. The
`imu_sample` command exists now for this reason. It is placeholder wiring.

### Current State

Read this section before you build the project. It tells you what works now.

**Works and is tested**

- `ort_types` builds. Its tests pass. `NmeaMessage::from_bytes` and
  `ImuData::from_bytes` are complete.
- The desktop backend builds. Its 4 tests pass. The DTO conversions are
  complete. `desktop/src/bindings.ts` matches the Rust commands.
- `firmware/src/imu/math.rs` is complete. It holds the math that converts
  raw register values to physical units. It has no hardware access.

**Builds, but is not complete**

- `cargo build -p firmware --target thumbv7em-none-eabi` succeeds. The crate
  compiles.
- Many function bodies use `todo!()` or `defmt::todo!()`. These functions
  will panic if you call them on real hardware. The doc comments above each
  function state the intended behavior. Use them as the specification for
  the work.
  - `firmware/src/blink.rs`: `FlashDelay::duration`, `gps_connected_blink`,
    and `blink_task`.
  - `firmware/src/gps.rs`: `Gps::new`, `Gps::read_sentence`, and
    `setup_gps`.
  - `firmware/src/imu/mod.rs`: `scan_i2c_bus`, `log_roll_pitch`,
    `log_raw_imu_data`, and `imu_task`.
- The frontend in `desktop/src/App.tsx` is the default Tauri and React
  template. It does not call the `parse_nmea` command or the `imu_sample`
  command yet.

### Running

#### Requirements

- [Rust](https://rustup.rs/), with the 2024 edition toolchain.
- The bare-metal target for the firmware:

  ```bash
  rustup target add thumbv7em-none-eabi
  ```

- [`just`](https://github.com/casey/just), the command runner for the
  build, flash, and test recipes.
  - macOS: `brew install just`
  - Linux: `cargo install just`. Or see the [packaging status
    table](https://github.com/casey/just#packages) for your distribution's
    package manager.
  - Windows: `winget install --id Casey.Just` or `scoop install just`
- [`probe-rs`](https://probe.rs/docs/getting-started/installation/), to
  flash the board and to read the defmt logs over RTT.

  ```bash
  cargo install probe-rs-tools
  ```

- An STM32F411CE board (the "Black Pill") and an ST-Link probe.
- [Bun](https://bun.sh/), for the desktop frontend. The repository holds a
  `bun.lock` file.
- The [Tauri system
  dependencies](https://tauri.app/start/prerequisites/) for your operating
  system.

#### Hardware

| Signal | Pin | Notes |
| --- | --- | --- |
| Status LED | PC13 | On the board. It is active-low. A low level turns the LED on. |
| IMU (I2C1) | PB6 = SCL, PB7 = SDA | MPU6050 at address 0x68 with AD0 low, or 0x69 with AD0 high. |
| GPS (USART1) | PA9 = TX, PA10 = RX | TX goes from the MCU to the GPS. RX comes from the GPS to the MCU. The default rate is 9600 baud. |

The I2C lines need external 4.7k pull-up resistors to 3V3. `embassy-stm32`
configures the I2C pins as open-drain. It does not supply pull-ups. The
earlier ESP32 build used the weak internal pull-ups of that MCU (a
difference from the current hardware).

#### Firmware

The firmware crate builds, but many functions are not complete yet. Flashed
firmware built from this code will panic at those `todo!()` calls. See
[Current State](#current-state).

```bash
just build     # cargo build -p firmware --target thumbv7em-none-eabi --release
just run       # build, flash, and stream the defmt logs over RTT
just flash     # build, flash, reset, and detach
just reset     # reset the target without a new flash
just probes    # list the connected probes
```

`just run` is the default recipe.

The release profile keeps debug symbols. `probe-rs` and `defmt` decode
addresses against the symbols. The symbols stay in the ELF file. They are
not part of the flashed image.

The `justfile` does not use `--connect-under-reset`. That option needs the
NRST line between the ST-Link and the board. The usual four-wire connection
(SWDIO, SWCLK, GND, and 3V3) has no reset line. With this option, the
command waits for a reset line that is not present, then times out.

If a flash or an attach operation fails, the target is usually in WFI. It
ignores SWD in this state. Put the chip into the ROM bootloader to stop the
firmware, then flash it again:

1. Hold BOOT0.
2. Tap NRST.
3. Release BOOT0. The LED stops blinking.
4. Run `just flash`.

`just unbrick` prints these steps.

To flash across USB with DFU instead of an ST-Link, put the chip into DFU
mode with the same three button steps. Then run `just dfu`. This recipe
needs `cargo-binutils` and `dfu-util`.

#### Desktop application

```bash
cd desktop
bun install
bun run tauri dev     # start the application in development mode
bun run tauri build   # build a bundle with the `dist` profile
```

Cargo reads `[profile.*]` settings only from the root of the workspace. For
this reason, the `dist` profile is in the root `Cargo.toml`, not in
`desktop/src-tauri/Cargo.toml`.

#### Tests

```bash
just test-types            # cargo test -p ort_types
cargo test -p desktop      # DTO conversions and the TypeScript binding export
```

The desktop test suite writes the file `desktop/src/bindings.ts`. Run this
test after you change a Tauri command or a DTO. Commit the result.

### Contributing

Contributions are welcome. This includes ideas, bug fixes, and features. A
feature or an idea can be refused if it does not agree with the goal of this
project (a stable, robust, open DIY platform for race telemetry). To
contribute, look at the open issues, or open a new issue.

The `todo!()` function bodies listed in [Current State](#current-state) are
a good place to start. Each stub has a doc comment that states the intended
behavior.

### Troubleshooting

#### `probe-rs` cannot attach to the target

The target is usually in WFI. It ignores SWD in this state. Put the chip
into the ROM bootloader with the steps in [Firmware](#firmware). Then flash
it again.

#### `cargo build` builds only `ort_types`

This is correct. The firmware crate builds for `thumbv7em-none-eabi`. The
desktop app builds for the host. These targets cannot build in the same
command. The workspace sets `default-members` to `ort_types`. Give a target
with the `-p` flag to build the other crates.

#### The TypeScript bindings disagree with the Rust commands

Run `cargo test -p desktop`. This test regenerates the file
`desktop/src/bindings.ts`. Commit the file after the test writes it.
