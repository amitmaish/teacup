mod layout;
mod renderer;

use std::sync::{Arc, Mutex};

use glfw::{Action, Context, Key, Window, fail_on_errors};
use layout::{Container, LayoutMode, Rectangle, Sizing, UI};
use renderer::{
    mesh_builder::{self},
    pipeline_builder::PipelineBuilder,
};
use tinycolors as color;
use wgpu::{
    CommandEncoderDescriptor, Device, DeviceDescriptor, Instance, InstanceDescriptor, LoadOp,
    Operations, PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor, StoreOp,
    Surface, SurfaceConfiguration, SurfaceTargetUnsafe, TextureUsages,
};

struct State<'a> {
    window: &'a mut Window,
    instance: Instance,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: (i32, i32),
    render_pipeline: wgpu::RenderPipeline,
    ui: UI,
}

impl<'a> State<'a> {
    async fn new(window: &'a mut Window) -> Self {
        let size = window.get_size();

        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let target = unsafe { SurfaceTargetUnsafe::from_window(&window).unwrap() };

        let surface = unsafe { instance.create_surface_unsafe(target).unwrap() };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Device"),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_capabilities
                .formats
                .iter()
                .copied()
                .find(|f| f.is_srgb())
                .unwrap_or(surface_capabilities.formats[0]),
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        pipeline_builder.set_buffer_layout(mesh_builder::Vertex::get_layout());
        let render_pipeline = pipeline_builder.build_pipeline(&device);

        let mut ui = UI {
            size: (size.0 * 2, size.1 * 2),
            ..Default::default()
        };
        let mut root = Rectangle {
            layout_mode: LayoutMode::LeftToRight,
            sizing: Sizing::GROW,
            padding: 16,
            child_gap: 16,
            color: color::srgb::RED,
            ..Default::default()
        };

        let child = Rectangle {
            sizing: Sizing::GROW,
            color: color::srgb::GREEN,
            min_width: 100,
            max_width: Some(200),
            ..Default::default()
        };
        root.children.push(Arc::new(Mutex::new(child)));

        let child = Rectangle {
            sizing: Sizing::GROW,
            color: color::srgb::PURPLE,
            ..Default::default()
        };
        root.children.push(Arc::new(Mutex::new(child)));

        let child = Rectangle {
            sizing: Sizing::GROW,
            color: color::srgb::AQUA,
            ..Default::default()
        };
        root.children.push(Arc::new(Mutex::new(child)));

        let mut child = Rectangle {
            layout_mode: LayoutMode::TopToBottom,
            sizing: Sizing::GROW,
            padding: 16,
            child_gap: 16,
            color: color::srgb::BLUE,
            ..Default::default()
        };

        let inner = Rectangle {
            sizing: Sizing::GROW,
            min_width: 100,
            min_height: 50,
            color: color::srgb::WHITE,
            ..Default::default()
        };
        child.children.push(Arc::new(Mutex::new(inner)));

        let inner = Rectangle {
            sizing: Sizing::GROW,
            min_width: 100,
            min_height: 50,
            color: color::srgb::BLACK,
            ..Default::default()
        };
        child.children.push(Arc::new(Mutex::new(inner)));

        root.children.push(Arc::new(Mutex::new(child)));

        ui.root_item = Arc::new(Mutex::new(root));

        Self {
            window,
            instance,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            ui,
        }
    }

    fn render(&mut self) -> anyhow::Result<()> {
        let drawable = self.surface.get_current_texture()?;
        let image_view = drawable
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("render encoder"),
            });

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
        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("renderpass"),
                color_attachments: &[Some(color_attatchment)],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            self.ui.compute_layout();
            self.ui.draw(&mut render_pass, &self.device, self.size);
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
            self.ui.size = (new_size.0 * 2, new_size.1 * 2);
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
