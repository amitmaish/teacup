#![allow(dead_code)]

use std::sync::{Arc, Weak};

use glm::Vec3;
use tokio::sync::Mutex;

use crate::renderer_backend::mesh_builder::{Mesh, make_rectangle};

pub struct UI {
    root_item: Arc<Mutex<Container>>,
    background_color: [f64; 3],
    size: (u64, u64),
}

#[derive(Default)]
pub struct Size {
    width: usize,
    height: usize,
}

#[derive(Default)]
pub enum SizingMode {
    Fixed,
    #[default]
    Fit,
    Grow,
    Custom,
}

#[derive(Default)]
pub struct Sizing {
    width: SizingMode,
    height: SizingMode,
}

#[derive(Default)]
pub struct Bounds<T> {
    min: Option<T>,
    max: Option<T>,
}

#[derive(Default)]
pub enum LayoutMode {
    TopToBottom,
    #[default]
    LeftToRight,
    Custom,
}

pub enum Primative {
    Container(Container),
}

pub enum Container {
    Rectangle(Rectangle),
    Scroll(ScrollContainer),
}

#[derive(Default)]
pub struct TCPrimative {
    width: Bounds<usize>,
    height: Bounds<usize>,
    size: Size,
    sizing: Sizing,
    parent: Weak<Mutex<Container>>,
}

#[derive(Default)]
pub struct TCContainer {
    primative: TCPrimative,
    padding: usize,
    child_gap: usize,
    background_color: [f64; 4],
    layout_mode: LayoutMode,
    children: Vec<Arc<Mutex<Primative>>>,
}

#[derive(Default)]
pub struct Rectangle {
    container: TCContainer,
}

#[derive(Default)]
pub struct ScrollContainer {
    container: TCContainer,
    scroll_amount: f64,
}

pub fn make_ss_rectangle(x: i16, y: i16, w: i16, h: i16, color: Vec3, size: (i32, i32)) -> Mesh {
    make_rectangle(
        (x as f32 / size.0 as f32) - 1.0,
        1.0 - (y as f32 / size.1 as f32),
        w as f32 / size.0 as f32,
        h as f32 / size.1 as f32,
        color,
    )
}
