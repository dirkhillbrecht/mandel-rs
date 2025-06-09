// Main program for the mandel-rs project

mod data_point;
mod data_plane;
mod data_storage;
mod simple_mandelbrot;

use crate::data_point::DataPoint;
use crate::data_storage::DataStorage;

fn main() {
    let point = DataPoint::new(15,2.3,-5.6);
    println!("Hello, world, created a data point: {:?}",point);
    let mut storage: DataStorage = DataStorage::new(-2.3,1.0,-1.5,1.5,100,100,150);
    println!("Storage created, now computing mandelbrot set…");
    simple_mandelbrot::compute_mandelbrot(&mut storage);
    println!("Computation finished");

    let test_point=storage.plane().get(50,50);
    println!("Center point: {:?}", test_point);
    let edge_point=storage.plane().get(2, 2);
    println!("Edge point: {:?}", edge_point);
}

// end of file
