/// Application frame for the Iced-based mandel-rs GUI
use crate::comp::mandelbrot_engine::MandelbrotEngine;
use crate::storage::visualization::viz_storage::VizStorage;

pub struct MandelRSApp {
    pub storage: Option<VizStorage>,
    pub engine: Option<MandelbrotEngine>,
    pub auto_start_computation: bool,
    pub computing: bool,
    pub sidebar_visible: bool,
    pub left: String,
    pub right: String,
    pub top: String,
    pub bottom: String,
    pub width: String,
    pub height: String,
    pub max_iteration: String,
}

impl Default for MandelRSApp {
    fn default() -> Self {
        MandelRSApp {
            storage: None,
            engine: None,
            auto_start_computation: true,
            computing: false,
            sidebar_visible: true,

            /*            // Full mandelbrot set
                        left: "-2.1".to_string(),
                        right: "0.75".to_string(),
                        top: "1.25".to_string(),
                        bottom: "-1.25".to_string(),
                        width: "800".to_string(),
                        height: "600".to_string(),
                        max_iteration: "200".to_string(),
            */
            /*            // Deep zoom elephant valley
                        left: "-0.7512".to_string(),
                        right: "-0.7502".to_string(),
                        top: "0.1103".to_string(),
                        bottom: "0.1093".to_string(),
                        width: "800".to_string(),
                        height: "600".to_string(),
                        max_iteration: "2000".to_string(),
            */
            /*            // Spiral region
                        left: "-0.7269".to_string(),
                        right: "-0.7259".to_string(),
                        top: "0.1889".to_string(),
                        bottom: "0.1879".to_string(),
                        width: "2800".to_string(),
                        height: "1800".to_string(),
                        max_iteration: "2000".to_string(),
            */
            // Seahorse valley
            left: "-0.7463".to_string(),
            right: "-0.7453".to_string(),
            top: "0.1102".to_string(),
            bottom: "0.1092".to_string(),
            width: "1200".to_string(),
            height: "800".to_string(),
            max_iteration: "2000".to_string(),
        }
    }
}

pub fn launch() -> iced::Result {
    iced::application(
        "Mandelbrot Fractal Visualizer",
        super::update::update,
        super::view::view,
    )
    .subscription(super::subscription::subscription)
    .run()
}

// end of file
