#[cfg(target_arch = "wasm32")]
use lazy_static::lazy_static;
#[cfg(target_arch = "wasm32")]
use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
pub struct JsBinding {
    buttons: Vec<Mutex<bool>>,
    // btn_reset: Mutex<bool>,
}
#[cfg(target_arch = "wasm32")]
impl JsBinding {
    // 상태를 설정하는 메서드
    fn set_state(&self, btn_index: usize, pressed: bool) {
        let mut state = self.buttons[btn_index].lock().unwrap();
        *state = pressed;
    }

    // 상태를 읽는 메서드
    pub fn get_state(&self, btn_index: usize) -> bool {
        let state = self.buttons[btn_index].lock().unwrap();
        *state
    }

    pub fn reset(&self) {
        for button in &self.buttons {
            let mut state = button.lock().unwrap();
            *state = false;
        }
    }
}

#[cfg(target_arch = "wasm32")]
lazy_static! {
    pub static ref JS_BINDING: Arc<JsBinding> = Arc::new(JsBinding {
        buttons: vec![
            Mutex::new(false),
            Mutex::new(false),
            Mutex::new(false),
            Mutex::new(false),
        ],
    });
}

#[cfg(target_arch = "wasm32")]
pub fn set_btn_start_game(btn_index: usize, new_state: bool) {
    JS_BINDING.set_state(btn_index, new_state);
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn set_state_from_js(btn_type: usize, pressed: bool) {
        set_btn_start_game(btn_type, pressed);
    }
}
