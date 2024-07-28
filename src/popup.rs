use std::env;
use std::sync::Arc;
use std::sync::mpsc::{Sender};
use egui::Context;

pub struct Popup {
    sender: Sender<Choice>,
    verbose: bool,
}

pub enum Choice{
    Yes,
    No,
}

impl Popup {
    pub fn new(sender: Sender<Choice>, verbose: bool) -> Self {
        Self {
            sender,
            verbose,
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
                                    if self.verbose {println!("(Popup) User chose to proceed.");}
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                                }
                                Err(_) => { if self.verbose {eprintln!("(Popup) Error sending choice.")} }
                            }
                        }
                        if ui.button("No").clicked() {
                            match self.sender.send(Choice::No) {
                                Ok(_) => {
                                    if self.verbose {println!("(Popup) User chose not to proceed.")};
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                                Err(_) => { if self.verbose{eprintln!("(Popup) Error sending choice.")} }
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

pub fn run_popup(sender: Sender<Choice>,verbose:bool) {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.drag_and_drop = Some(false);
    native_options.viewport.maximize_button = Some(false);
    native_options.viewport.minimize_button = Some(false);
    native_options.viewport.resizable = Some(false);
    native_options.viewport.inner_size = Some(egui::vec2(300.0, 120.0));
    native_options.centered = true;
    native_options.viewport.drag_and_drop = Some(false);

    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let assets_path = exe_path.parent().unwrap().join("assets");
    let logo_path=assets_path.join("logo.png");

    let icon = image::open(&logo_path).expect("Failed to open icon path").to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    native_options.viewport.icon = Some(Arc::from(egui::IconData {
        rgba: icon.into_raw(),
        width: icon_width,
        height: icon_height,
    }));

    eframe::run_native(
        "Back-up",
        native_options,
        Box::new(move |_cc| Box::new(Popup::new(sender,verbose))),
    ).expect("Terminated");
}
