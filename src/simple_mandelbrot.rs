// Most basic and simple implementation of a mandelbrot computation algorithm

use crate::data_point::DataPoint;
use crate::data_storage::{DataStorage,ComputeProgress};
use tokio::sync::mpsc;
use std::sync::Arc;

pub async fn compute_in_background(
    left: f64, right: f64, bottom: f64, top: f64,
    width: u32, height: u32, max_iteration: u32,
    progress_sender: mpsc::Sender<ComputeProgress>
) {
    // Create static data storage
    let mut storage = DataStorage::new(left, right, bottom, top, width, height, max_iteration);
    let total_pixels=(width*height) as usize;

    // Now compute the stuff
    let mut completed_pixels = 0;
    for x in 0..storage.plane().width() {
        let x_coo=storage.plane().x(x);
        for y in 0..storage.plane().height() {
            let y_coo=storage.plane().y(y);
            storage.plane_mut().set(x,y,data_point_at(x_coo,y_coo,max_iteration));
            completed_pixels += 1;

            // Now send progress information
            if completed_pixels % 10000 ==0 {
                let progress = ComputeProgress::new(completed_pixels,total_pixels);
                let _ = progress_sender.send(progress).await;
            }
        }
    }
    let final_progress = ComputeProgress::new(total_pixels,total_pixels);
    let _ = progress_sender.send(progress).await;
}

pub fn compute_mandelbrot(storage: &mut DataStorage) {
    let max_iteration=storage.max_iteration();
    for x in 0..storage.plane().width() {
        let x_coo=storage.plane().x(x);
        for y in 0..storage.plane().height() {
            let y_coo=storage.plane().y(y);
            storage.plane_mut().set(x,y,data_point_at(x_coo,y_coo,max_iteration));
        }
    }
}

pub fn data_point_at(c_real:f64,c_imag:f64,max_iteration:u32) -> DataPoint {
    let mut z_real=0.0;
    let mut z_imag=0.0;
    for i in 0..max_iteration {
        let z_real_square=z_real*z_real;
        let z_imag_square=z_imag*z_imag;
        let z_real_new=z_real_square-z_imag_square+c_real;
        let z_imag_new=2.0*z_real*z_imag+c_imag;
        if z_real_square+z_imag_square>4.0 { // make this configurable later
            return DataPoint::new(i,z_real_new,z_imag_new);
        }
        z_real=z_real_new;
        z_imag=z_imag_new;
    }
    // Final iteration must compute one more loop
    let z_real_square=z_real*z_real;
    let z_imag_square=z_imag*z_imag;
    let z_real_new=z_real_square-z_imag_square+c_real;
    let z_imag_new=2.0*z_real*z_imag+c_imag;
    return DataPoint::new(max_iteration,z_real_new,z_imag_new);
}

// end of file
