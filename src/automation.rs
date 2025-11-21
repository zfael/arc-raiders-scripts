use crate::input::InputEvent;
use crate::state::AppState;
use crossbeam_channel::Receiver;
use enigo::{Direction, Enigo, Key, Keyboard, Mouse, Settings};
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};

type EnigoResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Start the automation engine in a background thread
pub fn start_automation(state: AppState, rx: Receiver<InputEvent>) -> anyhow::Result<()> {
    std::thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo");
        let mut last_real_click: Option<Instant> = None;
        let mut clicking_active = false;
        let mut last_synthetic_click: Option<Instant> = None;
        let mut down_event_count = 0; // Track DOWN events to detect real releases
        const HOLD_THRESHOLD_MS: u64 = 100; // Consider "held" after 100ms
        const SYNTHETIC_EVENT_WINDOW_MS: u64 = 30; // Events within 30ms of synthetic click are ignored
        
        println!("✓ Automation engine started");

        loop {
            // Check for input events (non-blocking)
            while let Ok(event) = rx.try_recv() {
                // Check if this event is likely from our synthetic click
                let is_synthetic = if let Some(last_synth) = last_synthetic_click {
                    last_synth.elapsed().as_millis() < SYNTHETIC_EVENT_WINDOW_MS as u128
                } else {
                    false
                };

                match event {
                    InputEvent::MouseButtonDown => {
                        down_event_count += 1;
                        // Ignore DOWN events that are likely from our synthetic clicks
                        if !is_synthetic && !clicking_active {
                            last_real_click = Some(Instant::now());
                            clicking_active = true;
                            println!("Mouse button DOWN detected (real click)");
                            
                            // Handle reload cancel on click down
                            if state.is_reload_cancel_enabled() {
                                if let Err(e) = execute_reload_cancel(&mut enigo, &state) {
                                    eprintln!("Error executing reload cancel: {:?}", e);
                                }
                            }
                        }
                    }
                    InputEvent::MouseButtonUp => {
                        // Count UP events
                        if down_event_count > 0 {
                            down_event_count -= 1;
                        }
                        
                        // Process UP event if it seems real (not within synthetic window)
                        // OR if we've received multiple UPs (definitely a real release)
                        if !is_synthetic || down_event_count == 0 {
                            last_real_click = None;
                            clicking_active = false;
                            down_event_count = 0; // Reset counter
                            println!("Mouse button UP detected (stopping rapid fire)");
                        }
                    }
                    InputEvent::ToggleReloadCancel => {
                        // Toggle reload cancel on/off
                        let current = state.is_reload_cancel_enabled();
                        state.set_reload_cancel_enabled(!current);
                        if !current {
                            println!("🔄 RELOAD CANCEL ENABLED");
                        } else {
                            println!("⭕ RELOAD CANCEL DISABLED");
                        }
                    }
                    InputEvent::WeaponSlot1 => {
                        state.set_current_weapon_slot(1);
                        if state.get_auto_toggle_by_weapon() {
                            let enabled = state.is_reload_cancel_enabled();
                            println!("🔫 Switched to Weapon Slot 1 - Reload Cancel: {}", if enabled { "ON" } else { "OFF" });
                        }
                    }
                    InputEvent::WeaponSlot2 => {
                        state.set_current_weapon_slot(2);
                        if state.get_auto_toggle_by_weapon() {
                            let enabled = state.is_reload_cancel_enabled();
                            println!("🔫 Switched to Weapon Slot 2 - Reload Cancel: {}", if enabled { "ON" } else { "OFF" });
                        }
                    }
                    InputEvent::QKeyPressed => {
                        // Disable reload cancel when Q is pressed (using item)
                        if state.is_reload_cancel_enabled() {
                            state.set_reload_cancel_enabled(false);
                            println!("⭕ RELOAD CANCEL DISABLED (Q key pressed - using item)");
                        }
                    }
                }
            }

            // Handle semi-auto continuous clicking if mouse is held
            if state.is_semiauto_enabled() && clicking_active {
                if let Some(real_click_time) = last_real_click {
                    // Only start rapid clicking after threshold to avoid interfering with single clicks
                    let elapsed_ms = real_click_time.elapsed().as_millis();
                    if elapsed_ms >= HOLD_THRESHOLD_MS as u128 {
                        let delay_ms = state.get_semiauto_timing();
                        let can_click = crate::input::can_send_click(&state, delay_ms);
                        
                        if can_click {
                            if let Err(e) = send_click(&mut enigo, &state) {
                                eprintln!("Error sending click: {:?}", e);
                            } else {
                                last_synthetic_click = Some(Instant::now());
                                println!("Sent automatic click (elapsed: {}ms, delay: {}ms)", elapsed_ms, delay_ms);
                            }
                        }
                    }
                }
            }

            // Small sleep to avoid busy-waiting
            thread::sleep(Duration::from_millis(1));
        }
    });

    Ok(())
}

/// Send a single mouse click
fn send_click(enigo: &mut Enigo, state: &AppState) -> EnigoResult<()> {
    enigo.button(enigo::Button::Left, Direction::Click)?;
    state.set_last_click_time(Instant::now());
    Ok(())
}

/// Execute the reload cancel sequence: Q -> [1 or 2]
fn execute_reload_cancel(enigo: &mut Enigo, state: &AppState) -> EnigoResult<()> {
    let base_timing = state.get_reload_cancel_timing();
    let weapon_slot = state.get_reload_cancel_weapon_slot();
    
    // Wait for base timing to ensure the click is fully processed
    thread::sleep(Duration::from_millis(add_jitter(base_timing)));
    
    // Press Q (quick-use equipment)
    press_key(enigo, Key::Unicode('q'), 50)?;
    
    // Wait between keys to ensure each registers
    thread::sleep(Duration::from_millis(add_jitter(base_timing)));
    
    // Press weapon slot key (1 or 2)
    let weapon_key = if weapon_slot == 1 { '1' } else { '2' };
    press_key(enigo, Key::Unicode(weapon_key), 50)?;
    
    Ok(())
}

/// Press and release a key with specified hold time
fn press_key(enigo: &mut Enigo, key: Key, hold_ms: u64) -> EnigoResult<()> {
    enigo.key(key, Direction::Press)?;
    thread::sleep(Duration::from_millis(add_jitter(hold_ms)));
    enigo.key(key, Direction::Release)?;
    Ok(())
}

/// Add random jitter to timing to avoid detection
fn add_jitter(base_ms: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let jitter: i64 = rng.gen_range(-5..=5); // Reduced jitter to ±5ms for more consistency
     // Minimum 5ms
    (base_ms as i64 + jitter).max(5) as u64
}
