mod renderer_backend;

use glfw::{Action, Context, Key, Window, fail_on_errors};
use renderer_backend::{mesh_builder, pipeline_builder::PipelineBuilder};
use wgpu::{
    CommandEncoderDescriptor, Device, DeviceDescriptor, Instance, InstanceDescriptor, LoadOp,
    Operations, PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor, StoreOp,
    Surface, SurfaceConfiguration, SurfaceTargetUnsafe, TextureUsages,
};

struct State<'a> {
    instance: Instance,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: (i32, i32),
    window: &'a mut Window,
    render_pipeline: wgpu::RenderPipeline,
    triangle_mesh: wgpu::Buffer,
}

impl<'a> State<'a> {
    async fn new(window: &'a mut Window) -> Self {
        let size = window.get_framebuffer_size();

        let instance_descriptor = InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };

        let instance = wgpu::Instance::new(&instance_descriptor);

        let target = unsafe { SurfaceTargetUnsafe::from_window(&window).unwrap() };

        let surface = unsafe { instance.create_surface_unsafe(target).unwrap() };

        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        };
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        };

        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let triangle_mesh = mesh_builder::make_triangle(&device);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        pipeline_builder.set_buffer_layout(mesh_builder::Vertex::get_layout());
        let render_pipeline = pipeline_builder.build_pipeline(&device);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            triangle_mesh,
        }
    }

    fn render(&mut self) -> anyhow::Result<()> {
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = CommandEncoderDescriptor {
            label: Some("render encoder"),
        };

        let mut command_encoder = self
            .device
            .create_command_encoder(&command_encoder_descriptor);

        let color_attatchment = RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                store: StoreOp::Store,
            },
        };
        let render_pass_descriptor = RenderPassDescriptor {
            label: Some("renderpass"),
            color_attachments: &[Some(color_attatchment)],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        };

        {
            let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.triangle_mesh.slice(..));
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        anyhow::Ok(())
    }

    fn resize(&mut self, new_size: (i32, i32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);
            self.update_surface();
        }
    }

    fn update_surface(&mut self) {
        let target = unsafe { SurfaceTargetUnsafe::from_window(&self.window).unwrap() };

        self.surface = unsafe { self.instance.create_surface_unsafe(target).unwrap() };

        self.surface.configure(&self.device, &self.config);
    }
}

pub async fn run() -> anyhow::Result<()> {
    let mut glfw = glfw::init(fail_on_errors!())?;

    let (mut window, events) = glfw
        .create_window(800, 600, "teacup", glfw::WindowMode::Windowed)
        .unwrap();

    // window.set_all_polling(true);
    window.set_key_polling(true);
    window.set_size_polling(true);
    window.make_current();

    let mut state = State::new(&mut window).await;

    while !state.window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close
                | glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _)
                | glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => {
                    state.window.set_should_close(true)
                }
                glfw::WindowEvent::Size(x, y) => {
                    state.resize((x, y));
                }
                _ => {
                    println!("{:?}", event);
                }
            }
        }

        match state.render() {
            Ok(_) => {}
            Err(e) => eprintln!("{:?}", e),
        }

        state.window.swap_buffers();
    }

    anyhow::Ok(())
}
