use winapi::um::timeapi::timeBeginPeriod;

/// Initialize high-resolution timing for Windows
/// This enables 1ms precision for std::thread::sleep
pub fn init_timing() {
    unsafe {
        // Request 1ms timer resolution
        timeBeginPeriod(1);
    }
}

/// Check if running with administrator privileges
/// Required for input injection into elevated processes like games
pub fn check_permissions() -> bool {
    // On Windows, we'll always return true since the app.manifest requests elevation
    // The OS will automatically prompt for admin rights on startup
    true
}
