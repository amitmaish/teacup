pub struct PipelineBuilder {
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        PipelineBuilder {
            shader_filename: "dummy".to_string(),
            vertex_entry: "dummy".to_string(),
            fragment_entry: "dummy".to_string(),
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
            vertex_buffer_layouts: Vec::new(),
        }
    }

    pub fn set_shader_module(
        &mut self,
        shader_filename: &str,
        vertex_entry: &str,
        fragment_entry: &str,
    ) {
        self.shader_filename = shader_filename.to_string();
        self.vertex_entry = vertex_entry.to_string();
        self.fragment_entry = fragment_entry.to_string();
    }

    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn set_buffer_layout(&mut self, layout: wgpu::VertexBufferLayout<'static>) {
        self.vertex_buffer_layouts.push(layout);
    }

    pub fn build_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader module"),
            source: wgpu::ShaderSource::Wgsl(default_shader::SOURCE.into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(
            &(wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }),
        );

        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some(&self.vertex_entry),
                buffers: &self.vertex_buffer_layouts,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                targets: &render_targets,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }
}

mod default_shader {
    wgsl_inline::wgsl!(
    struct Vertex {
        @location(0) position: vec3<f32>,
        @location(1) color: vec3<f32>,
    }

    struct VertexPayload {
        @builtin(position) position: vec4<f32>,
        @location(0) color: vec3<f32>,
    };

    @vertex
    fn vs_main(vertex: Vertex) -> VertexPayload {

        var out: VertexPayload;
        out.position = vec4<f32>(vertex.position, 1.0);
        out.color = vertex.color;
        return out;
    }

    @fragment
    fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
        return vec4<f32>(in.color, 1.0);
    }
    );
}
