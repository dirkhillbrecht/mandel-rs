// Local GUI for mandel-rs

use eframe::{self, Storage};
use egui;
use crate::{data_storage::DataStorage, simple_mandelbrot};

pub struct MandelbrotApp {
    storage: Option<DataStorage>,
    computing: bool,

}

impl MandelbrotApp {
    pub fn new() -> Self {
        MandelbrotApp { storage: None, computing: false}
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
            if ui.button("Compute Mandelbrot").clicked() {
                println!("Compute started");
                self.computing=true;
                let mut storage = DataStorage::new(-2.0,1.0,-0.3,1.5,800,800,150);
                simple_mandelbrot::compute_mandelbrot(&mut storage);
                self.storage=Some(storage);
                self.computing=false;
                println!("Compute ended");
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
