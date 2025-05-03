#![allow(dead_code)]

use std::sync::{Arc, Mutex, Weak};

use log::{Level, log};
use tinyutils::color::srgb;

use crate::renderer_backend::mesh_builder::{Mesh, make_ss_rectangle};

pub trait Container: Send {
    fn fit_sizing(&mut self);
    fn set_child_positions(&mut self);

    fn draw(&self, render_pass: &mut wgpu::RenderPass, device: &wgpu::Device, size: (u16, u16));
}

pub trait Primative: Send {
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

    #[allow(unused_variables)]
    fn draw_prim(
        &self,
        render_pass: &mut wgpu::RenderPass,
        device: &wgpu::Device,
        size: (u16, u16),
    ) {
    }

    fn get_mesh(&self, size: (u16, u16)) -> Mesh;

    fn as_container(&mut self) -> Option<&mut dyn Container> {
        None
    }
}

pub struct UI {
    pub background_color: srgb,
    pub size: (u16, u16),
    pub root_item: Arc<Mutex<dyn Container>>,
}
impl Default for UI {
    fn default() -> Self {
        Self {
            root_item: Arc::new(Mutex::new(TCContainer {})),
            background_color: Default::default(),
            size: Default::default(),
        }
    }
}

impl UI {
    pub fn compute_layout(&self) {
        if let Ok(mut container) = self.root_item.lock() {
            container.fit_sizing();
            container.set_child_positions();
        }
    }
}

impl Container for UI {
    fn fit_sizing(&mut self) {
        if let Ok(mut container) = self.root_item.lock() {
            container.fit_sizing();
        }
    }

    fn set_child_positions(&mut self) {
        if let Ok(mut root) = self.root_item.lock() {
            root.set_child_positions();
        }
    }

    fn draw(&self, render_pass: &mut wgpu::RenderPass, device: &wgpu::Device, size: (u16, u16)) {
        if let Ok(root) = self.root_item.lock() {
            root.draw(render_pass, device, size);
        }
    }
}

#[derive(Debug, Default)]
pub enum SizingMode {
    Fixed(u16),
    #[default]
    Fit,
    Grow,
}

#[derive(Debug, Default)]
pub struct Sizing {
    pub width: SizingMode,
    pub height: SizingMode,
}

impl Sizing {
    pub const FIT: Sizing = Sizing {
        width: SizingMode::Fit,
        height: SizingMode::Fit,
    };

    pub const GROW: Sizing = Sizing {
        width: SizingMode::Grow,
        height: SizingMode::Grow,
    };
}

#[derive(Debug, Default, Clone)]
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

    fn draw(&self, _render_pass: &mut wgpu::RenderPass, _device: &wgpu::Device, _size: (u16, u16)) {
        log!(
            Level::Error,
            "TCContainer can't be drawn as it is just a temp struct. replace with a proper container"
        )
    }
}

pub struct Rectangle {
    pub width: u16,
    pub height: u16,
    pub perfered_width: u16,
    pub perfered_height: u16,
    pub min_width: u16,
    pub min_height: u16,
    pub max_width: Option<u16>,
    pub max_height: Option<u16>,
    pub position: (u16, u16),
    pub layout_mode: LayoutMode,
    pub sizing: Sizing,
    pub padding: u16,
    pub child_gap: u16,
    pub color: srgb,
    pub parent: Weak<Mutex<dyn Container>>,
    pub children: Vec<Arc<Mutex<dyn Primative>>>,
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

    fn draw_prim(
        &self,
        render_pass: &mut wgpu::RenderPass,
        device: &wgpu::Device,
        size: (u16, u16),
    ) {
        let mut mesh = make_ss_rectangle(
            self.position.0,
            self.position.1,
            self.width,
            self.height,
            self.color,
            size,
        );
        mesh.draw(render_pass, device);
    }

    fn get_mesh(&self, size: (u16, u16)) -> Mesh {
        make_ss_rectangle(
            self.position.0,
            self.position.1,
            self.width,
            self.height,
            self.color,
            size,
        )
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

                off_axis_size += 2 * self.padding;

                match self.sizing.width {
                    SizingMode::Fixed(w) => {
                        self.perfered_width = w;
                        self.width = w;
                    }
                    SizingMode::Fit | SizingMode::Grow => {
                        self.perfered_width = axis_size;
                        self.width = axis_size;
                    }
                }

                match self.sizing.height {
                    SizingMode::Fixed(h) => {
                        self.perfered_height = h;
                        self.height = h;
                    }
                    SizingMode::Fit | SizingMode::Grow => {
                        self.perfered_height = off_axis_size;
                        self.height = off_axis_size;
                    }
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

    fn draw(&self, render_pass: &mut wgpu::RenderPass, device: &wgpu::Device, size: (u16, u16)) {
        let mut mesh = make_ss_rectangle(
            self.position.0,
            self.position.1,
            self.width,
            self.height,
            self.color,
            size,
        );
        mesh.draw(render_pass, device);

        for child in &self.children {
            if let Ok(mut prim) = child.lock() {
                if let Some(container) = prim.as_container() {
                    container.draw(render_pass, device, size);
                } else {
                    prim.draw_prim(render_pass, device, size);
                }
            }
        }
    }
}
