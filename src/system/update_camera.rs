use specs::{Read, System, Write};

use crate::resources::{Camera, InputHandler, MouseButtons};

pub struct UpdateCamera;

impl<'a> System<'a> for UpdateCamera {
    type SystemData = (Write<'a, Camera>, Read<'a, InputHandler>);

    fn run(&mut self, (mut camera, input_handler): Self::SystemData) {
        let mut move_delta = [0., 0., 0.];
        let delta = input_handler.mouse_wheel_delta;
        move_delta[2] = delta[1];

        if input_handler.is_button_pressed(MouseButtons::MIDDLE) {
            move_delta[0] = input_handler.mouse_delta[0];
            move_delta[1] = input_handler.mouse_delta[1];
        }
        camera.move_by(move_delta);
    }
}
