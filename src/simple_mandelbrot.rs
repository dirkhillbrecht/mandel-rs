// Most basic and simple implementation of a mandelbrot computation algorithm

use crate::data_point::DataPoint;
use crate::DataStorage;

pub fn compute_mandelbrot(storage: &mut DataStorage) {
    let x_dist=storage.x_max()-storage.x_min();
    let y_dist=storage.y_max()-storage.y_min();
    let x_pix=x_dist/(storage.plane().width()-1) as f64;
    let y_pix=y_dist/(storage.plane().height()-1) as f64;
    let max_iteration=storage.max_iteration();
    for x in 0..storage.plane().width() {
        let x_coo=storage.x_min()+x as f64*x_pix;
        for y in 0..storage.plane().height() {
            let y_coo=storage.y_max()-y as f64*y_pix;
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
