use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use wgpu::util::DeviceExt;

use crate::core::MeshType;
use crate::gfx::mesh::{MeshData, Vertex};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ShaderUniforms {
    pub view_proj: [f32; 16],
    pub model: [f32; 16],
    pub light_dir: [f32; 3],
    pub ambient_strength: f32,
    pub color_tint: [f32; 4],
    pub camera_pos: [f32; 3],
    pub specular_strength: f32,
    pub light_color: [f32; 3],
    pub shininess: f32,
}

pub struct GpuMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

pub struct ScenePipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub hex_mesh: GpuMesh,
    pub cube_mesh: GpuMesh,
    pub pyramid_mesh: GpuMesh,
    pub sphere_mesh: GpuMesh,
    pub torus_mesh: GpuMesh,
    pub cylinder_mesh: GpuMesh,
    pub plane_mesh: GpuMesh,
    pub grid_mesh: GpuMesh,
    pub current_mesh: MeshType,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub grid_uniform_buffer: wgpu::Buffer,
    pub grid_bind_group: wgpu::BindGroup,
}

impl ScenePipeline {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3D Scene WGSL Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/scene.wgsl").into()),
        });

        let dummy_uniforms = ShaderUniforms {
            view_proj: Mat4::IDENTITY.to_cols_array(),
            model: Mat4::IDENTITY.to_cols_array(),
            light_dir: [0.5, -1.0, -0.6],
            ambient_strength: 0.35,
            color_tint: [1.0, 1.0, 1.0, 1.0],
            camera_pos: [0.0, 0.0, 3.2],
            specular_strength: 0.7,
            light_color: [1.0, 1.0, 1.0],
            shininess: 32.0,
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene Uniform Buffer"),
            contents: bytemuck::cast_slice(&[dummy_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Scene Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene Uniform Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let grid_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Grid Uniform Buffer"),
            contents: bytemuck::cast_slice(&[dummy_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let grid_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Grid Uniform Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: grid_uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Scene Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let depth_stencil = depth_format.map(|format| wgpu::DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3D Scene Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Double sided for transparent rotation view
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Pre-bake all primitive mesh buffers into GPU VRAM (Zero runtime allocations)
        let hex_data = MeshData::hexagonal_prism();
        let hex_mesh = GpuMesh {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Hexagon Vertex Buffer"),
                contents: bytemuck::cast_slice(&hex_data.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Hexagon Index Buffer"),
                contents: bytemuck::cast_slice(&hex_data.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            num_indices: hex_data.indices.len() as u32,
        };

        let cube_data = MeshData::cube();
        let cube_mesh = GpuMesh {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Cube Vertex Buffer"),
                contents: bytemuck::cast_slice(&cube_data.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Cube Index Buffer"),
                contents: bytemuck::cast_slice(&cube_data.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            num_indices: cube_data.indices.len() as u32,
        };

        let pyramid_data = MeshData::pyramid();
        let pyramid_mesh = GpuMesh {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Pyramid Vertex Buffer"),
                contents: bytemuck::cast_slice(&pyramid_data.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Pyramid Index Buffer"),
                contents: bytemuck::cast_slice(&pyramid_data.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            num_indices: pyramid_data.indices.len() as u32,
        };

        let sphere_data = MeshData::uv_sphere();
        let sphere_mesh = GpuMesh {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Sphere Vertex Buffer"),
                contents: bytemuck::cast_slice(&sphere_data.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Sphere Index Buffer"),
                contents: bytemuck::cast_slice(&sphere_data.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            num_indices: sphere_data.indices.len() as u32,
        };

        let torus_data = MeshData::torus();
        let torus_mesh = GpuMesh {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Torus Vertex Buffer"),
                contents: bytemuck::cast_slice(&torus_data.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Torus Index Buffer"),
                contents: bytemuck::cast_slice(&torus_data.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            num_indices: torus_data.indices.len() as u32,
        };

        let cylinder_data = MeshData::cylinder();
        let cylinder_mesh = GpuMesh {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Cylinder Vertex Buffer"),
                contents: bytemuck::cast_slice(&cylinder_data.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Cylinder Index Buffer"),
                contents: bytemuck::cast_slice(&cylinder_data.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            num_indices: cylinder_data.indices.len() as u32,
        };

        let plane_data = MeshData::subdivided_plane();
        let plane_mesh = GpuMesh {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Plane Vertex Buffer"),
                contents: bytemuck::cast_slice(&plane_data.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Plane Index Buffer"),
                contents: bytemuck::cast_slice(&plane_data.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            num_indices: plane_data.indices.len() as u32,
        };

        let grid_data = MeshData::grid();
        let grid_mesh = GpuMesh {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Grid Vertex Buffer"),
                contents: bytemuck::cast_slice(&grid_data.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Grid Index Buffer"),
                contents: bytemuck::cast_slice(&grid_data.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            num_indices: grid_data.indices.len() as u32,
        };

        Self {
            pipeline,
            hex_mesh,
            cube_mesh,
            pyramid_mesh,
            sphere_mesh,
            torus_mesh,
            cylinder_mesh,
            plane_mesh,
            grid_mesh,
            current_mesh: MeshType::HexagonPrism,
            uniform_buffer,
            bind_group,
            grid_uniform_buffer,
            grid_bind_group,
        }
    }

    #[inline(always)]
    pub fn active_mesh(&self) -> &GpuMesh {
        match self.current_mesh {
            MeshType::HexagonPrism => &self.hex_mesh,
            MeshType::Cube => &self.cube_mesh,
            MeshType::Pyramid => &self.pyramid_mesh,
            MeshType::Sphere => &self.sphere_mesh,
            MeshType::Torus => &self.torus_mesh,
            MeshType::Cylinder => &self.cylinder_mesh,
            MeshType::Plane => &self.plane_mesh,
        }
    }

    #[inline(always)]
    pub fn grid_mesh(&self) -> &GpuMesh {
        &self.grid_mesh
    }

    #[inline(always)]
    pub fn set_mesh(&mut self, mesh_type: MeshType) {
        self.current_mesh = mesh_type;
    }

    #[inline(always)]
    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniforms: &ShaderUniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[*uniforms]));
    }

    #[inline(always)]
    pub fn update_grid_uniforms(&self, queue: &wgpu::Queue, uniforms: &ShaderUniforms) {
        queue.write_buffer(
            &self.grid_uniform_buffer,
            0,
            bytemuck::cast_slice(&[*uniforms]),
        );
    }
}
