/// Storage implementation for mandel.rs
///
/// This module contains all files which implement the data storage for mandel.rs data.
/// It is split into two parts:
///
/// 1. Computational storage is used for the computation algorithms.
/// 2. Visualization storage is used for the program parts actually showing data.
///
/// Having these two storage types separate from each other decouples both functions
/// and allows the computation algorithms highly efficient access to the storage.

// Subfolders
pub mod computation;
pub mod visualization;
pub mod event;

// Files herein
pub mod data_point;
pub mod image_comp_properties;

// end of file
