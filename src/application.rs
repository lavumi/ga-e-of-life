use crate::game_state::GameState;
use crate::renderer::*;
use instant::Instant;
use wgpu::SurfaceError;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowId;

pub struct Application {
    game_state: GameState,
    render_context: RenderContextType,
    screen_size: PhysicalSize<u32>,
    prev_time: Instant,
}

impl ApplicationHandler<RenderContext> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let RenderContextType::Builder(builder) = &mut self.render_context {
            builder.build_and_send(event_loop);
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, graphics: RenderContext) {
        self.render_context = RenderContextType::Graphics(graphics);
        //이게 맞아?
        //여기서 세팅되고 redraw_request 가 호출이 안되서 생기는 문제인데
        //더군다나 resize 도 호출이 이상하게 되는듯
        //resize 2번 후에 user_event 가 호출된다
        self.render().unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => {
                        self.game_state.start_auto_playing(0.1);
                    }
                    PhysicalKey::Code(KeyCode::KeyQ) => {
                        self.game_state.restart();
                    }
                    _ => {}
                }
            }

            WindowEvent::CursorMoved { .. }
            | WindowEvent::MouseWheel { .. }
            | WindowEvent::MouseInput { .. } => {
                self.game_state.handle_mouse_input(event);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                log::info!("{:?}", event);
                self.resize(new_size);
            }
            WindowEvent::RedrawRequested => {
                self.update();
                match self.render() {
                    Ok(_) => {}

                    Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                        //todo Reconfigure the surface if it's lost or outdated
                    }
                    Err(SurfaceError::OutOfMemory) => {}
                    Err(SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
                // }
            }
            _ => (),
        }
    }
}

impl Application {
    pub(crate) fn new(event_loop: &EventLoop<RenderContext>) -> Self {
        let mut gs = GameState::default();
        gs.init();

        Self {
            game_state: gs,
            render_context: RenderContextType::Builder(RenderContextBuilder::new(
                event_loop.create_proxy(),
            )),
            prev_time: Instant::now(),
            screen_size: PhysicalSize::default(),
        }
    }

    fn update_delta_time(&mut self) -> f32 {
        let elapsed_time = self.prev_time.elapsed().as_millis() as f32 / 1000.0;
        self.prev_time = Instant::now();
        elapsed_time
    }

    fn update(&mut self) {
        let dt = self.update_delta_time();
        self.game_state.update(dt);

        #[cfg(target_arch = "wasm32")]
        self.check_wasm_input();
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let RenderContextType::Graphics(render_context) = &mut self.render_context else {
            return Err(SurfaceError::Lost);
        };

        //1. update camera
        let camera_uniform = self.game_state.get_camera_uniform();
        render_context.update_camera_buffer(camera_uniform);

        // //2. update meshes
        let instances = self.game_state.get_cell_instance();
        render_context.update_mesh_instance(instances);

        let instances = self.game_state.get_text_data();
        render_context.update_text_instance(instances);
        render_context.render()
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        let RenderContextType::Graphics(_) = &mut self.render_context else {
            log::info!("resize called but graphics not initialized");
            self.screen_size = size;
            return;
        };
        //todo 이거 뭔가 버그 있는거 같음
        //wasm 에서 계속 크기가 2배로 커지네
        // render_context.resize(size);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn check_wasm_input(&mut self) {
        use crate::js_binding::JS_BINDING;
        let start_btn = JS_BINDING.get_state(0);
        if start_btn {
            self.start_auto_playing(0.1);
        }

        let reset_btn = JS_BINDING.get_state(1);
        if reset_btn {
            self.reset_game();
        }
        JS_BINDING.reset();
    }

    #[cfg(target_arch = "wasm32")]
    pub fn start_auto_playing(&mut self, tick: f32) {
        self.game_state.start_auto_playing(tick);
    }
    #[cfg(target_arch = "wasm32")]
    pub fn reset_game(&mut self) {
        self.game_state.restart();
    }
}
