use std::ops::DerefMut;

use cgmath::Vector3;
use tinycolors::srgb;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: srgb,
}

#[derive(Debug)]
pub struct Mesh {
    pub verticies: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn draw(&mut self, render_pass: &mut wgpu::RenderPass, device: &wgpu::Device) {
        let vertex_buffer = make_verticies(device, self.verticies.deref_mut());
        let index_buffer = make_indecies(device, self.indices.deref_mut());
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
    }
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
    }
}

fn array_to_u8_vec<T: Sized>(p: &mut [T]) -> Vec<u8> {
    let temp: Vec<u8> = p
        .iter_mut()
        .map(|f| any_as_u8_slice(f))
        .flat_map(|s| s.iter())
        .cloned()
        .collect();
    temp
}

pub fn make_verticies<'a, T: Into<&'a mut [Vertex]>>(
    device: &wgpu::Device,
    vertecies: T,
) -> wgpu::Buffer {
    let verticies = array_to_u8_vec(vertecies.into());
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("vertex buffer"),
        contents: &verticies,
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn make_indecies<'a, T: Into<&'a mut [u16]>>(
    device: &wgpu::Device,
    indices: T,
) -> wgpu::Buffer {
    let indices = array_to_u8_vec(indices.into());

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("index buffer"),
        contents: &indices,
        usage: wgpu::BufferUsages::INDEX,
    })
}

pub fn make_rectangle(x: f32, y: f32, w: f32, h: f32, color: srgb) -> Mesh {
    let verticies = vec![
        Vertex {
            position: Vector3 { x, y, z: 0.0 },
            color,
        },
        Vertex {
            position: Vector3 {
                x: x + w,
                y,
                z: 0.0,
            },
            color,
        },
        Vertex {
            position: Vector3 {
                x,
                y: y - h,
                z: 0.0,
            },
            color,
        },
        Vertex {
            position: Vector3 {
                x: x + w,
                y: y - h,
                z: 0.0,
            },
            color,
        },
    ];

    let indices: Vec<u16> = vec![0, 2, 1, 3, 1, 2];

    Mesh { verticies, indices }
}

pub fn make_ss_rectangle(x: i32, y: i32, w: i32, h: i32, color: srgb, size: (i32, i32)) -> Mesh {
    let x = (x as f32 / size.0 as f32) - 1.0;
    let y = 1.0 - (y as f32 / size.1 as f32);
    let w = w as f32 / size.0 as f32;
    let h = h as f32 / size.1 as f32;

    make_rectangle(x, y, w, h, color)
}
