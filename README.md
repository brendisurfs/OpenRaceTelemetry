<img width="1280" height="320" alt="OrtGithubImage" src="https://github.com/user-attachments/assets/187a4ebe-e4b7-43f4-907a-d34e5ed48dbb" />


# Open Race Telemetry
An open-source telemetry system for motorcycle racers,
enabling people to build their own telemetry systems
for a fraction of the price of commercial systems.

### Status

🚧 In Progress. This project is under active development and is not ready for use.

The project moved from C++ and PlatformIO on an ESP32 to Rust and Embassy on an
STM32F411CE. The shared types and the desktop app build and pass their tests.
The firmware does not compile yet. See [Current State](#current-state) for the
detail.

### Overview

Telemetry in racing is important for many reasons. It lets you map your racing
lines, your speed in corners, and the places where you can push harder. These
all help you to improve lap times.

I wanted to build something that I would use. I did not want to buy a system
that I cannot customize, extend, and adapt for my needs. I like to build things,
and I like to explore how things work inside.

This repository holds the software for the Open Race Telemetry project. The
software reads an IMU and a GPS receiver on the microcontroller, and shows the
telemetry in a desktop application.

### Project Structure

The repository is a Cargo workspace with three crates.

| Path | Crate | Purpose |
| --- | --- | --- |
| `ort_types/` | `ort_types` | Data types shared by the firmware and the desktop app. `no_std`, with no hardware access and no sensor math. |
| `firmware/` | `firmware` | Embassy firmware for the STM32F411CE. Builds for the `thumbv7em-none-eabi` target and produces the `ort_firmware` binary. |
| `desktop/` | `desktop` | Tauri 2 desktop application. Rust backend in `desktop/src-tauri/`, React and TypeScript frontend in `desktop/src/`. |

The firmware and the desktop app build for different targets, so they cannot
build together. A bare `cargo build` builds only `ort_types`, because the
workspace sets `default-members` to that crate.

#### Shared types

`ort_types` holds the data that crosses the boundary between the device and the
desktop app.

- `ImuData` is one sample set from the MPU6050, in raw register units.
  `ImuData::from_bytes` decodes the 14-byte burst read.
- `NmeaMessage` is the talker and message type from an NMEA sentence. For
  example, `$GPGGA,...` gives the talker `GP` and the message type `GGA`.
  `NmeaMessage::from_bytes` parses the prefix.

Two optional features control the derives, so that neither consumer pays for the
other. The `defmt` feature is for the firmware and its RTT logging. The `serde`
feature is for the desktop app and its JSON.

The crate keeps fixed-size ASCII fields as byte arrays. This shape is correct
for the wire format and the event log, and it keeps the types `Copy` and free of
allocation.

#### Desktop data flow

The Rust backend gives the frontend a different shape than the wire format. The
data transfer objects (DTOs) in `desktop/src-tauri/src/dto.rs` convert the byte
arrays to strings one time, at the IPC boundary. This keeps `ort_types` `no_std`
and free of codegen concerns, and it makes sure that the generated TypeScript
describes the JSON that the backend really sends.

- `NmeaMessageDto` converts the ASCII byte arrays to `String`. A corrupt frame
  becomes U+FFFD instead of a failed IPC call.
- `ImuDataDto` keeps the `i16` register values and gives the frontend camelCase
  field names.

`specta` and `tauri-specta` generate `desktop/src/bindings.ts` from the Tauri
commands. The generation runs as a test, and not at application start. A test
makes CI fail on a stale checked-in file. If the generation ran at start, the
application could ship bindings that disagree with the Rust commands.

A type gets into `bindings.ts` only if a command mentions it. The `imu_sample`
command exists now for this reason, and it is placeholder wiring.

### Current State

Use this section to know what works before you build.

**Works and is tested**

- `ort_types` builds and its tests pass. `NmeaMessage::from_bytes` and
  `ImuData::from_bytes` are complete.
- The desktop backend builds and its 4 tests pass. The DTO conversions are
  complete, and `desktop/src/bindings.ts` agrees with the Rust commands.
- `firmware/src/imu_math.rs` is complete. It holds the math that converts raw
  register values to physical units, and it has no hardware access.

**Does not work**

- The firmware does not compile. `cargo build -p firmware` gives 8 errors.
  - `firmware/src/imu.rs` uses `I2c<'d, Async>`, but `I2c` in
    `embassy-stm32` 0.6.0 takes two generic parameters, `M: Mode` and
    `IM: MasterMode`.
  - `firmware/src/main.rs` gives `peri.PC13` to `Led::new`, which asks for
    `impl Pin`. Version 0.6.0 wraps the peripheral in `Peri<'_, PC13>`, and that
    wrapper does not implement `Pin`.
  - The `bind_interrupts!` block in `firmware/src/main.rs` does not bind the DMA
    stream interrupts that `I2c::new` and `Uart::new` ask for. The missing
    bindings are `DMA1_STREAM6`, `DMA1_STREAM0`, `DMA2_STREAM7`, and
    `DMA2_STREAM2`.
- The driver bodies in `firmware/src/blink.rs`, `firmware/src/gps.rs`, and
  `firmware/src/imu.rs` are `todo!()`. The doc comments on those functions state
  the intended behavior and are the specification for the work.
- The frontend in `desktop/src/App.tsx` is the default Tauri and React template.
  It does not call `parse_nmea` or `imu_sample` yet.

### Running

#### Requirements

- [Rust](https://rustup.rs/) with the 2024 edition toolchain.
- The bare-metal target for the firmware:

  ```bash
  rustup target add thumbv7em-none-eabi
  ```

- [`just`](https://github.com/casey/just), the command runner for the build,
  flash, and test recipes.
  - macOS: `brew install just`
  - Linux: `cargo install just`, or see the [packaging status table](https://github.com/casey/just#packages) for the package manager of your distribution.
  - Windows: `winget install --id Casey.Just` or `scoop install just`
- [`probe-rs`](https://probe.rs/docs/getting-started/installation/) to flash the
  board and to read the defmt logs over RTT.

  ```bash
  cargo install probe-rs-tools
  ```

- An STM32F411CE board (the "Black Pill") and an ST-Link probe.
- [Bun](https://bun.sh/) for the desktop frontend. The repository holds a
  `bun.lock` file.
- The [Tauri system dependencies](https://tauri.app/start/prerequisites/) for
  your operating system.

#### Hardware

| Signal | Pin | Notes |
| --- | --- | --- |
| Status LED | PC13 | On the board. Active-low, so a low level turns the LED on. |
| IMU (I2C1) | PB6 = SCL, PB7 = SDA | MPU6050 at address 0x68 with AD0 low, or 0x69 with AD0 high. |
| GPS (USART1) | PA9 = TX, PA10 = RX | TX goes from the MCU to the GPS. RX comes from the GPS to the MCU. Default rate is 9600 baud. |

The I2C lines need external 4.7k pull-up resistors to 3V3. `embassy-stm32`
configures the I2C pins as open-drain and does not supply pull-ups. The earlier
ESP32 build used the weak internal pull-ups of that MCU, so this is a change
from the previous hardware.

#### Firmware

The firmware does not compile yet, so these recipes fail today. See
[Current State](#current-state).

```bash
just build     # cargo build -p firmware --target thumbv7em-none-eabi --release
just run       # build, flash, and stream the defmt logs over RTT
just flash     # build, flash, reset, and detach
just reset     # reset the target without a new flash
just probes    # list the connected probes
```

`just run` is the default recipe.

The release profile keeps debug symbols. `probe-rs` and `defmt` decode addresses
against the symbols, and the symbols stay in the ELF file and are not part of
the flashed image.

The `justfile` does not use `--connect-under-reset`. That option needs the NRST
line between the ST-Link and the board. With the usual four-wire connection of
SWDIO, SWCLK, GND, and 3V3, the option waits for a reset line that is not
present, and then it times out.

If a flash or an attach operation fails, the target is usually in WFI and
ignores SWD. Put the chip into the ROM bootloader to stop your firmware, and
then flash it again:

1. Hold BOOT0.
2. Tap NRST.
3. Release BOOT0. The LED stops to blink.
4. Run `just flash`.

`just unbrick` prints these steps.

To flash across USB with DFU instead of an ST-Link, put the chip into DFU mode
with the same three button steps, and then run `just dfu`. This recipe needs
`cargo-binutils` and `dfu-util`.

#### Desktop application

```bash
cd desktop
bun install
bun run tauri dev     # start the application in development mode
bun run tauri build   # build a bundle with the `dist` profile
```

Cargo reads `[profile.*]` only in the root of the workspace, so the `dist`
profile is in the root `Cargo.toml` and not in `desktop/src-tauri/Cargo.toml`.

#### Tests

```bash
just test-types            # cargo test -p ort_types
cargo test -p desktop      # DTO conversions and the TypeScript binding export
```

The desktop test suite writes `desktop/src/bindings.ts`. Run it after you change
a Tauri command or a DTO, and commit the result.

### Contributing

Contributions, such as ideas, bug fixes, and features, are always welcome.
But note that a feature or an idea can be refused if it does not agree with the
goal of this project, which is a stable, robust, open DIY platform for race
telemetry. To contribute, look at the open issues, or open an issue.

The firmware errors and the `todo!()` bodies in [Current State](#current-state)
are a good place to start. Each stub has a doc comment that states the intended
behavior.

### Troubleshooting

#### `probe-rs` cannot attach to the target

The target is usually in WFI and ignores SWD. Put the chip into the ROM
bootloader with the steps in [Firmware](#firmware), and then flash it again.

#### `cargo build` builds only `ort_types`

This is correct. The firmware builds for `thumbv7em-none-eabi` and the desktop
app builds for the host, so they cannot build together. The workspace sets
`default-members` to `ort_types`. Give a target with `-p` to build the other
crates.

#### The TypeScript bindings disagree with the Rust commands

Run `cargo test -p desktop`. The test regenerates `desktop/src/bindings.ts`.
Commit the file after the test writes it.
