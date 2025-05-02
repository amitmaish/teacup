#![allow(dead_code)]

use std::sync::{Arc, Mutex, Weak};

use log::{Level, log};

pub struct UI {
    root_item: Arc<Mutex<dyn Container>>,
    background_color: [f64; 3],
    size: (u16, u16),
}

impl Container for UI {
    fn fit_sizing(&mut self) {
        if let Ok(mut container) = self.root_item.lock() {
            container.fit_sizing()
        }
    }

    fn set_child_positions(&mut self) {
        if let Ok(mut root) = self.root_item.lock() {
            root.set_child_positions();
        }
    }
}

#[derive(Default)]
pub enum SizingMode {
    Fixed(u16),
    #[default]
    Fit,
    Grow,
}

#[derive(Default)]
pub struct Sizing {
    width: SizingMode,
    height: SizingMode,
}

#[derive(Default, Clone)]
pub enum LayoutMode {
    TopToBottom,
    #[default]
    LeftToRight,
}

struct TCContainer {}

impl Container for TCContainer {
    fn fit_sizing(&mut self) {
        log!(
            Level::Error,
            "TCContainer can't compute layout as it is just a temp struct. replace with a proper container"
        )
    }

    fn set_child_positions(&mut self) {
        log!(
            Level::Error,
            "TCContainer can't compute layout as it is just a temp struct. replace with a proper container"
        )
    }
}

pub trait Container {
    fn fit_sizing(&mut self);
    fn set_child_positions(&mut self);
}

pub trait Primative {
    fn get_parent(&self) -> Weak<Mutex<dyn Container>>;
    fn set_parent(&mut self, parent: Weak<Mutex<dyn Container>>);

    fn get_width(&self) -> u16;
    fn get_min_width(&self) -> u16;
    fn get_max_width(&self) -> Option<u16>;
    fn get_perfered_width(&self) -> u16;

    fn set_width(&mut self, width: u16);
    fn set_min_width(&mut self, width: u16);
    fn set_max_width(&mut self, width: Option<u16>);
    fn set_perfered_width(&mut self, width: u16);

    fn get_height(&self) -> u16;
    fn get_min_height(&self) -> u16;
    fn get_max_height(&self) -> Option<u16>;
    fn get_perfered_height(&self) -> u16;

    fn set_height(&mut self, height: u16);
    fn set_min_height(&mut self, height: u16);
    fn set_max_height(&mut self, height: Option<u16>);
    fn set_perfered_height(&mut self, height: u16);

    fn get_position(&self) -> (u16, u16);
    fn set_position(&mut self, position: (u16, u16));

    fn as_container(&mut self) -> Option<&mut dyn Container> {
        None
    }
}

pub struct Rectangle {
    width: u16,
    height: u16,
    perfered_width: u16,
    perfered_height: u16,
    min_width: u16,
    min_height: u16,
    max_width: Option<u16>,
    max_height: Option<u16>,
    position: (u16, u16),
    layout_mode: LayoutMode,
    sizing: Sizing,
    padding: u16,
    child_gap: u16,
    color: srgb,
    parent: Weak<Mutex<dyn Container>>,
    children: Vec<Arc<Mutex<dyn Primative>>>,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            parent: Weak::<Mutex<TCContainer>>::new(),

            width: Default::default(),
            height: Default::default(),
            perfered_width: Default::default(),
            perfered_height: Default::default(),
            min_width: Default::default(),
            min_height: Default::default(),
            max_width: Default::default(),
            max_height: Default::default(),
            position: Default::default(),
            layout_mode: Default::default(),
            sizing: Default::default(),
            padding: Default::default(),
            child_gap: Default::default(),
            color: Default::default(),
            children: Default::default(),
        }
    }
}

impl Primative for Rectangle {
    fn get_parent(&self) -> Weak<Mutex<dyn Container>> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: Weak<Mutex<dyn Container>>) {
        self.parent = parent;
    }

    fn get_width(&self) -> u16 {
        self.width
    }

    fn get_min_width(&self) -> u16 {
        self.min_width
    }

    fn get_max_width(&self) -> Option<u16> {
        self.max_width
    }

    fn get_perfered_width(&self) -> u16 {
        self.perfered_width
    }

    fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    fn set_min_width(&mut self, width: u16) {
        self.min_width = width;
    }

    fn set_max_width(&mut self, width: Option<u16>) {
        self.max_width = width;
    }

    fn set_perfered_width(&mut self, width: u16) {
        self.perfered_width = width;
    }

    fn get_height(&self) -> u16 {
        self.height
    }

    fn get_min_height(&self) -> u16 {
        self.min_height
    }

    fn get_max_height(&self) -> Option<u16> {
        self.max_height
    }

    fn get_perfered_height(&self) -> u16 {
        self.perfered_height
    }

    fn set_height(&mut self, height: u16) {
        self.height = height;
    }

    fn set_min_height(&mut self, height: u16) {
        self.min_height = height;
    }

    fn set_max_height(&mut self, height: Option<u16>) {
        self.max_height = height;
    }

    fn set_perfered_height(&mut self, height: u16) {
        self.perfered_height = height;
    }

    fn get_position(&self) -> (u16, u16) {
        self.position
    }

    fn set_position(&mut self, position: (u16, u16)) {
        self.position = position;
    }

    fn as_container(&mut self) -> std::option::Option<&mut dyn Container> {
        Some(self as &mut dyn Container)
    }
}

impl Container for Rectangle {
    fn fit_sizing(&mut self) {
        match self.layout_mode {
            LayoutMode::TopToBottom => todo!(),
            LayoutMode::LeftToRight => {
                let mut axis_size: u16 = 2 * self.padding;
                let mut off_axis_size: u16 = 0;
                for child in &self.children {
                    if let Ok(mut prim) = child.lock() {
                        if let Some(container) = prim.as_container() {
                            container.fit_sizing();
                        } else {
                            let width = prim.get_min_width();
                            prim.set_perfered_width(width);
                            let height = prim.get_min_height();
                            prim.set_perfered_height(height);
                        }

                        axis_size += prim.get_width();
                        off_axis_size = off_axis_size.max(prim.get_height());
                    }
                }
                let len = self.children.len();
                if len > 0 {
                    axis_size += (len as u16 - 1) * self.child_gap;
                }

                match self.sizing.width {
                    SizingMode::Fixed(w) => self.perfered_width = w,
                    SizingMode::Fit | SizingMode::Grow => self.perfered_width = axis_size,
                }

                match self.sizing.height {
                    SizingMode::Fixed(h) => self.perfered_height = h,
                    SizingMode::Fit | SizingMode::Grow => self.perfered_height = off_axis_size,
                }
            }
        }
    }

    fn set_child_positions(&mut self) {
        match self.layout_mode {
            LayoutMode::TopToBottom => todo!(),
            LayoutMode::LeftToRight => {
                let mut child_position = self.position;
                child_position.0 += self.padding;
                child_position.1 += self.padding;

                for child in &self.children {
                    if let Ok(mut prim) = child.lock() {
                        prim.set_position(child_position);
                        child_position.0 += prim.get_width() + self.child_gap;

                        if let Some(container) = prim.as_container() {
                            container.set_child_positions();
                        }
                    }
                }
            }
        }
    }
}
