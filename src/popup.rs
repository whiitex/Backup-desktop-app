use std::sync::Arc;
use std::sync::mpsc::{Sender};
use egui::Context;

pub struct Popup {
    sender: Sender<Choice>
}

pub enum Choice{
    Yes,
    No,
}

impl Popup {
    pub fn new(sender: Sender<Choice>) -> Self {
        Self {
            sender
        }
    }
}
impl eframe::App for Popup {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Do you want to begin backup?");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.centered_and_justified(|ui| {
                        if ui.button("Yes").clicked() {
                            match self.sender.send(Choice::Yes) {
                                Ok(_) => {
                                    println!("(Popup) User chose to proceed.");
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                                }
                                Err(_) => { eprintln!("(Popup) Error sending choice.") }
                            }
                        }
                        if ui.button("No").clicked() {
                            match self.sender.send(Choice::No) {
                                Ok(_) => {
                                    println!("(Popup) User chose not to proceed.");
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                                Err(_) => { eprintln!("(Popup) Error sending choice.") }
                            }
                        }
                    });
                });
                ui.add_space(10.0);
                ui.label("You can also use a gesture");
            });
        });
    }
}

pub fn run_popup(sender: Sender<Choice>) {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.drag_and_drop = Some(false);
    native_options.viewport.maximize_button = Some(false);
    native_options.viewport.minimize_button = Some(false);
    native_options.viewport.resizable = Some(false);
    native_options.viewport.inner_size = Some(egui::vec2(300.0, 120.0));
    native_options.centered = true;
    native_options.viewport.drag_and_drop = Some(false);

    let icon = image::open("./assets/logo.png").expect("Failed to open icon path").to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    native_options.viewport.icon = Some(Arc::from(egui::IconData {
        rgba: icon.into_raw(),
        width: icon_width,
        height: icon_height,
    }));

    eframe::run_native(
        "Back-up",
        native_options,
        Box::new(|_cc| Box::new(Popup::new(sender))),
    ).expect("Terminated");
}
