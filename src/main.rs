use eframe::egui;
use egui::{RichText, FontId};
use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Indecisive Timer", native_options, Box::new(|cc| Ok(Box::new(TimerApp::new(cc))))).expect("An error occured creating the window");
}

struct Event {
    enabled: bool,
    chance: f64,
    changes: fn(f64) -> f64,
}

#[derive(Default)]
struct TimerApp {
    time_remaining: f64,
    last_time: f64,
    last_second: f64,
    started: bool,
    events: Vec<Event>,
    rng: ThreadRng,
}

impl TimerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        
        Self::default()
    }
}

fn format_time(time: f64) -> String {
    let hours = time as u64 / 3600;
    let minutes = time as u64 / 60;
    let seconds = time as u64 % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

impl eframe::App for TimerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.ctx().request_repaint(); // Make sure this is rerun every frame regardless of if window is focused
            if self.started == false {
                self.time_remaining = 120.0; // set initial time
                self.last_second = ui.ctx().time();
                // RANDOM EVENTS
                self.events.push(Event {
                    enabled: true,
                    chance: 0.05, // 5% chance
                    changes: |time_remaining| time_remaining * 2.0,
                });
                self.events.push(Event {
                    enabled: true,
                    chance: 0.05, // 5% chance
                    changes: |time_remaining| time_remaining / 2.0,
                });
                self.rng = rand::rng();
                

                self.started = true;
            } else {
                let now = ui.ctx().time();
                let last = self.last_time;
                self.time_remaining -= now - last;
                self.last_time = now;
                if now - self.last_second >= 1.0 {
                    for event in self.events.iter() {
                        if event.enabled == true {
                            if rand::random_bool(event.chance) {
                                self.time_remaining = (event.changes)(self.time_remaining);
                            }
                        }
                    }
                    self.last_second = now;
                }
            }
            ui.label(RichText::new(format!("{}", format_time(self.time_remaining))).font(FontId::proportional(64.0)));
        });
    }
}