use core_graphics::event::CGEvent;
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

/// Initialize timing for macOS (no-op, macOS has good default timing)
pub fn init_timing() {
    // macOS has sufficient timing precision by default
}

/// Check if accessibility permissions are granted
/// Required for rdev to capture input events and enigo to send them
pub fn check_permissions() -> bool {
    // Try to create a CGEventSource - this will fail if we don't have accessibility permissions
    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState);
    if source.is_err() {
        return false;
    }
    
    // Try to create a test event - if this works, we have permissions
    let event = CGEvent::new(source.unwrap());
    event.is_ok()
}
