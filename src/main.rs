// Main program for the mandel-rs project

mod data_point;
mod data_plane;
mod data_storage;

use crate::data_point::DataPoint;
use crate::data_plane::DataPlane;
use crate::data_storage::DataStorage;

fn main() {
    let point = DataPoint::new(15,2.3,-5.6);
    println!("Hello, world, created a data point: {:?}",point);
//    let storage: DataStorage = DataStorage::new(640, 480);
//    storage.set(0, 0, point);
//    println!("I've got a storage now: {:?}",storage);
}

// end of file
