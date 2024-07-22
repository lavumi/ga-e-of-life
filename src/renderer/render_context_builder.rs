use crate::configs;
use crate::renderer::RenderContext;
use std::future::Future;
use std::sync::Arc;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::Window;

pub struct RenderContextBuilder {
    event_loop_proxy: Option<EventLoopProxy<RenderContext>>,
}

impl RenderContextBuilder {
    pub fn new(event_loop_proxy: EventLoopProxy<RenderContext>) -> Self {
        Self {
            event_loop_proxy: Some(event_loop_proxy),
        }
    }
    pub fn build_and_send(&mut self, event_loop: &ActiveEventLoop) {
        let Some(event_loop_proxy) = self.event_loop_proxy.take() else {
            // event_loop_proxy is already spent - we already constructed Graphics
            return;
        };

        #[cfg(target_arch = "wasm32")]
        {
            let gfx_fut = create_render_context(event_loop);
            wasm_bindgen_futures::spawn_local(async move {
                let gfx = gfx_fut.await;
                assert!(event_loop_proxy.send_event(gfx).is_ok());
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let gfx = pollster::block_on(create_render_context(event_loop));
            assert!(event_loop_proxy.send_event(gfx).is_ok());
        }
    }
}

pub fn create_render_context(
    event_loop: &ActiveEventLoop,
) -> impl Future<Output = RenderContext> + 'static {
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
    let size = PhysicalSize::new(configs::SCREEN_SIZE[0], configs::SCREEN_SIZE[1]);

    // The instance is a handle to our GPU
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::default();
    let surface = match instance.create_surface(Arc::clone(&window)) {
        Ok(surface) => surface,
        Err(e) => {
            panic!("Failed to create surface: {:?}", e);
        }
    };

    async move {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu`s features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            // .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 0,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        let depth_texture = crate::renderer::texture::TextureViewAndSampler::create_depth_texture(
            &device,
            &config,
            "depth_texture",
        );
        let color = wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };

        let aspect_ratio = size.width as f32 / size.height as f32;
        let viewport_data = [0., 0., size.width as f32, size.height as f32, 0., 1.];

        let mut gpu_resource_manager =
            crate::renderer::gpu_resource_manager::GPUResourceManager::default();
        gpu_resource_manager.initialize(&device);
        let mut pipeline_manager = crate::renderer::pipeline_manager::PipelineManager::default();
        pipeline_manager.init_pipelines(&device, config.format, &gpu_resource_manager);

        let font_manager = crate::renderer::font_manager::FontManager::default();

        let mut rs = RenderContext {
            window,
            device,
            surface,
            queue,
            config,
            gpu_resource_manager,
            pipeline_manager,
            color,
            depth_texture,
            aspect_ratio,
            viewport_data,
            font_manager,
        };

        rs.init_resources().await;
        rs
    }
}

pub enum RenderContextType {
    Builder(RenderContextBuilder),
    Graphics(RenderContext),
}
