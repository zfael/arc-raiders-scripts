use winapi::um::shellapi::IsUserAnAdmin;
use winapi::um::timeapi::{timeBeginPeriod, timeEndPeriod};

/// Initialize high-resolution timing for Windows
/// This enables 1ms precision for std::thread::sleep
pub fn init_timing() {
    unsafe {
        // Request 1ms timer resolution
        timeBeginPeriod(1);
    }
}

/// Cleanup timing on shutdown (should be called when app exits)
pub fn cleanup_timing() {
    unsafe {
        timeEndPeriod(1);
    }
}

/// Check if running with administrator privileges
/// Required for input injection into elevated processes like games
pub fn check_permissions() -> bool {
    unsafe {
        IsUserAnAdmin() != 0
    }
}
