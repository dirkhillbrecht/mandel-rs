use crate::{
    comp::math_data::MathPreset,
    gui::iced::app::ImageRenderScheme,
    storage::visualization::coloring::presets::{GradientColorPreset, IterationAssignment},
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
    #[allow(dead_code)]
    MousePressed(iced::Point),
    #[allow(dead_code)]
    MouseDragged(iced::Point),
    #[allow(dead_code)]
    MouseReleased(iced::Point),
}

// end of file
