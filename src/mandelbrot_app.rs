// Local GUI for mandel-rs

use eframe::{self};
use egui;
use crate::{data_storage::DataStorage, simple_mandelbrot};

pub struct MandelbrotApp {
    storage: Option<DataStorage>,
    computing: bool,
    left: String,
    right: String,
    top: String,
    bottom: String,
    width: String,
    height: String,
    max_iteration: String,
}

impl MandelbrotApp {
    pub fn new() -> Self {
        MandelbrotApp {
            storage: None,
            computing: false,
            left: "-2.1".to_string(),
            right: "0.75".to_string(),
            top: "1.25".to_string(),
            bottom: "-1.25".to_string(),
            width: "800".to_string(),
            height: "600".to_string(),
            max_iteration: "200".to_string()
        }
    }
    fn iteration_to_color(it: u32, maxit: u32) -> [u8; 3] {
        if it==maxit {
            [0,0,0]
        }
        else {
            // Some simple color gradient
            let ratio=it as f32/maxit as f32;
            let xor=((it%2)*255) as u8;
            let red=((255.0 * ratio * 5.0) % 255.0) as u8 ^ xor;
            let green=((255.0 * (1.0 - ratio) *3.0) % 255.0) as u8 ^ xor;
            let blue=((128.0 + 127.0*ratio*2.0) % 255.0) as u8 ^ xor;
            [red,green,blue]
        }        
    }
    fn draw_fractal(&self,ui: &mut egui::Ui, storage: &DataStorage) {
        let width=storage.plane().width();
        let height=storage.plane().height();

        // Create pixels
        let mut pixels=Vec::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(point)=storage.plane().get(x, y) {
                    let color=Self::iteration_to_color(point.iteration_count(), storage.max_iteration());
                    pixels.extend_from_slice(&color);
                }
                else {
                    pixels.extend_from_slice(&[255,0,255]);
                }
            }
        }
        let color_image = egui::ColorImage::from_rgb([width,height], &pixels);
        let texture=ui.ctx().load_texture("mandelbrot", color_image, egui::TextureOptions::default());
        ui.image(&texture);
    }
}

impl eframe::App for MandelbrotApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello Mandelbrot");
            ui.horizontal(|ui| {
                ui.label("Left: ");
                ui.text_edit_singleline(&mut self.left);
                ui.label("Right: ");
                ui.text_edit_singleline(&mut self.right);
            });
            ui.horizontal(|ui| {
                ui.label("Top: ");
                ui.text_edit_singleline(&mut self.top);
                ui.label("Bottom: ");
                ui.text_edit_singleline(&mut self.bottom);
            });
            ui.horizontal(|ui| {
                ui.label("Width: ");
                ui.text_edit_singleline(&mut self.width);
                ui.label("Height: ");
                ui.text_edit_singleline(&mut self.height);
                ui.label("Max. iteration: ");
                ui.text_edit_singleline(&mut self.max_iteration);
            });
            if ui.button("Compute Mandelbrot").clicked() {
                if let (Ok(left), Ok(right), Ok(bottom), Ok(top),
                        Ok(width), Ok(height), Ok(max_iteration)) = (
                    self.left.parse::<f64>(),
                    self.right.parse::<f64>(),
                    self.bottom.parse::<f64>(),
                    self.top.parse::<f64>(),
                    self.width.parse::<u32>(),
                    self.height.parse::<u32>(),
                    self.max_iteration.parse::<u32>(),
                ) {
                    println!("Compute started");
                    self.computing=true;
                    let mut storage = DataStorage::new(left,right,bottom,top,
                        width,height,max_iteration);
                    simple_mandelbrot::compute_mandelbrot(&mut storage);
                    self.storage=Some(storage);
                    self.computing=false;
                    println!("Compute ended");
                }
                else {
                    println!("Problem with input data");
                }
            }
            if self.computing {
                ui.label("Computing…");
            }
            else {
                if let Some(storage) = &self.storage {
                    ui.label(format!("Computed {}*{} fractal",storage.plane().width(),storage.plane().height()));
                }
                else {
                    ui.label("Mandelbrot computation idle.");
                }
            }
            if let Some(storage)=&self.storage {
                self.draw_fractal(ui, storage);
            }
        });
    }
}

// end of file
