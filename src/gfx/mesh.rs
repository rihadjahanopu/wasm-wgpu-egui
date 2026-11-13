use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x3
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl MeshData {
    /// Generates a true 3D extruded Hexagonal Prism with top and bottom caps and shaded sides.
    pub fn hexagonal_prism() -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let height = 0.5;
        let radius = 0.8;
        let num_sides = 6;

        let colors = [
            [0.0, 1.0, 1.0], // Cyan
            [1.0, 0.2, 0.8], // Hot Pink
            [1.0, 0.9, 0.1], // Amber Yellow
            [0.1, 1.0, 0.5], // Neon Mint
            [0.2, 0.5, 1.0], // Deep Azure
            [1.0, 0.4, 0.1], // Coral Red
        ];

        // Top center vertex
        let top_center_idx = vertices.len() as u16;
        vertices.push(Vertex {
            position: [0.0, height, 0.0],
            normal: [0.0, 1.0, 0.0],
            color: [1.0, 1.0, 1.0],
        });

        // Top perimeter vertices
        let top_start = vertices.len() as u16;
        for i in 0..num_sides {
            let angle = (i as f32) * std::f32::consts::TAU / (num_sides as f32);
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            vertices.push(Vertex {
                position: [x, height, z],
                normal: [0.0, 1.0, 0.0],
                color: colors[i % colors.len()],
            });
        }

        // Top cap triangles
        for i in 0..num_sides {
            let next = (i + 1) % num_sides;
            indices.push(top_center_idx);
            indices.push(top_start + i as u16);
            indices.push(top_start + next as u16);
        }

        // Bottom center vertex
        let bot_center_idx = vertices.len() as u16;
        vertices.push(Vertex {
            position: [0.0, -height, 0.0],
            normal: [0.0, -1.0, 0.0],
            color: [0.4, 0.4, 0.4],
        });

        // Bottom perimeter vertices
        let bot_start = vertices.len() as u16;
        for i in 0..num_sides {
            let angle = (i as f32) * std::f32::consts::TAU / (num_sides as f32);
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            vertices.push(Vertex {
                position: [x, -height, z],
                normal: [0.0, -1.0, 0.0],
                color: colors[i % colors.len()],
            });
        }

        // Bottom cap triangles (reversed winding for downward face)
        for i in 0..num_sides {
            let next = (i + 1) % num_sides;
            indices.push(bot_center_idx);
            indices.push(bot_start + next as u16);
            indices.push(bot_start + i as u16);
        }

        // Side quads
        for i in 0..num_sides {
            let next = (i + 1) % num_sides;
            let a1 = (i as f32) * std::f32::consts::TAU / (num_sides as f32);
            let a2 = (next as f32) * std::f32::consts::TAU / (num_sides as f32);

            let mid_angle = (a1 + a2) * 0.5;
            let norm = [mid_angle.cos(), 0.0, mid_angle.sin()];

            let base_idx = vertices.len() as u16;
            let c = colors[i % colors.len()];

            // 4 vertices for this side quad
            vertices.push(Vertex {
                position: [radius * a1.cos(), height, radius * a1.sin()],
                normal: norm,
                color: c,
            }); // 0
            vertices.push(Vertex {
                position: [radius * a2.cos(), height, radius * a2.sin()],
                normal: norm,
                color: colors[next % colors.len()],
            }); // 1
            vertices.push(Vertex {
                position: [radius * a2.cos(), -height, radius * a2.sin()],
                normal: norm,
                color: colors[next % colors.len()],
            }); // 2
            vertices.push(Vertex {
                position: [radius * a1.cos(), -height, radius * a1.sin()],
                normal: norm,
                color: c,
            }); // 3

            indices.push(base_idx);
            indices.push(base_idx + 1);
            indices.push(base_idx + 2);

            indices.push(base_idx);
            indices.push(base_idx + 2);
            indices.push(base_idx + 3);
        }

        Self { vertices, indices }
    }

    /// Generates a shaded 3D Cube with 6 face normals and distinct colors.
    pub fn cube() -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let s = 0.65;
        let faces = [
            // Normal, Color, 4 corners
            (
                [0.0, 0.0, 1.0],
                [0.1, 0.8, 1.0],
                [[-s, -s, s], [s, -s, s], [s, s, s], [-s, s, s]],
            ), // Front
            (
                [0.0, 0.0, -1.0],
                [1.0, 0.3, 0.3],
                [[s, -s, -s], [-s, -s, -s], [-s, s, -s], [s, s, -s]],
            ), // Back
            (
                [0.0, 1.0, 0.0],
                [0.3, 1.0, 0.3],
                [[-s, s, s], [s, s, s], [s, s, -s], [-s, s, -s]],
            ), // Top
            (
                [0.0, -1.0, 0.0],
                [0.9, 0.8, 0.2],
                [[-s, -s, -s], [s, -s, -s], [s, -s, s], [-s, -s, s]],
            ), // Bottom
            (
                [1.0, 0.0, 0.0],
                [1.0, 0.4, 0.8],
                [[s, -s, s], [s, -s, -s], [s, s, -s], [s, s, s]],
            ), // Right
            (
                [-1.0, 0.0, 0.0],
                [0.6, 0.2, 1.0],
                [[-s, -s, -s], [-s, -s, s], [-s, s, s], [-s, s, -s]],
            ), // Left
        ];

        for (norm, col, corners) in faces {
            let base = vertices.len() as u16;
            for p in corners {
                vertices.push(Vertex {
                    position: p,
                    normal: norm,
                    color: col,
                });
            }
            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);

            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 3);
        }

        Self { vertices, indices }
    }

    /// Generates a 3D Pyramid with apex and square base.
    pub fn pyramid() -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let b = 0.7;
        let h = 0.8;
        let apex = [0.0, h, 0.0];

        let base_corners = [
            [-b, -0.4, b],  // Front-left
            [b, -0.4, b],   // Front-right
            [b, -0.4, -b],  // Back-right
            [-b, -0.4, -b], // Back-left
        ];

        let side_colors = [
            [1.0, 0.2, 0.4], // Red-Pink
            [0.2, 0.8, 1.0], // Azure
            [0.3, 1.0, 0.4], // Lime
            [1.0, 0.8, 0.1], // Gold
        ];

        // 4 triangular sides
        for i in 0..4 {
            let next = (i + 1) % 4;
            let c1 = base_corners[i];
            let c2 = base_corners[next];

            // Calculate face normal
            let v1 = glam::Vec3::from(c1) - glam::Vec3::from(apex);
            let v2 = glam::Vec3::from(c2) - glam::Vec3::from(apex);
            let norm = v2.cross(v1).normalize().to_array();

            let base = vertices.len() as u16;
            vertices.push(Vertex {
                position: apex,
                normal: norm,
                color: [1.0, 1.0, 1.0],
            });
            vertices.push(Vertex {
                position: c1,
                normal: norm,
                color: side_colors[i],
            });
            vertices.push(Vertex {
                position: c2,
                normal: norm,
                color: side_colors[next],
            });

            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
        }

        // Base square
        let base_start = vertices.len() as u16;
        let base_norm = [0.0, -1.0, 0.0];
        let base_col = [0.4, 0.4, 0.5];
        for c in base_corners {
            vertices.push(Vertex {
                position: c,
                normal: base_norm,
                color: base_col,
            });
        }
        indices.push(base_start);
        indices.push(base_start + 2);
        indices.push(base_start + 1);

        indices.push(base_start);
        indices.push(base_start + 3);
        indices.push(base_start + 2);

        Self { vertices, indices }
    }
}
