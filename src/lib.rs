use crate::application::Application;
use std::error::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event_loop::{ControlFlow, EventLoop};

mod application;
mod builder;
mod components;
mod configs;
mod game_state;
mod renderer;
mod resources;
mod system;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn wasm_start() {
    start().await.unwrap_throw();
}

pub async fn start() -> Result<(), Box<dyn Error>> {
    // let title = "wgpu_wasm";
    // let width = configs::SCREEN_SIZE[0];
    // let height = configs::SCREEN_SIZE[1];

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

            // #[wasm_bindgen(module = "/defined-in-js.js")]
            // extern "C" {
            //     fn render(gene : &str,pos : &str);
            // }
            //
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = console)]
                fn log(s: &str);
            }
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    // let (wb, event_loop) = WinitState::create(title, width, height);
    // let asset_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/").to_string();
    let mut application = Application::default();
    event_loop
        .run_app(&mut application)
        .expect("TODO: panic message");

    Ok(())
}
