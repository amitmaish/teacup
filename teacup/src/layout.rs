#![allow(dead_code)]

use std::{
    ops::{DerefMut, Not},
    sync::{Arc, Mutex},
};

use cgmath::Zero;
use log::{Level, log};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tinyutils::color::srgb;

use crate::renderer_backend::mesh_builder::{Mesh, make_ss_rectangle};

pub trait Container: Send {
    fn fit_sizing(&mut self);
    fn grow_sizing(&mut self);
    fn set_child_positions(&mut self);

    fn draw(&self, render_pass: &mut wgpu::RenderPass, device: &wgpu::Device, size: (i32, i32));

    fn get_sizing(&self) -> &Sizing;
    fn get_sizing_along_axis(&self, axis: Axis) -> &SizingMode;
    fn as_primative(&mut self) -> Option<&mut dyn Primative> {
        None
    }
}

pub trait Primative: Send {
    fn get_width(&self) -> i32;
    fn get_min_width(&self) -> i32;
    fn get_max_width(&self) -> Option<i32>;

    fn set_width(&mut self, width: i32);
    fn set_min_width(&mut self, width: i32);
    fn set_max_width(&mut self, width: Option<i32>);

    fn get_height(&self) -> i32;
    fn get_min_height(&self) -> i32;
    fn get_max_height(&self) -> Option<i32>;

    fn set_height(&mut self, height: i32);
    fn set_min_height(&mut self, height: i32);
    fn set_max_height(&mut self, height: Option<i32>);

    fn get_size_along_axis(&self, axis: Axis) -> i32;
    fn set_size_along_axis(&mut self, axis: Axis, size: i32);
    fn get_min_along_axis(&self, axis: Axis) -> i32;
    fn get_max_along_axis(&self, axis: Axis) -> Option<i32>;

    fn get_position(&self) -> (i32, i32);
    fn set_position(&mut self, position: (i32, i32));

    #[allow(unused_variables)]
    fn draw_prim(
        &self,
        render_pass: &mut wgpu::RenderPass,
        device: &wgpu::Device,
        size: (i32, i32),
    ) {
    }

    fn get_mesh(&self, size: (i32, i32)) -> Mesh;

    fn as_container(&mut self) -> Option<&mut dyn Container> {
        None
    }
}

#[derive(Debug, Default)]
pub enum SizingMode {
    Fixed(i32),
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

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}
impl Not for Axis {
    type Output = Axis;

