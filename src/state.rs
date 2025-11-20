use parking_lot::RwLock;
use std::sync::Arc;
use std::time::Instant;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    inner: Arc<RwLock<AppStateInner>>,
}

#[derive(Debug)]
struct AppStateInner {
    /// Whether semi-auto max fire rate is enabled
    pub semiauto_enabled: bool,
    
    /// Whether reload cancel is enabled
    pub reload_cancel_enabled: bool,
    
    /// Timing for reload cancel sequence (ms between keypresses)
    pub reload_cancel_timing: u64,
    
    /// Timing for semi-auto click rate (ms between clicks)
    pub semiauto_timing: u64,
    
    /// Weapon slot for reload cancel (1 or 2)
    pub reload_cancel_weapon_slot: u8,
    
    /// Whether mouse is currently held down
    pub mouse_held: bool,
    
    /// Last time a click was sent (for anti-loop protection)
    pub last_click_time: Option<Instant>,
}

impl Default for AppStateInner {
    fn default() -> Self {
        Self {
            semiauto_enabled: false,
            reload_cancel_enabled: false,
            reload_cancel_timing: crate::platform::get_default_timing(),
            semiauto_timing: 60, // 60ms = ~16.6 clicks/second
            reload_cancel_weapon_slot: 1, // Default to weapon slot 1
            mouse_held: false,
            last_click_time: None,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AppStateInner::default())),
        }
    }
    
    // Getters
    pub fn is_semiauto_enabled(&self) -> bool {
        self.inner.read().semiauto_enabled
    }
    
    pub fn is_reload_cancel_enabled(&self) -> bool {
        self.inner.read().reload_cancel_enabled
    }
    
    pub fn get_reload_cancel_timing(&self) -> u64 {
        self.inner.read().reload_cancel_timing
    }
    
    pub fn get_semiauto_timing(&self) -> u64 {
        self.inner.read().semiauto_timing
    }
    
    pub fn get_reload_cancel_weapon_slot(&self) -> u8 {
        self.inner.read().reload_cancel_weapon_slot
    }
    
    pub fn get_last_click_time(&self) -> Option<Instant> {
        self.inner.read().last_click_time
    }
    
    // Setters
    pub fn set_semiauto_enabled(&self, enabled: bool) {
        self.inner.write().semiauto_enabled = enabled;
    }
    
    pub fn set_reload_cancel_enabled(&self, enabled: bool) {
        self.inner.write().reload_cancel_enabled = enabled;
    }
    
    pub fn set_reload_cancel_timing(&self, timing: u64) {
        self.inner.write().reload_cancel_timing = timing;
    }
    
    pub fn set_semiauto_timing(&self, timing: u64) {
        self.inner.write().semiauto_timing = timing;
    }
    
    pub fn set_reload_cancel_weapon_slot(&self, slot: u8) {
        self.inner.write().reload_cancel_weapon_slot = slot.clamp(1, 2);
    }
    
    pub fn set_mouse_held(&self, held: bool) {
        self.inner.write().mouse_held = held;
    }
    
    pub fn set_last_click_time(&self, time: Instant) {
        self.inner.write().last_click_time = Some(time);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
