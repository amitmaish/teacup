#![allow(dead_code)]

pub struct UI {
    root_item: TeacupPrimative,
    background_color: [f64; 3],
    size: (u64, u64),
}

pub enum Unit {
    Pixel(u64),
    MM(f64),
    Inch(f64),
}

impl Default for Unit {
    fn default() -> Self {
        Self::Pixel(0)
    }
}

#[derive(Default)]
pub enum Size {
    Fixed,
    #[default]
    Fit,
    Grow,
    Custom,
}

#[derive(Default)]
pub enum LayoutMode {
    Vertical,
    #[default]
    Horizontal,
    Custom,
}

pub enum TeacupPrimative {
    Rectangle(Rectangle),
}

impl TeacupPrimative {
    pub fn compute_layout() {}
}

#[derive(Default)]
pub struct Rectangle {
    min_width: Option<Unit>,
    max_width: Option<Unit>,
    min_height: Option<Unit>,
    max_height: Option<Unit>,
    padding: Unit,
    child_gap: Unit,
    background_color: [f64; 4],
    size_mode: Size,
    layout_mode: LayoutMode,
    children: Vec<TeacupPrimative>,
}
