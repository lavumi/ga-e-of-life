use crate::renderer::mesh::Mesh;
use crate::renderer::vertex::Vertex;
use wgpu::util::DeviceExt;
use wgpu::Device;

fn create_buffers<T: bytemuck::Pod>(
    device: &Device,
    vertex_data: &[T],
    index_data: &[u16],
) -> (wgpu::Buffer, wgpu::Buffer) {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertex_data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(index_data),
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    });

    (vertex_buffer, index_buffer)
}

pub(crate) fn make_tile_mesh(device: &Device, atlas_name: String) -> Mesh {
    //region [ Vertex Data ]
    let tile_size = [1.0, 1.0];
    let tile_size_half = [tile_size[0] * 0.5, tile_size[1] * 0.5];
    let vertex: [Vertex; 4] = [
        //Front
        Vertex {
            position: [-tile_size_half[0], -tile_size_half[1], 0.0],
            tex_coords: [1.0, 0.0],
            // tex_coords: [offset[0] , offset[1] + uv_size[1]],
        },
        Vertex {
            position: [tile_size_half[0], -tile_size_half[1], 0.0],
            tex_coords: [0.0, 0.],
            // tex_coords: [offset[0] +uv_size[0], offset[1] +uv_size[1]],
        },
        Vertex {
            position: [tile_size_half[0], tile_size_half[1], 0.0],
            tex_coords: [0.0, 1.0],
            // tex_coords: [offset[0] +uv_size[0], offset[1] +0.0],
        },
        Vertex {
            position: [-tile_size_half[0], tile_size_half[1], 0.0],
            tex_coords: [1.0, 1.0],
            // tex_coords: offset ,
        },
    ];
    let indices: [u16; 6] = [
        //front
        0, 1, 2, 2, 3, 0,
    ];

    //endregion

    let (vertex_buffer, index_buffer) = create_buffers(device, &vertex, &indices);

    let num_indices = indices.len() as u32;
    let num_instances = 0; //instance_data.len() as u32;

    Mesh {
        atlas_name,
        vertex_buffer,
        index_buffer,
        instance_buffer: None,
        num_indices,
        num_instances,
    }
}

#[allow(unused)]
pub fn make_cube_mesh(device: &Device, atlas_name: String) -> Mesh {
    //region [ Vertex Data ]
    let size = [1.0, 1.0, 1.0];
    let size_half = [size[0] * 0.5, size[1] * 0.5, size[2] * 0.5];
    let vertex: [Vertex; 8] = [
        // Front
        Vertex {
            position: [-size_half[0], -size_half[1], size_half[2]],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [size_half[0], -size_half[1], size_half[2]],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [size_half[0], size_half[1], size_half[2]],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-size_half[0], size_half[1], size_half[2]],
            tex_coords: [0.0, 1.0],
        },
        // Back
        Vertex {
            position: [-size_half[0], -size_half[1], -size_half[2]],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [size_half[0], -size_half[1], -size_half[2]],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [size_half[0], size_half[1], -size_half[2]],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-size_half[0], size_half[1], -size_half[2]],
            tex_coords: [1.0, 1.0],
        },
    ];
    let indices: [u16; 36] = [
        // Front face
        0, 1, 2, 2, 3, 0, // Back face
        4, 5, 6, 6, 7, 4, // Top face
        3, 2, 6, 6, 7, 3, // Bottom face
        0, 1, 5, 5, 4, 0, // Right face
        1, 2, 6, 6, 5, 1, // Left face
        0, 3, 7, 7, 4, 0,
    ];

    //endregion

    let (vertex_buffer, index_buffer) = create_buffers(device, &vertex, &indices);

    let num_indices = indices.len() as u32;
    let num_instances = 0; //instance_data.len() as u32;

    Mesh {
        atlas_name,
        vertex_buffer,
        index_buffer,
        instance_buffer: None,
        num_indices,
        num_instances,
    }
}
