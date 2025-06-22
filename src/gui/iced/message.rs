/// Messages passed around in the Iced-based mandel-rs-GUI
#[derive(Debug, Clone)]
pub enum Message {
    ToggleSidebar,
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
}

// end of file
