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
        let mut mouse_hold_start: Option<Instant> = None;
        const HOLD_THRESHOLD_MS: u64 = 100; // Consider "held" after 100ms

        loop {
            // Check for input events (non-blocking)
            if let Ok(event) = rx.try_recv() {
                match event {
                    InputEvent::MouseButtonDown => {
                        mouse_hold_start = Some(Instant::now());
                        
                        // Handle reload cancel on click down
                        if state.is_reload_cancel_enabled() {
                            if let Err(e) = execute_reload_cancel(&mut enigo, &state) {
                                eprintln!("Error executing reload cancel: {:?}", e);
                            }
                        }
                    }
                    InputEvent::MouseButtonUp => {
                        mouse_hold_start = None;
                    }
                }
            }

            // Handle semi-auto continuous clicking if mouse is held
            if state.is_semiauto_enabled() && state.is_mouse_held() {
                if let Some(hold_start) = mouse_hold_start {
                    // Only start rapid clicking after threshold to avoid interfering with single clicks
                    if hold_start.elapsed().as_millis() >= HOLD_THRESHOLD_MS as u128 {
                        if crate::input::can_send_click(&state, state.get_semiauto_timing()) {
                            if let Err(e) = send_click(&mut enigo, &state) {
                                eprintln!("Error sending click: {:?}", e);
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

/// Execute the reload cancel sequence: R -> Q -> 1
fn execute_reload_cancel(enigo: &mut Enigo, state: &AppState) -> EnigoResult<()> {
    let base_timing = state.get_reload_cancel_timing();
    
    // Small initial delay to let the original click register
    thread::sleep(Duration::from_millis(add_jitter(5)));
    
    // Press R (reload)
    press_key(enigo, Key::Unicode('r'))?;
    thread::sleep(Duration::from_millis(add_jitter(base_timing)));
    
    // Press Q (quick-use equipment)
    press_key(enigo, Key::Unicode('q'))?;
    thread::sleep(Duration::from_millis(add_jitter(base_timing)));
    
    // Press 1 (switch back to gun)
    press_key(enigo, Key::Unicode('1'))?;
    
    Ok(())
}

/// Press and release a key with realistic hold time
fn press_key(enigo: &mut Enigo, key: Key) -> EnigoResult<()> {
    enigo.key(key, Direction::Press)?;
    thread::sleep(Duration::from_millis(add_jitter(20))); // Hold for ~20ms
    enigo.key(key, Direction::Release)?;
    Ok(())
}

/// Add random jitter to timing to avoid detection
fn add_jitter(base_ms: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let jitter: i64 = rng.gen_range(-10..=10); // ±10ms jitter
    let result = (base_ms as i64 + jitter).max(5) as u64; // Minimum 5ms
    result
}
