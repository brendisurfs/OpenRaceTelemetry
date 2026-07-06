# Open Race Telemetry

An open-source telemetry system for motorcycle racers,
enabling people to build their own telemetry systems
for a fraction of the price of commercil systems.

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

You will need PlatformIO's cli tool to run this.
[TODO]

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
