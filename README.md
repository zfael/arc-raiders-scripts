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

- **Windows (Primary)**: `arc-raiders-scripts-windows-x64.exe`
- **macOS Intel**: `arc-raiders-scripts-macos-x64`
- **macOS Apple Silicon**: `arc-raiders-scripts-macos-arm64`

## Features

- **Semiauto Max Fire Rate**: Hold left mouse button to continuously fire semi-automatic weapons at maximum rate
- **Reload Cancel**: Automatically cancels reload animations using the R→Q→1 exploit

## Requirements

### Windows (Primary Platform)
- Windows 10 or later
- Administrator privileges (automatically requested on launch)

### macOS (Testing Only)
- macOS 10.15 or later
- Accessibility permissions (prompted on first run)
- Input Monitoring permissions (prompted on first run)

## Building

```bash
# Build for release
cargo build --release

# Build for specific target
cargo build --release --target x86_64-pc-windows-msvc    # Windows
cargo build --release --target x86_64-apple-darwin       # macOS Intel
cargo build --release --target aarch64-apple-darwin      # macOS ARM

# Run in development
cargo run
```

The compiled binary will be in `target/release/arc-raiders-scripts` (or `.exe` on Windows).

## Releasing

To create a new release with pre-built binaries:

1. Create and push a new tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. GitHub Actions will automatically:
   - Build for Windows, macOS Intel, and macOS ARM
   - Create a new release
   - Upload all binaries as downloadable assets

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
- Includes anti-loop protection to prevent infinite clicking
- Adds random jitter (±10ms) to avoid pattern detection

### Reload Cancel
- Triggers on every left mouse click
- Executes key sequence: R (reload) → Q (quick-use) → 1 (weapon)
- Timing is configurable (default: 75ms on Windows, 100ms on macOS)
- Includes random jitter for realistic timing

## Technical Details

- **GUI Framework**: egui/eframe for cross-platform native UI
- **Input Listening**: rdev for capturing mouse/keyboard events
- **Input Simulation**: enigo for sending synthetic inputs
- **Concurrency**: Multi-threaded architecture with lock-free channels
- **Timing**: Platform-specific high-resolution timing (1ms precision on Windows)

## Architecture

```
GUI Thread (egui)
    ↓ Shared State (Arc<RwLock>)
Input Listener (rdev)
    ↓ Event Channel
Automation Engine (enigo)
```

## Platform-Specific Notes

### Windows
- Requires UAC elevation (embedded manifest handles this)
- Uses `SendInput()` API for input simulation
- High-resolution timer enabled (1ms precision)
- May be blocked by kernel-level anti-cheat systems

### macOS
- Requires Accessibility permissions grant
- Requires Input Monitoring permissions grant
- System will prompt on first use
- Higher input latency than Windows (~10-20ms)

## Troubleshooting

### Windows: "Input not working in game"
- Ensure the application is running as Administrator
- Check if the game is also running as Administrator
- Some games with anti-cheat may block all input automation

### macOS: "Failed to listen for events"
- Go to System Preferences → Security & Privacy → Privacy
- Add the application to Accessibility and Input Monitoring

### "Macro timing feels wrong"
- Adjust the timing sliders in the GUI
- Add more delay if keys/clicks aren't registering
- Reduce delay if the macro feels sluggish

## License

This project is for educational purposes only. Use responsibly and at your own risk.
