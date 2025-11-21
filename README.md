# Arc Raiders Scripts

A gaming automation tool for Arc Raiders with semi-auto fire rate enhancement and reload cancel features.

## ⚠️ WARNING

**This tool uses macros to automate game inputs. Using this tool may:**
- Violate the game's Terms of Service
- Result in permanent account bans
- Be detected by anti-cheat systems

**Use at your own risk. The authors are not responsible for any consequences.**

## Download

Download pre-built binaries from the [Releases](https://github.com/zfael/arc-raiders-scripts/releases) page:

- **Windows**: `arc-raiders-scripts-v{version}-windows-x64.exe`

## Features

- **Semiauto Max Fire Rate**: Hold left mouse button to continuously fire semi-automatic weapons at maximum rate
- **Reload Cancel**: Automatically cancels reload animations using the Q→1 key sequence

## Requirements

- Windows 10 or later
- Administrator privileges (automatically requested on launch)
- [Interception driver](https://github.com/oblitum/Interception) for input simulation

## Building

```bash
# Build for release
cargo build --release

# Run in development
cargo run
```

The compiled binary will be in `target/release/arc-raiders-scripts.exe`.

## Releasing

This project uses automated releases via GitHub Actions with conventional commits:

- `feat:` commits trigger a minor version bump (v1.0.0 → v1.1.0)
- `fix:` commits trigger a patch version bump (v1.0.0 → v1.0.1)
- `feat!:` or `BREAKING CHANGE:` trigger a major version bump (v1.0.0 → v2.0.0)

GitHub Actions will automatically:
- Build for Windows x64
- Create a new release
- Upload the binary as a downloadable asset

## Usage

1. Launch the application (on Windows, it will request admin privileges)
2. Check the features you want to enable:
   - **Semiauto Max Fire Rate**: Hold mouse button to rapid-fire
   - **Reload Cancel**: Activates on every mouse click
3. Adjust timing settings as needed:
   - **Semi-Auto Click Rate**: Controls clicks per second (lower = faster)
   - **Reload Cancel Timing**: Delay between key presses in sequence
4. The application runs in the background - macros work system-wide

## How It Works

### Semiauto Max Fire Rate
- Detects when left mouse button is held down for >100ms
- Sends repeated left clicks at configured rate (default: 60ms = ~16 clicks/sec)
- Includes anti-loop protection with event filtering to prevent stuck firing
- Adds random jitter (±5ms) to avoid pattern detection
- Uses DOWN/UP event counter for reliable mouse release detection

### Reload Cancel
- Triggers on every left mouse click
- Executes key sequence: Q (quick-use) → 1 (weapon switch back)
- Timing is configurable (default: 75ms)
- Includes random jitter (±5ms) for realistic timing
- Key hold time: 50ms for reliable registration

## Technical Details

- **GUI Framework**: egui/eframe for native UI
- **Input Listening**: rdev for capturing mouse/keyboard events
- **Input Simulation**: Interception driver for kernel-level input injection
- **Concurrency**: Multi-threaded architecture with lock-free channels
- **Timing**: High-resolution timing (1ms precision)

## Architecture

```
GUI Thread (egui)
    ↓ Shared State (Arc<RwLock>)
Input Listener (rdev)
    ↓ Event Channel
Automation Engine (Interception)
```

## Notes

- Requires UAC elevation (embedded manifest handles this)
- Uses Interception driver for kernel-level input injection
- High-resolution timer enabled (1ms precision)
- Event filtering prevents synthetic clicks from causing stuck firing
- Supports F1 toggle for rapid fire, F2 toggle for reload cancel, Q to disable reload cancel

## Troubleshooting

### "Input not working in game"
- Ensure the application is running as Administrator
- Verify Interception driver is installed correctly
- Check if the game has kernel-level anti-cheat (may block driver)

### "Interception driver not found"
- Download and install the [Interception driver](https://github.com/oblitum/Interception)
- Copy `interception.dll` to the same folder as the executable
- Restart your computer after driver installation

### "Macro timing feels wrong"
- Adjust the timing sliders in the GUI
- Add more delay if keys/clicks aren't registering
- Reduce delay if the macro feels sluggish

## License

This project is for educational purposes only. Use responsibly and at your own risk.
