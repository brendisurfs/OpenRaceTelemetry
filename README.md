# Open Race Telemetry

An open-source telemetry system for motorcycle racers,
enabling people to build their own telemetry systems
for a fraction of the price of commercil systems.

### Status

🚧 In Progress — this project is under active development and not yet ready for use.

### Overview

Telemetry in racing is incredibly important for a variety of reasons within racing.
It allows you to map out your racing lines, speed in corners, and where you can push further.
These all result in improving lap times.

I wanted to build something that I would ultimately like to use.
I didn't want to buy a system that didn't allow me to customize, extend, and adapt for my needs.
I enjoy building out things, I enjoy exploring the inner workings of things.

This project holds the software components for the Open Race Telemetry project.
The software includes components for the ESP32, IMU, and other components added to the module.

### Project Structure

[TODO]

### How To Use This Project

[TODO]

### Running

#### Requirements

- [PlatformIO Core](https://docs.platformio.org/en/latest/core/installation/index.html) (the `pio` CLI)
- [`just`](https://github.com/casey/just) — command runner used to drive builds/uploads/tests
  - macOS: `brew install just`
  - Linux: `cargo install just`, or see the [packaging status table](https://github.com/casey/just#packages) for your distro's package manager
  - Windows: `winget install --id Casey.Just` or `scoop install just`
- [clang / clangd](https://clangd.llvm.org/installation) — for editor autocomplete and diagnostics against `compile_commands.json`
  - macOS: `brew install llvm`
  - Linux: install the `clangd` package for your distro (e.g. `apt install clangd` on Debian/Ubuntu, `pacman -S clang` on Arch), or use the [LLVM apt repository](https://apt.llvm.org/) for the latest version
  - Windows: install the [LLVM release](https://github.com/llvm/llvm-project/releases) (includes `clangd`), or `winget install LLVM.LLVM`
- An Arduino Nano ESP32 (or update `platformio.ini` to match your board)
- A USB cable to connect the board to your computer

#### Uploading

1. Connect the board to your computer via USB.
2. Put the board into bootloader mode. On the Arduino Nano ESP32, double-press the reset button — the onboard LED will pulse, indicating it's ready to receive firmware.
3. Update `upload_port` in `platformio.ini` to match the port your board enumerates as (find it with `pio device list`).
4. Build, upload, and monitor serial output in one step:

   ```bash
   just upload-and-monitor
   ```

   See the `justfile` for individual `build`, `upload`, `test-native`, and `test-esp32` recipes.

### Contributing

[TODO]

### Troubleshooting

#### `pio run -t compiledb` fails with "No module named pip.**main**"

If `pio run -t compiledb` (or any PlatformIO command that installs packages) fails with:

```
No module named pip.__main__; 'pip' is a package and cannot be directly executed
```

PlatformIO's bundled Python environment is missing a working `pip` install (the `pip` "package" it finds is a broken/namespace stub with no real files). This can happen after a Python version upgrade via Homebrew.

Fix it by reinstalling pip into PlatformIO's Python environment with `ensurepip`:

```bash
/opt/homebrew/Cellar/platformio/<version>/libexec/bin/python -m ensurepip --upgrade
```

Replace `<version>` with your installed PlatformIO version (find it with `ls /opt/homebrew/Cellar/platformio/`). Verify it worked with:

```bash
/opt/homebrew/Cellar/platformio/<version>/libexec/bin/python -m pip --version
```

Then re-run `pio run -t compiledb`.
