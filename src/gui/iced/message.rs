use euclid::Vector2D;
use iced::Point;

use crate::{
    comp::math_data::MathPreset,
    gui::iced::app::ImageRenderScheme,
    storage::{
        coord_spaces::StageSpace,
        visualization::coloring::presets::{GradientColorPreset, IterationAssignment},
    },
};

/// Messages passed around in the Iced-based mandel-rs-GUI
#[derive(Debug, Clone)]
pub enum Message {
    ToggleSidebar,
    PresetChanged(MathPreset),
    PresetClicked,
    LeftChanged(String),
    RightChanged(String),
    TopChanged(String),
    BottomChanged(String),
    WidthChanged(String),
    HeightChanged(String),
    MaxIterationChanged(String),
    ComputeClicked,
    StopClicked,
    UpdateViz,
    ColorSchemeChanged(GradientColorPreset),
    IterationAssignmentChanged(IterationAssignment),
    RenderSchemeChanged(ImageRenderScheme),
    ShiftStage(Vector2D<i32, StageSpace>),
    ZoomStart((Point, i32)),
    ZoomTick(i32),
    ZoomEndCheck,
    #[allow(dead_code)]
    MousePressed(iced::Point),
    #[allow(dead_code)]
    MouseDragged(iced::Point),
    #[allow(dead_code)]
    MouseReleased(iced::Point),
}

// end of file
