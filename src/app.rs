use std::borrow::Cow;
use std::env;
use std::fs::File;
use egui::{Context, TextureHandle, TextureOptions};
use std::future::Future;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use gif::{Frame};
use image::{DynamicImage, RgbaImage};
use crate::config::Config;

pub struct BackupApp {
    source_channel: (Sender<String>, Receiver<String>),
    destination_channel: (Sender<String>, Receiver<String>),
    source: String,
    destination: String,
    extension: String,
    prev_extension: String,
    current_frame: usize,
    frames_rect: Vec<Frame<'static>>,
    rect_texture: Option<TextureHandle>,
    frames_line: Vec<Frame<'static>>,
    line_texture: Option<TextureHandle>,
    frame_duration: Duration,
    start_time: std::time::Instant,
    is_focused: bool
}

impl Default for BackupApp {
    fn default() -> Self {
        let mut app = Self {
            source_channel: channel(),
            destination_channel: channel(),
            source: String::from("No folder selected"),
            destination: String::from("No folder selected"),
            extension: "".to_string(),
            prev_extension: "".to_string(),
            current_frame: 0,
            frames_rect: Vec::new(),
            frame_duration: Duration::from_millis(25),
            start_time: std::time::Instant::now(),
            rect_texture: None,
            frames_line: Vec::new(),
            line_texture: None,
            is_focused: false
        };

        app.load_config();
        app
    }
}

impl BackupApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>,frames_rect: Vec<Frame<'static>>,frames_line: Vec<Frame<'static>>) -> Self {
        let mut app=BackupApp::default();
        for f in frames_rect.iter(){
            app.frames_rect.push(f.to_owned());
        }
        for f in frames_line.iter(){
            app.frames_line.push(f.to_owned());
        }
        app
    }

    fn save_config(&self) {
        let config = Config::new(self.source.clone(), self.destination.clone(), self.extension.clone());
        config.save();
    }

    fn load_config(&mut self) {
        let mut config = Config::default();
        config.load();
        self.source = config.source;
        self.destination = config.destination;
        self.extension = config.extension;
    }
}

impl eframe::App for BackupApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        ctx.input(|i|{
            if i.focused {
                if !self.is_focused {
                    self.is_focused = true;
                    self.load_config();
                }
            }
            else { self.is_focused = false;}
        });
        if self.is_focused {ctx.request_repaint();}

        if let Ok(source) = self.source_channel.1.try_recv() {
            self.source = source;
            self.save_config();
        }

        if let Ok(destination) = self.destination_channel.1.try_recv() {
            self.destination = destination;
            self.save_config();
        }

        if self.extension != self.prev_extension {
            self.prev_extension = self.extension.clone();
            self.save_config();
        }

        egui::CentralPanel::default().show(ctx, |ui| {

            let response = ui.button("Instructions");
            let popup_id = ui.make_persistent_id("info");
            if response.clicked() {
                ui.memory_mut(|mem| mem.toggle_popup(popup_id));
            }
            let below = egui::AboveOrBelow::Below;
            egui::popup::popup_above_or_below_widget(ui, popup_id, &response, below, |ui| {
                ui.set_min_width(200.0);
                ui.label("To start backup you have to move the pointer to the top-left of the screen and move anti-clockwise along the edges.");

                let elapsed = self.start_time.elapsed();
                let new_frame = (elapsed.as_millis() / self.frame_duration.as_millis() as u128) as usize % self.frames_rect.len();
                if new_frame != self.current_frame{
                    self.current_frame = new_frame;
                    let mut image = merge_frames(&self.frames_rect[..self.current_frame+1]);
                    image=image.resize(100, 50, image::imageops::FilterType::Nearest);
                    let size = [image.width() as usize, image.height() as usize];
                    let image_buffer = image.into_rgba8();

                    let pixels: Vec<_> = image_buffer.into_vec();

                    let egui_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

                    self.rect_texture = Some(ctx.load_texture("rectangle_animation", egui_image, TextureOptions::default()));

                    // Line
                    let mut image = merge_frames(&self.frames_line[..self.current_frame+1]);
                    image=image.resize(100, 50, image::imageops::FilterType::Nearest);
                    let size = [image.width() as usize, image.height() as usize];
                    let image_buffer = image.into_rgba8();

                    let pixels: Vec<_> = image_buffer.into_vec();

                    let egui_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

                    self.line_texture = Some(ctx.load_texture("line_animation", egui_image, TextureOptions::default()));
                }

                if let Some(texture) = &self.rect_texture {
                        ui.image(texture);
                }

                ui.label("To confirm, draw an horizontal line from left to right clicking the right mouse button.");

                if let Some(texture) = &self.line_texture {
                    ui.image(texture);
                }

                ctx.request_repaint();
            });



            ui.vertical_centered(|ui| {
                ui.heading("Welcome to the Back-up App!");

                ui.add_space(20.0);

                ui.label(format!("Source: {}", self.source.clone()));
                if ui.button("Select folder to backup").clicked() {
                    let sender = self.source_channel.0.clone();
                    let task = rfd::AsyncFileDialog::new().pick_folder();
                    let ctx = ui.ctx().clone();
                    execute(async move {
                        let folder = task.await;
                        if let Some(folder) = folder {
                            let path = folder.path();
                            println!("Selected folder: {:?}", path);
                            sender.send(path.to_string_lossy().to_string()).unwrap();

                            ctx.request_repaint();
                        }
                    });
                }

                ui.add_space(10.0);
                ui.label(format!("Destination: {}", self.destination.clone()));
                if ui.button("Select folder to save backup").clicked() {
                    let sender = self.destination_channel.0.clone();
                    let task = rfd::AsyncFileDialog::new().pick_folder();
                    let ctx = ui.ctx().clone();
                    execute(async move {
                        let folder = task.await;
                        if let Some(folder) = folder {
                            let path = folder.path();
                            println!("Selected folder: {:?}", path);
                            sender.send(path.to_string_lossy().to_string()).unwrap();
                            ctx.request_repaint();
                        }
                    });
                }

                ui.add_space(20.0);

                ui.vertical_centered(|ui| {
                    ui.label(format!(
                        "Selected extension: {}",
                        if self.extension.is_empty() {
                            "Any".to_string()
                        } else {
                            format!(".{}", self.extension.clone())
                        }
                    ));
                    ui.add(egui::TextEdit::singleline(&mut self.extension).hint_text("Enter extension").desired_width(100.0));

                });
                
                ui.add_space(20.0);
                if ui.button("Close").clicked() {
                    self.save_config();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                }
                ui.add_space(5.0);
                ui.label("Ctrl+T to open this window again.");
            });
        });
    }
}

fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}

pub fn run_backup_app() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    native_options.viewport.inner_size = Some(egui::vec2(400.0, 310.0));


    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let assets_path = exe_path.parent().unwrap().join("assets");
    let logo_path=assets_path.join("logo.png");
    let rectangle_path=assets_path.join("rectangle_animation.gif");
    let line_path=assets_path.join("line_animation.gif");

    let frames_rect:Vec<Frame<'static>> = load_gif_frames(rectangle_path.to_str().unwrap().to_string());
    let frames_line:Vec<Frame<'static>> = load_gif_frames(line_path.to_str().unwrap().to_string());

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
        Box::new(|cc| Box::new(BackupApp::new(cc,frames_rect,frames_line))),
    ).expect("Terminated");
}

fn load_gif_frames(path: String) -> Vec<Frame<'static>> {
    let file = File::open(path.as_str()).unwrap();
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info(file).unwrap();
    let mut frames = Vec::new();

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        frames.push(frame.clone());
    }

    frames
}

fn merge_frames(frames: &[Frame]) -> DynamicImage {
    let width = frames[0].width as u32;
    let height = frames[0].height as u32;
    let mut image = RgbaImage::new(width, height);

    // Initialize the image with transparent pixels
    for pixel in image.pixels_mut() {
        *pixel = image::Rgba([0, 0, 0, 0]);
    }

    for frame in frames {
        let x = frame.left as u32;
        let y = frame.top as u32;
        let frame_width = frame.width as u32;
        let frame_height = frame.height as u32;

        // Convert Cow to a slice
        let frame_buffer: &[u8] = match &frame.buffer {
            Cow::Borrowed(slice) => slice,
            Cow::Owned(ref vec) => vec,
        };

        for row in 0..frame_height {
            for col in 0..frame_width {
                let idx = ((row * frame_width) + col) as usize * 4;
                let r = frame_buffer[idx];
                let g = frame_buffer[idx + 1];
                let b = frame_buffer[idx + 2];
                let a = frame_buffer[idx + 3];

                let target_x = x + col;
                let target_y = y + row;

                if target_x < width && target_y < height && a==255{
                    image.put_pixel(target_x, target_y, image::Rgba([r, g, b, a]));
                }
            }
        }
    }


    DynamicImage::ImageRgba8(image)
}
