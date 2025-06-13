// Most basic and simple implementation of a mandelbrot computation algorithm

use crate::data_point::DataPoint;
use crate::data_storage::DataStorage;

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
