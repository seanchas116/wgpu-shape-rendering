use crate::util::cast_to_u8_slice;
use cgmath::{Point2, Point3};
use lyon::lyon_tessellation::VertexBuffers;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: cgmath::Point2<f32>,
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Mesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_to_u8_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: cast_to_u8_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Mesh {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        }
    }

    pub fn from_tessellation(device: &wgpu::Device, geometry: &VertexBuffers<Vertex, u16>) -> Mesh {
        Mesh::new(device, &geometry.vertices, &geometry.indices)
    }

    pub fn pentagon(device: &wgpu::Device) -> Mesh {
        const VERTICES: &[Vertex] = &[
            Vertex {
                position: Point2::new(-0.0868241, 0.49240386),
            }, // A
            Vertex {
                position: Point2::new(-0.49513406, 0.06958647),
            }, // B
            Vertex {
                position: Point2::new(-0.21918549, -0.44939706),
            }, // C
            Vertex {
                position: Point2::new(0.35966998, -0.3473291),
            }, // D
            Vertex {
                position: Point2::new(0.44147372, 0.2347359),
            }, // E
        ];

        const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

        Self::new(&device, VERTICES, INDICES)
    }

    pub fn triangle(device: &wgpu::Device) -> Mesh {
        const VERTICES: &[Vertex] = &[
            Vertex {
                position: Point2::new(-0.5, -0.5),
            },
            Vertex {
                position: Point2::new(0.5, -0.5),
            },
            Vertex {
                position: Point2::new(0.0, 0.5),
            },
        ];

        const INDICES: &[u16] = &[0, 1, 2];

        Self::new(&device, VERTICES, INDICES)
    }

    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }
}
