use crate::{
    comp::math_data::MathPreset,
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
}

// end of file
