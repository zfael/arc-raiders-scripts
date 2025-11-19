#[cfg(windows)]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

/// Initialize platform-specific timing for high-resolution sleep
pub fn init_timing() {
    #[cfg(windows)]
    windows::init_timing();
    
    #[cfg(target_os = "macos")]
    macos::init_timing();
}

/// Check if the application has necessary permissions
pub fn check_permissions() -> bool {
    #[cfg(windows)]
    return windows::check_permissions();
    
    #[cfg(target_os = "macos")]
    return macos::check_permissions();
}

/// Get platform-specific default timing for reload cancel sequence (in milliseconds)
pub fn get_default_timing() -> u64 {
    #[cfg(windows)]
    return 75;
    
    #[cfg(target_os = "macos")]
    return 100;
    
    #[cfg(not(any(windows, target_os = "macos")))]
    return 75;
}
