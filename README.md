<img width="1280" height="320" alt="OrtGithubImage" src="https://github.com/user-attachments/assets/187a4ebe-e4b7-43f4-907a-d34e5ed48dbb" />

# Open Race Telemetry

Open Race Telemetry is an open-source telemetry system for motorcycle racers, letting you build your own telemetry system at a low cost, compared to commercial systems.

_Quick mention_

> I decided to re-write this README because the original one I wrote with AI assistance, and that felt...wrong. This project is part learning, part seeing a real project through to the end, building something I want to use every single day. I want to be able to describe, in detail, every part of this project from memory.
> So, unless its like an emergency or something, no AI usage for helping with docs or just banging out features. I want to suffer through the learning process. Thanks!

### Status

NOTE: This project is under active development. It is not ready
for commercial/official use yet.

### Overview

Telemetry is incredibly important in motorsports and mechanical systems of all kinds, as it gives you detailed information about your hardware with detailed precision.

In motorcycles, this is just as important. Not only for the various safety features it provides (adaptive rear braking, IMU lean angle sensitivity for traction control, wheelie control, etc.), but it allows for competitive riders to dial in their craft and extract the best lap times they can.

The goal of this project is to enable everyone on two wheels (and even four!) to access detailed telemetry at a reasonable price, all while knowing your data isn't being sold.

### Project Structure

This repo contains two Cargo workspaces and three crates. the `firmware` crate is its own workspace. This is due to rust-analyzer only being able to analyze one workspace target at a time.

| Path         | Crate       | Purpose                                                                                                                           |
| ------------ | ----------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `ort_types/` | `ort_types` | Data types shared by the firmware and the desktop app. It is `no_std`. It has no hardware access and no sensor math.              |
| `firmware/`  | `firmware`  | Embassy firmware for the STM32F411CE. It builds for the `thumbv7em-none-eabi` target. It produces the `ort_firmware` binary.      |
| `desktop/`   | `desktop`   | Tauri 2 desktop application. The Rust backend is in `desktop/src-tauri/`. The React and TypeScript frontend is in `desktop/src/`. |

While the firmware and desktop app build for different targets, you are able to use `just` to build both sequentially.

### Packages

- `ort_types`
- `desktop`
- `firmware`

**What works right now**
as of 2026-10-23, here are the parts that work:

- `ort_types` builds and it's tests pass.
- Byte parsing for Nmea messages and Imu data are complete (unless I find something new or make a change to the hardware).
- The desktop builds, however the UI is not complete.
- `imu/math.rs` is considered complete. It holds the math that converts raw register values to usable data. Since it has no hardware access, it is easy to unit test.

**Incomplete**

- The firmware technically builds, but many features are yet to be implemented.
- Many of the function bodies have `todo!()` placeholders. They will panic if you call them.
- The desktop app is incomplete. This will be the last thing I work on. I have scaffolded a demo UI in another branch at the time of writing, however, it is not the final decision and was just testing. Once the hardware and data layers are stable, I will focus on the desktop app.

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

| Signal       | Pin                  | Notes                                                                                             |
| ------------ | -------------------- | ------------------------------------------------------------------------------------------------- |
| Status LED   | PC13                 | On the board. It is active-low. A low level turns the LED on.                                     |
| IMU (I2C1)   | PB6 = SCL, PB7 = SDA | MPU6050 at address 0x68 with AD0 low, or 0x69 with AD0 high.                                      |
| GPS (USART1) | PA9 = TX, PA10 = RX  | TX goes from the MCU to the GPS. RX comes from the GPS to the MCU. The default rate is 9600 baud. |

The I2C lines need external 4.7k pull-up resistors to 3V3. `embassy-stm32`
configures the I2C pins as open-drain. It does not supply pull-ups. The
earlier ESP32 build used the weak internal pull-ups of that MCU (a
difference from the current hardware).

#### Firmware

The firmware crate builds, but many functions are not complete yet. Flashed
firmware built from this code will panic at those `todo!()` calls. See
[Current State](#current-state).

```bash
just build-firmware  # build the firmware for thumbv7em-none-eabi
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

### Troubleshooting

#### `probe-rs` cannot attach to the target

The target is usually in WFI. It ignores SWD in this state. Put the chip
into the ROM bootloader with the steps in [Firmware](#firmware). Then flash
it again.

#### `cargo build` builds only `ort_types`

This is correct. The firmware has its own workspace and builds for
`thumbv7em-none-eabi`. The desktop app builds for the host. These targets
cannot build in the same command. The root workspace sets `default-members`
to `ort_types`.

Use the manifest path to build the firmware from the repository root:

```bash
cargo build --manifest-path firmware/Cargo.toml --target thumbv7em-none-eabi
```

From the `firmware/` directory, `cargo build` reads
`firmware/.cargo/config.toml` and uses the correct target. The command
`cargo build -p firmware` from the repository root does not work, because
`firmware` is not a member of the root workspace.

#### The TypeScript bindings disagree with the Rust commands

Run `cargo test -p desktop`. This test regenerates the file
`desktop/src/bindings.ts`. Commit the file after the test writes it.

### Contributing

Contributions are always welcome! Right now, I will happily read any feature ideas, bug fixes, or design proposals. A feature or idea may be refused if it does not align with the immediate goal of the project (that being a stable, open-source DIY platform for race telemetry). If any PRs are opened and its just straight up AI, I will most definitely refuse it.
To contribute, please look at the open issues, or open a new issue.

### Thank you

Thanks for checking out my project! I am incredibly passionate for motorsports, software, and I love the challenge of building something that is out of my comfort zone. This is all a learning process for me on the hardware side, so for now, development is slower as I take the time to truly learn each thing I am doing, and not copy-pasting code from embassy's examples.

\- Brendan
