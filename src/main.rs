mod automation;
mod input;
mod platform;
mod state;

use crossbeam_channel::unbounded;
use eframe::egui;
use state::AppState;

fn main() -> anyhow::Result<()> {
    // Initialize platform-specific features
    platform::init_timing();
    
    // Check permissions - CRITICAL on macOS
    let has_permissions = platform::check_permissions();
    
    #[cfg(target_os = "macos")]
    if !has_permissions {
        eprintln!("\n❌ ERROR: Missing required permissions on macOS!");
        eprintln!("\nThis application requires Accessibility and Input Monitoring permissions.");
        eprintln!("\nTo grant permissions:");
        eprintln!("1. Open System Preferences → Security & Privacy → Privacy");
        eprintln!("2. Select 'Accessibility' and add this application");
        eprintln!("3. Select 'Input Monitoring' and add this application");
        eprintln!("4. Restart this application");
        eprintln!("\nApplication path: {:?}", std::env::current_exe()?);
        eprintln!("\nThe application will NOT work without these permissions.\n");
        
        // Still open GUI to show the message
    }
    
    #[cfg(windows)]
    if !has_permissions {
        eprintln!("Warning: Application may not be running as Administrator.");
        eprintln!("Some games may require administrator privileges for input automation.");
    }

    // Test enigo creation
    match enigo::Enigo::new(&enigo::Settings::default()) {
        Ok(_) => {
            if has_permissions {
                println!("✓ Input automation initialized successfully");
            }
        },
        Err(e) => {
            eprintln!("Failed to initialize input automation: {:?}", e);
        }
    }

    // Create shared state
    let state = AppState::new();
    
    // Create channel for input events
    let (tx, rx) = unbounded::<input::InputEvent>();
    
    // Only start listeners if we have permissions
    // Note: On macOS, rdev requires Input Monitoring permission which cannot be
    // reliably checked programmatically. If the app crashes, you need to manually
    // grant Input Monitoring permission in System Preferences.
    #[cfg(not(target_os = "macos"))]
    {
        if has_permissions {
            // Start input listener
            let listener_state = state.clone();
            input::start_listener(listener_state, tx)?;
            
            // Start automation engine
            let automation_state = state.clone();
            automation::start_automation(automation_state, rx)?;
            
            println!("✓ Input monitoring active");
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        eprintln!("\n⚠️  macOS IMPORTANT:");
        eprintln!("This app needs BOTH Accessibility AND Input Monitoring permissions.");
        eprintln!("\nGrant permissions at:");
        eprintln!("  System Preferences → Security & Privacy → Privacy → Accessibility");
        eprintln!("  System Preferences → Security & Privacy → Privacy → Input Monitoring");
        eprintln!("\nAfter granting permissions, rebuild and run:");
        eprintln!("  cargo clean && cargo build --release");
        eprintln!("  ./target/release/arc-raiders-scripts");
        eprintln!("\nFor now, the GUI will open but macros won't work on macOS.");
        eprintln!("Test on Windows where this will work properly.\n");
    }
    
    // Start GUI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 500.0])
            .with_min_inner_size([450.0, 500.0])
            .with_resizable(true),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "Arc Raiders Scripts",
        options,
        Box::new(|_cc| Box::new(MacroApp::new(state))),
    );
    
    Ok(())
}

struct MacroApp {
    state: AppState,
}

impl MacroApp {
    fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl eframe::App for MacroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Arc Raiders Scripts");
            ui.add_space(10.0);
            
            ui.separator();
            ui.add_space(10.0);
            
            // Warning banner
            ui.colored_label(
                egui::Color32::from_rgb(255, 150, 0),
                "⚠️ WARNING: Using macros may violate game Terms of Service"
            );
            ui.colored_label(
                egui::Color32::from_rgb(255, 150, 0),
                "and result in account bans. Use at your own risk!"
            );
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            // Feature checkboxes
            let mut semiauto = self.state.is_semiauto_enabled();
            if ui.checkbox(&mut semiauto, "Semiauto Max Fire Rate (not working)").changed() {
                self.state.set_semiauto_enabled(semiauto);
            }
            ui.label("  • Hold left mouse button to rapid-fire semi-auto weapons");
            ui.add_space(5.0);
            
            let mut reload_cancel = self.state.is_reload_cancel_enabled();
            if ui.checkbox(&mut reload_cancel, "Reload Cancel").changed() {
                self.state.set_reload_cancel_enabled(reload_cancel);
            }
            ui.label("  • Automatically cancels reload animation on click");
            
            // Weapon slot selector (indented under reload cancel)
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label("Weapon Slot:");
                let mut weapon_slot = self.state.get_reload_cancel_weapon_slot();
                if ui.radio_value(&mut weapon_slot, 1, "Slot 1").changed() {
                    self.state.set_reload_cancel_weapon_slot(1);
                }
                if ui.radio_value(&mut weapon_slot, 2, "Slot 2").changed() {
                    self.state.set_reload_cancel_weapon_slot(2);
                }
            });
            
            // Auto-toggle by weapon checkbox
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                let mut auto_toggle = self.state.get_auto_toggle_by_weapon();
                if ui.checkbox(&mut auto_toggle, "Auto-toggle by weapon slot").changed() {
                    self.state.set_auto_toggle_by_weapon(auto_toggle);
                }
            });
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new("  • Only enable reload cancel on selected slot").small().color(egui::Color32::GRAY));
            });
            
            ui.add_space(15.0);
            
            ui.separator();
            ui.add_space(10.0);
            
            // Settings
            ui.heading("Settings");
            ui.add_space(10.0);
            
            // Semi-auto timing
            ui.label("Semi-Auto Click Rate (ms between clicks):");
            let mut semiauto_timing = self.state.get_semiauto_timing() as f32;
            if ui.add(egui::Slider::new(&mut semiauto_timing, 30.0..=150.0)).changed() {
                self.state.set_semiauto_timing(semiauto_timing as u64);
            }
            ui.label(format!("  ~{:.1} clicks/second", 1000.0 / semiauto_timing));
            ui.add_space(10.0);
            
            // Reload cancel timing
            ui.label("Reload Cancel Timing (ms between keys):");
            let mut reload_timing = self.state.get_reload_cancel_timing() as f32;
            if ui.add(egui::Slider::new(&mut reload_timing, 30.0..=150.0)).changed() {
                self.state.set_reload_cancel_timing(reload_timing as u64);
            }
            ui.add_space(15.0);
            
            ui.separator();
            ui.add_space(10.0);
            
            // Status
            ui.horizontal(|ui| {
                ui.label("Status:");
                if semiauto || reload_cancel {
                    ui.colored_label(egui::Color32::GREEN, "● ACTIVE");
                } else {
                    ui.colored_label(egui::Color32::GRAY, "● INACTIVE");
                }
            });
        });
        
        // Request repaint to keep UI responsive
        ctx.request_repaint();
    }
}
