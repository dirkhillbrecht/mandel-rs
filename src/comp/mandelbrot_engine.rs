// Most basic and simple implementation of a mandelbrot computation algorithm

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use euclid::Point2D;
use rand::rng;
use rand::seq::SliceRandom;

use crate::storage::computation::comp_storage::CompStorage;
use crate::storage::coord_spaces::MathSpace;
use crate::storage::data_point::DataPoint;
use crate::storage::image_comp_properties::StageState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EngineState {
    PreStart,
    Running,
    Finished,
    Aborted,
}

pub struct MandelbrotEngine {
    #[allow(dead_code)] // Could be needed for deriving images from the original coordinates
    pub state: Arc<Mutex<EngineState>>,
    storage: Arc<CompStorage>,
    thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    stop_flag: Arc<AtomicBool>,
}

impl MandelbrotEngine {
    /// Create a new MandelbrotEngine for the given image computation properties
    /// The engine has _not_ started computation after
    pub fn new(storage: &Arc<CompStorage>) -> Self {
        MandelbrotEngine {
            state: Arc::new(Mutex::new(EngineState::PreStart)),
            storage: storage.clone(),
            thread_handle: Arc::new(Mutex::new(None)),
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Return the current state of the engine
    pub fn state(&self) -> EngineState {
        *self.state.lock().unwrap()
    }

    pub fn start(&self) {
        // Check if computation is already running
        // This block can only be entered _once_ at the same time, so the state test and change is atomic from the outside.
        {
            let mut state = self.state.lock().unwrap();
            if matches!(*state, EngineState::Running) {
                return;
            }
            *state = EngineState::Running;
        }

        // From hereon it is clear that computation is neither running nor in the process of being started
        // Reset stop flag
        self.stop_flag.store(false, Ordering::Relaxed);

        // Prepare starting the thread by creating moveable clones of the needed data
        let storage_for_thread = self.storage.clone();
        let state_for_thread = self.state.clone();
        let stop_flag_for_thread = self.stop_flag.clone();

        // Now spawn the computation thread
        let handle = thread::spawn(move || {
            // Perform the computation
            let result =
                stoppable_compute_mandelbrot_shuffled(&storage_for_thread, &stop_flag_for_thread);
            // Update the state once computation is either finished or aborted
            let mut state = state_for_thread.lock().unwrap();
            *state = if result {
                EngineState::Finished
            } else {
                EngineState::Aborted
            };
        });

        // Store the thread handle
        let mut thread_handle = self.thread_handle.lock().unwrap();
        *thread_handle = Some(handle);
    }

    pub fn stop(&self) {
        // Signal stop
        self.stop_flag.store(true, Ordering::Relaxed);

        // Wait for the thread to finish
        // Note: This needs to be redesigned, stopping should not block. Perhaps an additional engine state "Stopping"
        let mut thread_handle = self.thread_handle.lock().unwrap();
        if let Some(handle) = thread_handle.take() {
            handle.join().unwrap();
        }
    }

    pub fn storage(&self) -> Arc<CompStorage> {
        self.storage.clone()
    }
}

fn coord_sort_idx<T>(p: &Point2D<u32, T>) -> u32 {
    p.x.trailing_zeros().min(p.y.trailing_zeros())
}

fn order_coords<T>(p: &Point2D<u32, T>, q: &Point2D<u32, T>) -> std::cmp::Ordering {
    coord_sort_idx(p).cmp(&coord_sort_idx(q)).reverse()
}

fn stoppable_compute_mandelbrot_shuffled(storage: &CompStorage, stop_flag: &AtomicBool) -> bool {
    let max_iteration = storage.properties.max_iteration;
    let height = storage.properties.stage_properties.pixels.height as i32;
    let width = storage.properties.stage_properties.pixels.width as i32;
    let mut coords: Vec<Point2D<u32, MathSpace>> = Vec::with_capacity((height * width) as usize);
    let mut ycoo = Vec::with_capacity(height as usize);
    let mut xcoo = Vec::with_capacity(width as usize);
    for x in 0..width {
        xcoo.push(storage.properties.stage_properties.x(x));
    }
    for y in 0..height {
        ycoo.push(storage.properties.stage_properties.y(y));
        for x in 0..width {
            coords.push(Point2D::new(x as u32, y as u32));
        }
    }
    coords.shuffle(&mut rng());
    coords.sort_by(order_coords); // Needs appropriate presentation code, otherwise looks a bit strange
    let mut count = 0;
    let mut do_comp = true;
    storage.stage.set_state(StageState::Evolving);
    coords.into_iter().for_each(|point| {
        if do_comp {
            count += 1;
            if count % 1000 == 0 {
                if stop_flag.load(Ordering::Relaxed) {
                    storage.stage.set_state(StageState::Stalled);
                    do_comp = false; // Computation was aborted
                }
            }
            if !storage.stage.is_computed(point.x, point.y) {
                storage.stage.set(
                    point.x,
                    point.y,
                    data_point_at(
                        *(xcoo.get(point.x as usize).unwrap()),
                        *(ycoo.get(point.y as usize).unwrap()),
                        max_iteration,
                    ),
                );
            }
        }
    });
    storage.stage.set_state(StageState::Completed);
    true // Computation ended successfully
}

#[allow(dead_code)] // Currently not needed, but may be useful for testing or as blueprint for other algorithms
fn stoppable_compute_mandelbrot_linear(storage: &CompStorage, stop_flag: &AtomicBool) -> bool {
    let max_iteration = storage.properties.max_iteration;
    storage.stage.set_state(StageState::Evolving);
    for y in 0..storage.properties.stage_properties.pixels.height {
        // Check for cancellation every row, this is only interim as way too inflexible!
        if stop_flag.load(Ordering::Relaxed) {
            storage.stage.set_state(StageState::Stalled);
            return false; // Computation was aborted
        }
        let y_coo = storage.properties.stage_properties.y(y as i32);
        for x in 0..storage.properties.stage_properties.pixels.width {
            let x_coo = storage.properties.stage_properties.x(x as i32);
            if !storage.stage.is_computed(x, y) {
                storage
                    .stage
                    .set(x, y, data_point_at(x_coo, y_coo, max_iteration));
            }
        }
    }
    storage.stage.set_state(StageState::Completed);
    true // Computation ended successfully
}

// This is the actual mandelbrot set iteration depth computation algorithm, somehow the same as in 1978…
pub fn data_point_at(c_real: f64, c_imag: f64, max_iteration: u32) -> DataPoint {
    let mut z_real = 0.0;
    let mut z_imag = 0.0;
    for i in 0..max_iteration {
        let z_real_square = z_real * z_real;
        let z_imag_square = z_imag * z_imag;
        let z_real_new = z_real_square - z_imag_square + c_real;
        let z_imag_new = 2.0 * z_real * z_imag + c_imag;
        if z_real_square + z_imag_square > 4.0 {
            // make this configurable later
            return DataPoint::computed(i, Point2D::new(z_real_new, z_imag_new));
        }
        z_real = z_real_new;
        z_imag = z_imag_new;
    }
    // Final iteration must compute one more loop
    let z_real_square = z_real * z_real;
    let z_imag_square = z_imag * z_imag;
    let z_real_new = z_real_square - z_imag_square + c_real;
    let z_imag_new = 2.0 * z_real * z_imag + c_imag;
    return DataPoint::computed(max_iteration, Point2D::new(z_real_new, z_imag_new));
}

// end of file
