use cgmath::Vector2;
use std::collections::HashMap;

use specs::{Join, World, WorldExt};

use crate::builder::*;

use crate::components::*;
use crate::configs::SCREEN_SIZE;
use crate::renderer::*;
use crate::resources::Camera;
use crate::resources::*;
use crate::system;
use crate::system::UnifiedDispatcher;

pub struct GameState {
    pub world: World,
    dispatcher: Box<dyn UnifiedDispatcher + 'static>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            world: World::new(),
            dispatcher: system::build(),
        }
    }
}

impl GameState {
    pub fn init(&mut self) {
        self.world.register::<Transform>();
        self.world.register::<Tile>();
        self.world.register::<Cell>();

        self.world
            .insert(Camera::new(SCREEN_SIZE[0] as f32 / SCREEN_SIZE[1] as f32));
        // self.world.insert(DeltaTime(0.05));
        self.world.insert(InputHandler::default());
        self.world.insert(StageTick {
            current_spent: 0.0,
            stage_tick: 0.3,
        });

        // self.world.register::<>();
        self.init_game();
    }

    fn init_game(&mut self) {
        // agent(&mut self.world);
        cell_grid(&mut self.world);

        let positions_to_set_alive = vec![
            // y = 0일 때, x = -5부터 6까지
            [-5.0, 0.0],
            [-4.0, 0.0],
            [-3.0, 0.0],
            [-2.0, 0.0],
            [-1.0, 0.0],
            [0.0, 0.0],
            [1.0, 0.0],
            [2.0, 0.0],
            [3.0, 0.0],
            [4.0, 0.0],
            [5.0, 0.0],
            [6.0, 0.0],
            // y = 2일 때, x = -3부터 4까지
            [-3.0, 2.0],
            [-2.0, 2.0],
            [-1.0, 2.0],
            [0.0, 2.0],
            [1.0, 2.0],
            [2.0, 2.0],
            [3.0, 2.0],
            [4.0, 2.0],
            // y = -2일 때, x = -3부터 4까지
            [-3.0, -2.0],
            [-2.0, -2.0],
            [-1.0, -2.0],
            [0.0, -2.0],
            [1.0, -2.0],
            [2.0, -2.0],
            [3.0, -2.0],
            [4.0, -2.0],
            // y = 4일 때, x = -1부터 2까지
            [-1.0, 4.0],
            [0.0, 4.0],
            [1.0, 4.0],
            [2.0, 4.0],
            // y = -4일 때, x = -1부터 2까지
            [-1.0, -4.0],
            [0.0, -4.0],
            [1.0, -4.0],
            [2.0, -4.0],
        ];
        set_cells_alive_at_positions(&mut self.world, positions_to_set_alive);
    }

    fn update_delta_time(&mut self, dt: f32) {
        // let mut delta = self.world.write_resource::<DeltaTime>();
        // *delta = DeltaTime(dt);

        let mut stage = self.world.write_resource::<StageTick>();
        stage.current_spent += dt;
    }

    pub fn update(&mut self, dt: f32) {
        self.update_delta_time(dt);
        self.dispatcher.run_now(&mut self.world);
        self.world.maintain();
    }

    pub fn handle_mouse_input(&mut self, event: winit::event::WindowEvent) -> bool {
        use winit::event::*;
        let mut input_handler = self.world.write_resource::<InputHandler>();
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let mouse_position = <[f32; 2]>::from(position).into();
                input_handler.cursor_moved(mouse_position);
                true
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(x, y),
                ..
            } => {
                input_handler.mouse_wheel(Vector2 { x, y });
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
                let button = match button {
                    MouseButton::Left => MouseButtons::LEFT,
                    MouseButton::Right => MouseButtons::RIGHT,
                    MouseButton::Middle => MouseButtons::MIDDLE,
                    _ => MouseButtons::NONE,
                };
                input_handler.mouse_input(pressed, button);
                true
            }
            _ => false,
        }
    }

    pub fn get_camera_uniform(&self) -> [[f32; 4]; 4] {
        let camera = self.world.read_resource::<Camera>();
        camera.get_view_proj()
    }

    pub fn get_cell_instance(&self) -> HashMap<String, Vec<TileAttributes>> {
        let tiles = self.world.read_storage::<Tile>();
        let transforms = self.world.read_storage::<Transform>();
        let cells = self.world.read_storage::<Cell>();
        let rt_data = (&tiles, &transforms, &cells).join().collect::<Vec<_>>();

        let mut tile_instance_data_hashmap = HashMap::new();
        for (tile, transform, cell) in rt_data {
            if !cell.alive {
                continue;
            }
            let atlas = tile.atlas.clone();
            let instance = TileAttributes {
                uv: tile.uv,
                position: transform.position,
                size: transform.size,
                rotation: cgmath::Rad(transform.rotation),
            };

            tile_instance_data_hashmap
                .entry(atlas)
                .or_insert_with(Vec::new)
                .push(instance);
        }

        tile_instance_data_hashmap
    }

    pub fn get_text_data(&self) -> Vec<TextAttributes> {
        // vec![TextAttributes {
        //     content: "Welcome\nThis is Test message".to_string(),
        //     position: [-4.5, 8.5, 1.],
        //     size: 1.0,
        //     color: [0.0, 0.0, 0.0],
        // }
        vec![]
    }

    #[allow(unused)]
    pub fn force_restart(&mut self) {
        self.world.delete_all();
    }
}
