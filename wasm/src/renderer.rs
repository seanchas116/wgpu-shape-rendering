use log::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen]
    pub async fn new(canvas: web_sys::HtmlCanvasElement) -> Renderer {
        info!("renderer");

        let width = canvas.width();
        let height = canvas.height();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = if cfg!(target_arch = "wasm32") {
            instance.create_surface_from_canvas(&canvas)
        } else {
            unimplemented!()
        };

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
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let modes = surface.get_supported_modes(&adapter);

        Self {
            surface,
            device,
            queue,
            config,
            width,
            height,
        }
    }
}
