use std::sync::Arc;

use crate::boilerplate::{create_adapter, create_device, create_surface_config};
use rand::Rng;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayoutEntry, BlendState, Buffer, BufferBindingType,
    BufferDescriptor, BufferUsages, ColorWrites, Device, FragmentState, Instance, Queue,
    RenderPipeline, RenderPipelineDescriptor, ShaderStages, Surface, SurfaceConfiguration,
};
use winit::window::Window;

pub(crate) struct AppState {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    triangles: Vec<Triangle>,
}

struct Triangle {
    bind_group: BindGroup,
    scale_buffer: Buffer,
    scale: [f32; 2],
}
const TOTAL: usize = 100;
impl Triangle {
    fn new(
        device: &Device,
        render_pipeline: &RenderPipeline,
        color_offset: [f32; 8],
        scale: [f32; 2],
    ) -> Self {
        let data_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Data Buffer Descriptor"),
            contents: bytemuck::bytes_of(&color_offset),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let scale_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Scale Buffer Descriptor"),
            size: 4 * 2,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let data_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Data Bind Group Descriptor"),
            layout: &render_pipeline.get_bind_group_layout(0),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(data_buffer.as_entire_buffer_binding()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        scale_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        });

        Self {
            bind_group: data_bind_group,
            scale_buffer,
            scale,
        }
    }

    fn write_scale(&self, queue: &Queue) {
        queue.write_buffer(&self.scale_buffer, 0, bytemuck::bytes_of(&self.scale));
    }

    fn scale(&mut self) -> &mut [f32] {
        &mut self.scale
    }
}

impl AppState {
    pub(crate) fn new(window: Arc<Window>) -> AppState {
        let instance = Instance::new(wgpu::InstanceDescriptor::default());
        let size = window.inner_size();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = create_adapter(&instance);
        let (device, queue) = create_device(&adapter);
        let config = create_surface_config(&surface, &adapter, &size);
        let render_pipeline = Self::setup(&device, &config);

        let mut triangles = Vec::with_capacity(100);
        let mut rng = rand::thread_rng();
        for _ in 1..=TOTAL {
            let scale = rng.gen_range(0.1..0.3);
            let triangle = Triangle::new(
                &device,
                &render_pipeline,
                [
                    rng.gen(),
                    rng.gen(),
                    rng.gen(),
                    1.0,
                    rng.gen_range(-0.9..=0.9),
                    rng.gen_range(-0.9..=0.9),
                    0.0, // Padding
                    0.0, // Padding
                ],
                [scale, scale],
            );
            triangles.push(triangle);
        }

        Self {
            window,
            surface,
            device,
            queue,
            config,
            render_pipeline,
            triangles,
        }
    }

    pub(crate) fn window(&self) -> Arc<Window> {
        Arc::clone(&self.window)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn setup(device: &Device, config: &SurfaceConfiguration) -> RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor {
                        label: Some("Bind Group Layout 0"),
                        entries: &[
                            BindGroupLayoutEntry {
                                ty: wgpu::BindingType::Buffer {
                                    ty: BufferBindingType::Uniform,
                                    has_dynamic_offset: false,
                                    min_binding_size: None,
                                },
                                count: None,
                                binding: 0,
                                visibility: ShaderStages::VERTEX_FRAGMENT,
                            },
                            BindGroupLayoutEntry {
                                ty: wgpu::BindingType::Buffer {
                                    ty: BufferBindingType::Uniform,
                                    has_dynamic_offset: false,
                                    min_binding_size: None,
                                },
                                count: None,
                                binding: 1,
                                visibility: ShaderStages::VERTEX,
                            },
                        ],
                    },
                )],
                push_constant_ranges: &[],
            });
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
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

    pub(crate) fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.render_pipeline);

            let aspect = (output.texture.width() / output.texture.height()) as f32;
            for triangle in &mut self.triangles {
                let scale = triangle.scale();
                scale[0] /= aspect;
                triangle.write_scale(&self.queue);
                pass.set_bind_group(0, &triangle.bind_group, &[]);
                pass.draw(0..3, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
