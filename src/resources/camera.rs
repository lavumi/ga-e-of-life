use cgmath::{Point2, Point3, SquareMatrix, Vector3};
// use winit::event::VirtualKeyCode::C;
use crate::configs::SCREEN_SIZE;

#[derive(Debug)]
struct View {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
}

impl View {
    fn move_by(&mut self, delta: [f32; 3]) -> [f32; 3] {
        self.eye.x += delta[0];
        self.eye.y += delta[1];
        self.eye.z += delta[2];

        self.eye.z = self.eye.z.clamp(2.0, 50.0);
        self.target.x += delta[0];
        self.target.y += delta[1];

        self.eye.into()
    }
    fn move_to(&mut self, position: [f32; 3]) -> [f32; 3] {
        self.eye.x = position[0];
        self.eye.y = position[1];
        // self.eye.z = position[2];
        self.target.x = position[0];
        self.target.y = position[1];

        self.eye.into()
    }
}

pub struct Camera {
    view: View,
    proj: cgmath::Matrix4<f32>,
    magic: [f32; 2],
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(1.33333)
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let fov = cgmath::Deg(60.0);
        let near = 0.1;
        let far = 100.0;
        let proj = cgmath::perspective(fov, aspect_ratio, near, far);

        let fov_rad = std::f32::consts::PI * 0.33333;
        //이게 뭐지???
        //과거의 나 대체 무슨 생각이었냐!
        let magic = [
            2.0 * f32::tan(fov_rad * 0.5 * aspect_ratio) / SCREEN_SIZE[0] as f32 * 0.5,
            -2.0 * f32::tan(fov_rad * 0.5) / SCREEN_SIZE[1] as f32 * 0.5,
        ];
        Self {
            view: View {
                eye: (0.0, 0.0, 60.0).into(),
                target: (0.0, 0.0, 0.0).into(),
                up: Vector3::unit_y(),
            },
            proj,
            magic,
        }
    }

    #[allow(unused)]
    pub fn move_to(&mut self, position: [f32; 3]) -> [f32; 3] {
        self.view.move_to(position)
    }

    pub fn move_by(&mut self, delta: [f32; 3]) -> [f32; 3] {
        let world_delta = [
            delta[0] * self.magic[0] * self.view.eye.z,
            delta[1] * self.magic[1] * self.view.eye.z,
            delta[2],
        ];

        self.view.move_by(world_delta)
    }

    #[allow(unused)]
    pub fn new_orthographic(height: u32) -> Self {
        let height = height as f32;
        let width = SCREEN_SIZE[0] as f32 / SCREEN_SIZE[1] as f32 * height;
        let near = 0.1;
        let far = 100.0;
        let proj = cgmath::ortho(-width, width, -height, height, near, far);
        let magic = [1., 1.];
        Self {
            view: View {
                eye: (0.0, 0.0, 30.0).into(),
                target: (0.0, 0.0, 0.0).into(),
                up: Vector3::unit_y(),
            },
            proj,
            magic,
        }
    }

    pub fn get_view_proj(&self) -> [[f32; 4]; 4] {
        let vp = self.build_view_projection_matrix();
        vp.into()
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.view.eye, self.view.target, self.view.up);
        self.proj * view
    }

    #[allow(unused)]
    fn screen_to_world(&self, position: Point2<f32>) -> Point3<f32> {
        let ndc_coords: cgmath::Vector4<f32> = [
            (2.0 * position[0]) / SCREEN_SIZE[0] as f32 - 1.0,
            1.0 - (2.0 * position[1]) / SCREEN_SIZE[1] as f32,
            1.0,
            1.0,
        ]
        .into();
        let inv_proj = self.proj.invert().unwrap();
        let inv_view = cgmath::Matrix4::look_at_rh(self.view.eye, self.view.target, self.view.up)
            .invert()
            .unwrap();
        let world_space_coords = inv_view * inv_proj * ndc_coords;

        let world_coords = world_space_coords.truncate() / world_space_coords.w;
        Point3 {
            x: world_coords.x,
            y: world_coords.y,
            z: world_coords.z,
        }
    }
}
