use glm::Vec3;
use wgpu::util::DeviceExt;

#[repr(C)]
pub struct Vertex {
    position: Vec3,
    color: Vec3,
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

pub fn make_verticies(device: &wgpu::Device) -> wgpu::Buffer {
    let verticies = [
        Vertex {
            position: Vec3::new(-0.75, -0.75, 0.0),
            color: Vec3::new(0.0, 0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.75, -0.75, 0.0),
            color: Vec3::new(1.0, 0.0, 0.5),
        },
        Vertex {
            position: Vec3::new(-0.75, 0.75, 0.0),
            color: Vec3::new(0.0, 1.0, 0.5),
        },
        Vertex {
            position: Vec3::new(0.75, 0.75, 0.0),
            color: Vec3::new(1.0, 1.0, 0.0),
        },
    ];

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("vertex buffer"),
        contents: any_as_u8_slice(&verticies),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn make_indecies(device: &wgpu::Device) -> wgpu::Buffer {
    let indices: [u16; 6] = [0, 1, 2, 2, 1, 3];

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("index buffer"),
        contents: any_as_u8_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    })
}

pub const NUM_INDICES: u32 = 6;
