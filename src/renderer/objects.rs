use crate::renderer::mesh::InstanceSimpleTileRaw;

pub struct TextAttributes {
    pub content: String,
    pub color: [f32; 3],
    pub position: [f32; 3],
    pub size: f32,
}

pub struct TileAttributes {
    pub uv: [f32; 4],
    pub position: [f32; 3],
    pub rotation: cgmath::Rad<f32>,
    pub size: [f32; 2],
}

impl TileAttributes {
    pub fn get_instance_matrix(&self) -> InstanceSimpleTileRaw {
        let position = cgmath::Vector3 {
            x: self.position[0],
            y: self.position[1],
            z: self.position[2],
        };
        let translation_matrix = cgmath::Matrix4::from_translation(position);
        let rotation_matrix = cgmath::Matrix4::from_angle_z(self.rotation);
        let scale_matrix = cgmath::Matrix4::from_nonuniform_scale(self.size[0], self.size[1], 1.0);
        let model = (translation_matrix * rotation_matrix * scale_matrix).into();

        InstanceSimpleTileRaw { uv: self.uv, model }
    }
}
