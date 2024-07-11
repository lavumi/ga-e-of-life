use crate::configs;
use crate::game_state::GameState;
use crate::renderer::*;
use instant::Instant;
use std::sync::Arc;
use wgpu::SurfaceError;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use winit::{event::*, window::Window};

pub struct Application<'window> {
    gs: GameState,
    rs: Option<RenderState<'window>>,

    window: Option<Arc<Window>>,

    prev_time: Instant,
}

impl Default for Application<'_> {
    fn default() -> Self {
        let mut gs = GameState::default();
        gs.init();
        Application {
            gs,
            rs: None,
            window: None,
            prev_time: Instant::now(),
        }
    }
}

impl<'window> ApplicationHandler for Application<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let width = configs::SCREEN_SIZE[0];
            let height = configs::SCREEN_SIZE[1];
            let win_attr = Window::default_attributes()
                .with_title("lavumi engine")
                .with_inner_size(LogicalSize::new(width, height));
            // use Arc.
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            #[cfg(target_arch = "wasm32")]
            {
                // Winit prevents sizing with CSS, so we have to set
                // the size manually when on web.
                use winit::dpi::PhysicalSize;
                window.set_min_inner_size(Some(PhysicalSize::new(
                    configs::SCREEN_SIZE[0],
                    configs::SCREEN_SIZE[1],
                )));

                use winit::platform::web::WindowExtWebSys;
                web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| {
                        let dst = doc.get_element_by_id("wgpu-wasm")?;
                        let canvas = web_sys::Element::from(window.canvas()?);
                        canvas.set_id("wasm-canvas");
                        dst.append_child(&canvas).ok()?;
                        Some(())
                    })
                    .expect("Couldn't append canvas to document body.");
            }

            self.window = Some(window.clone());
            self.prev_time = Instant::now();
            let rs = RenderState::new(window);
            self.rs = Some(rs);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CursorMoved { .. }
            | WindowEvent::MouseWheel { .. }
            | WindowEvent::MouseInput { .. } => {
                self.gs.handle_mouse_input(event);
            }

            WindowEvent::CloseRequested => {
                // macOS err: https://github.com/rust-windowing/winit/issues/3668
                // This will be fixed as winit 0.30.1.
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let (Some(rs), Some(window)) = (self.rs.as_mut(), self.window.as_ref()) {
                    rs.resize(new_size);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                // log::info!("update called");
                self.update();
                match self.render() {
                    Ok(_) => {}

                    Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                        //todo Reconfigure the surface if it's lost or outdated
                    }
                    Err(SurfaceError::OutOfMemory) => {}
                    Err(SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }

                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
                // }
            }
            _ => (),
        }
    }
}

impl Application<'_> {
    fn update_delta_time(&mut self) -> f32 {
        let elapsed_time = self.prev_time.elapsed().as_millis() as f32 / 1000.0;
        self.prev_time = Instant::now();
        elapsed_time
    }

    fn update(&mut self) {
        let dt = self.update_delta_time();
        self.gs.update(dt);
    }
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let Some(rs) = self.rs.as_mut() {
            //1. update camera
            let camera_uniform = self.gs.get_camera_uniform();
            rs.update_camera_buffer(camera_uniform);

            // //2. update meshes
            let instances = self.gs.get_cell_instance();
            rs.update_mesh_instance(instances);

            let instances = self.gs.get_text_data();
            rs.update_text_instance(instances);
            rs.render()
        } else {
            Err(SurfaceError::Lost)
        }
    }
    // #[allow(unused_variables)]
    // fn input(&mut self, event: &WindowEvent) -> bool {
    //     match event {
    //         WindowEvent::KeyboardInput { input, .. } => self.gs.handle_keyboard_input(input),
    //         WindowEvent::MouseInput { .. } => self.gs.handle_mouse_input(event),
    //         WindowEvent::CursorMoved { .. } => self.gs.handle_mouse_input(event),
    //         WindowEvent::MouseWheel { .. } => self.gs.handle_mouse_input(event),
    //
    //         _ => false,
    //     }
    // }

    #[allow(unused)]
    pub fn get_data_from_wasm(&self) -> (i32, i32) {
        (0, 0)
    }
}
