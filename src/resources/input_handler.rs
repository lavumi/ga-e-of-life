use cgmath::{EuclideanSpace, Point2, Vector2, Zero};
// use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};

bitflags::bitflags! {
    pub struct MouseButtons: u8 {
        const NONE = 0b00000000;
        const LEFT = 0b00000001;
        const RIGHT = 0b00000010;
        const MIDDLE = 0b00000100;
    }
}

pub struct InputHandler {
    mouse_position_prev: Point2<f32>,
    pub mouse_delta: Vector2<f32>,
    pub mouse_wheel_delta: Vector2<f32>,
    pub mouse_pressed: MouseButtons,
}

impl Default for InputHandler {
    fn default() -> Self {
        InputHandler {
            mouse_position_prev: Point2::origin(),
            mouse_delta: Vector2::zero(),
            mouse_wheel_delta: Vector2::zero(),
            mouse_pressed: MouseButtons::empty(),
        }
    }
}

impl InputHandler {
    pub fn is_button_pressed(&self, button: MouseButtons) -> bool {
        self.mouse_pressed.contains(button)
    }

    pub fn reset_delta(&mut self) {
        self.mouse_wheel_delta = Vector2::zero();
        self.mouse_delta = Vector2::zero();
    }

    pub fn cursor_moved(&mut self, position: Point2<f32>) {
        self.mouse_delta = self.mouse_position_prev - position;
        self.mouse_position_prev = position;
    }

    pub fn mouse_wheel(&mut self, delta: Vector2<f32>) {
        self.mouse_wheel_delta = delta;
    }

    pub fn mouse_input(&mut self, pressed: bool, button: MouseButtons) {
        if pressed {
            self.mouse_pressed.insert(button);
        } else {
            self.mouse_pressed.remove(button);
        }
    }
}
