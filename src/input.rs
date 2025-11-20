use crate::state::AppState;
use crossbeam_channel::Sender;
use rdev::{listen, Button, Event, EventType};
use std::panic;

#[derive(Debug, Clone)]
pub enum InputEvent {
    MouseButtonDown,
    MouseButtonUp,
    ToggleReloadCancel, // F2 key to toggle reload cancel
}

/// Start listening for input events in a background thread
pub fn start_listener(state: AppState, tx: Sender<InputEvent>) -> anyhow::Result<()> {
    std::thread::spawn(move || {
        // Set up panic handler for this thread
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let callback = move |event: Event| {
                // Keep callback fast - just detect events and send to channel
                match event.event_type {
                    EventType::ButtonPress(Button::Left) => {
                        state.set_mouse_held(true);
                        let _ = tx.send(InputEvent::MouseButtonDown);
                    }
                    EventType::ButtonRelease(Button::Left) => {
                        state.set_mouse_held(false);
                        let _ = tx.send(InputEvent::MouseButtonUp);
                    }
                    EventType::KeyPress(rdev::Key::F2) => {
                        let _ = tx.send(InputEvent::ToggleReloadCancel);
                    }
                    _ => {}
                }
            };

            // Start the event listener (blocks until error or shutdown)
            if let Err(e) = listen(callback) {
                eprintln!("Error in input listener: {:?}", e);
            }
        }));

        if let Err(e) = result {
            eprintln!("Input listener panicked: {:?}", e);
            #[cfg(target_os = "macos")]
            eprintln!("This is likely due to missing Accessibility or Input Monitoring permissions.");
            eprintln!("The application will continue to run, but input detection won't work.");
        }
    });

    Ok(())
}

/// Check if enough time has passed since last click to avoid loops
pub fn can_send_click(state: &AppState, min_delay_ms: u64) -> bool {
    if let Some(last_time) = state.get_last_click_time() {
        let elapsed = last_time.elapsed();
        elapsed.as_millis() >= min_delay_ms as u128
    } else {
        true
    }
}