    fn not(self) -> Self::Output {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

struct TCContainer {}

impl Container for TCContainer {
    fn fit_sizing(&mut self) {
        log!(
            Level::Error,
            "TCContainer can't compute layout as it is just a temp struct. replace with a proper container"
        )
    }

    fn grow_sizing(&mut self) {
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

    fn draw(&self, _render_pass: &mut wgpu::RenderPass, _device: &wgpu::Device, _size: (i32, i32)) {
        log!(
            Level::Error,
            "TCContainer can't be drawn as it is just a temp struct. replace with a proper container"
        )
    }

    fn get_sizing(&self) -> &Sizing {
        log!(
            Level::Error,
            "TCContainer has no sizing as it is just a temp struct. replace with a proper container"
        );
        &Sizing::FIT
    }

    fn get_sizing_along_axis(&self, _axis: Axis) -> &SizingMode {
        log!(
            Level::Error,
            "TCContainer has no sizing as it is just a temp struct. replace with a proper container"
        );
        &Sizing::FIT.width
    }
}

pub struct UI {
    pub background_color: srgb,
    pub size: (i32, i32),
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
    pub fn compute_layout(&mut self) {
        if let Ok(mut container) = self.root_item.lock() {
            container.fit_sizing();
            self.grow_root(container.deref_mut());
            container.grow_sizing();
            container.set_child_positions();
        }
    }

    fn grow_root(&self, root: &mut dyn Container) {
        if let SizingMode::Grow = root.get_sizing().width {
            if let Some(prim) = root.as_primative() {
                prim.set_width(self.size.0);
            }
        }
        if let SizingMode::Grow = root.get_sizing().height {
            if let Some(prim) = root.as_primative() {
                prim.set_height(self.size.1);
            }
        }
    }
}

impl Container for UI {
    fn fit_sizing(&mut self) {
        if let Ok(mut container) = self.root_item.lock() {
            container.fit_sizing();
        }
    }

    fn grow_sizing(&mut self) {
        log!(
            Level::Warn,
            "grow sizizng shouldn't be called on the main ui"
        );
    }

    fn set_child_positions(&mut self) {
        if let Ok(mut root) = self.root_item.lock() {
            root.set_child_positions();
        }
    }

    fn draw(&self, render_pass: &mut wgpu::RenderPass, device: &wgpu::Device, size: (i32, i32)) {
        if let Ok(root) = self.root_item.lock() {
            root.draw(render_pass, device, size);
        }
    }

    fn get_sizing(&self) -> &Sizing {
        &Sizing::GROW
    }

    fn get_sizing_along_axis(&self, _axis: Axis) -> &SizingMode {
        &Sizing::GROW.width
    }
}

#[derive(Default)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
    pub min_width: i32,
    pub min_height: i32,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
    pub position: (i32, i32),
    pub layout_mode: LayoutMode,
    pub sizing: Sizing,
    pub padding: i32,
    pub child_gap: i32,
    pub color: srgb,
    pub children: Vec<Arc<Mutex<dyn Primative>>>,
}

impl Primative for Rectangle {
    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_min_width(&self) -> i32 {
        self.min_width
    }

    fn get_max_width(&self) -> Option<i32> {
        self.max_width
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    fn set_min_width(&mut self, width: i32) {
        self.min_width = width;
    }

    fn set_max_width(&mut self, width: Option<i32>) {
        self.max_width = width;
    }

    fn get_height(&self) -> i32 {
        self.height
    }

    fn get_min_height(&self) -> i32 {
        self.min_height
    }

    fn get_max_height(&self) -> Option<i32> {
        self.max_height
    }

    fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    fn set_min_height(&mut self, height: i32) {
        self.min_height = height;
    }

    fn set_max_height(&mut self, height: Option<i32>) {
        self.max_height = height;
    }

    fn get_size_along_axis(&self, axis: Axis) -> i32 {
        match axis {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
        }
    }

    fn set_size_along_axis(&mut self, axis: Axis, size: i32) {
        match axis {
            Axis::Horizontal => self.width = size,
            Axis::Vertical => self.height = size,
        }
    }

    fn get_min_along_axis(&self, axis: Axis) -> i32 {
        match axis {
            Axis::Horizontal => self.min_width,
            Axis::Vertical => self.min_height,
        }
    }

    fn get_max_along_axis(&self, axis: Axis) -> Option<i32> {
        match axis {
            Axis::Horizontal => self.max_width,
            Axis::Vertical => self.max_height,
        }
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.position = position;
    }

    fn as_container(&mut self) -> std::option::Option<&mut dyn Container> {
        Some(self as &mut dyn Container)
    }

    fn draw_prim(
        &self,
        render_pass: &mut wgpu::RenderPass,
        device: &wgpu::Device,
        size: (i32, i32),
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

    fn get_mesh(&self, size: (i32, i32)) -> Mesh {
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
        let axis = match self.layout_mode {
            LayoutMode::TopToBottom => Axis::Vertical,
            LayoutMode::LeftToRight => Axis::Horizontal,
        };
        let mut axis_size: i32 = 2 * self.padding;
        let mut off_axis_size: i32 = 0;
        let mut first = false;
        let mut gap = 0;
        for child in &self.children {
            if let Ok(mut prim) = child.lock() {
                if let Some(container) = prim.as_container() {
                    container.fit_sizing();
                } else {
                    let size = prim.get_min_along_axis(axis);
                    prim.set_size_along_axis(axis, size);
                    let size = prim.get_min_along_axis(!axis);
                    prim.set_size_along_axis(!axis, size);
                }

                axis_size += prim.get_size_along_axis(axis) + gap;
                off_axis_size = off_axis_size.max(prim.get_size_along_axis(!axis));

                if !first {
                    first = true;
                    gap = self.child_gap;
                }
            }
        }

        off_axis_size += 2 * self.padding;
        match self.layout_mode {
            LayoutMode::TopToBottom => {
                match self.sizing.width {
                    SizingMode::Fixed(w) => {
                        self.width = w;
                    }
                    SizingMode::Fit | SizingMode::Grow => {
                        self.width = off_axis_size.max(self.min_width);
                        if let Some(max) = self.max_width {
                            self.width = self.width.min(max);
                        }
                    }
                }

                match self.sizing.height {
                    SizingMode::Fixed(h) => {
                        self.height = h;
                    }
                    SizingMode::Fit | SizingMode::Grow => {
                        self.height = axis_size.max(self.min_height);
                        if let Some(max) = self.max_height {
                            self.height = self.height.min(max);
                        }
                    }
                }
            }
            LayoutMode::LeftToRight => {
                match self.sizing.width {
                    SizingMode::Fixed(w) => {
                        self.width = w;
                    }
                    SizingMode::Fit | SizingMode::Grow => {
                        self.width = axis_size.max(self.min_width);
                        if let Some(max) = self.max_width {
                            self.width = self.width.min(max);
                        }
                    }
                }

                match self.sizing.height {
                    SizingMode::Fixed(h) => {
                        self.height = h;
                    }
                    SizingMode::Fit | SizingMode::Grow => {
                        self.height = off_axis_size.max(self.min_height);
                        if let Some(max) = self.max_height {
                            self.height = self.height.min(max);
                        }
                    }
                }
            }
        }
    }

    fn grow_sizing(&mut self) {
        let axis = match self.layout_mode {
            LayoutMode::TopToBottom => Axis::Vertical,
            LayoutMode::LeftToRight => Axis::Horizontal,
        };

        let used_space: i32 = self
            .children
            .par_iter()
            .map(|prim| {
                if let Ok(prim) = prim.lock() {
                    prim.get_size_along_axis(axis)
                } else {
                    0
                }
            })
            .sum();
        let mut remaining_space = self.get_size_along_axis(axis)
            - (self.padding * 2)
            - (self.child_gap * ((self.children.len() as i32) - 1))
            - used_space;

        let mut grow_list: Vec<Arc<Mutex<dyn Primative>>> = self
            .children
            .par_iter()
            .filter(|prim| {
                if let Ok(mut prim) = prim.lock() {
                    if let Some(container) = prim.as_container() {
                        matches!(container.get_sizing_along_axis(axis), SizingMode::Grow)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        let mut depth = grow_list.len() + 1;

        while remaining_space.is_positive() && !grow_list.is_empty() && !depth.is_zero() {
            depth -= 1;

            let smallest_size = grow_list
                .par_iter()
                .map(|prim| {
                    if let Ok(prim) = prim.lock() {
                        prim.get_size_along_axis(axis)
                    } else {
                        i32::MAX
                    }
                })
                .min()
                .unwrap_or(0);

            let min_growing_list: Vec<Arc<Mutex<dyn Primative>>> = grow_list
                .par_iter()
                .filter(|prim| {
                    if let Ok(prim) = prim.lock() {
                        prim.get_size_along_axis(axis) <= smallest_size
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();

            let filter: Vec<Arc<Mutex<dyn Primative>>> = grow_list
                .par_iter()
                .filter(|prim| {
                    if let Ok(prim) = prim.lock() {
                        prim.get_size_along_axis(axis) > smallest_size
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();

            let mut second_smallest_size: Option<i32> = None;

            for child in filter {
                let size = if let Ok(prim) = child.lock() {
                    prim.get_size_along_axis(axis)
                } else {
                    remaining_space
                };

                if let Some(min) = second_smallest_size {
                    second_smallest_size = Some(size.min(min));
                } else {
                    second_smallest_size = Some(size);
                }
            }

            // let second_smallest_size = filter
            //     .iter()
            //     .map(|prim| {
            //         if let Ok(prim) = prim.lock() {
            //             prim.get_size_along_axis(axis)
            //         } else {
            //             remaining_space
            //         }
            //     })
            //     .min();

            let grow_step = if let Some(second_smallest_size) = second_smallest_size {
                (second_smallest_size - smallest_size).min(remaining_space / min_growing_list.len() as i32)
            } else {
                remaining_space / min_growing_list.len() as i32
            };

            for (i, prim) in min_growing_list.iter().enumerate() {
                if let Ok(mut prim) = prim.lock() {
                    let prim_size = prim.get_size_along_axis(axis);
                    let prim_min_size = prim.get_min_along_axis(axis);
                    let prim_max_size = prim.get_max_along_axis(axis);
                    let prim_size = (prim_size + grow_step).max(prim_min_size);
                    prim.set_size_along_axis(axis, prim_size);
                    if let Some(max) = prim_max_size {
                        if prim_size >= max {
                            prim.set_size_along_axis(axis, max);
                            grow_list.remove(i);
                        }
                    }
                }
            }
            let used_space: i32 = self
                .children
                .par_iter()
                .map(|prim| {
                    if let Ok(prim) = prim.lock() {
                        prim.get_size_along_axis(axis)
                    } else {
                        0
                    }
                })
                .sum();
            remaining_space = self.get_size_along_axis(axis)
                - (self.padding * 2)
                - (self.child_gap * ((self.children.len() as i32) - 1).max(0))
                - used_space;
        }

        let grow_list: Vec<Arc<Mutex<dyn Primative>>> = self
            .children
            .par_iter()
            .filter(|prim| {
                if let Ok(mut prim) = prim.lock() {
                    if let Some(container) = prim.as_container() {
                        matches!(container.get_sizing_along_axis(!axis), SizingMode::Grow)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        let off_axis_size = self.get_size_along_axis(!axis) - (2 * self.padding);

        for child in grow_list {
            if let Ok(mut prim) = child.lock() {
                prim.set_size_along_axis(!axis, off_axis_size);
            }
        }

        for child in &self.children {
            if let Ok(mut prim) = child.lock() {
                if let Some(container) = prim.as_container() {
                    container.grow_sizing();
                }
            }
        }
    }

    fn set_child_positions(&mut self) {
        match self.layout_mode {
            LayoutMode::TopToBottom => {
                let mut child_position = self.position;
                child_position.0 += self.padding;
                child_position.1 += self.padding;

                for child in &self.children {
                    if let Ok(mut prim) = child.lock() {
                        prim.set_position(child_position);
                        child_position.1 += prim.get_height() + self.child_gap;

                        if let Some(container) = prim.as_container() {
                            container.set_child_positions();
                        }
                    }
                }
            }
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

    fn draw(&self, render_pass: &mut wgpu::RenderPass, device: &wgpu::Device, size: (i32, i32)) {
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

    fn get_sizing(&self) -> &Sizing {
        &self.sizing
    }

    fn get_sizing_along_axis(&self, axis: Axis) -> &SizingMode {
        match axis {
            Axis::Horizontal => &self.sizing.width,
            Axis::Vertical => &self.sizing.height,
        }
    }

    fn as_primative(&mut self) -> Option<&mut dyn Primative> {
        Some(self as &mut dyn Primative)
    }
}
