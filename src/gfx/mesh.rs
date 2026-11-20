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

    /// Generates a smooth UV Sphere with latitude and longitude rings.
    pub fn uv_sphere() -> Self {
        let radius = 0.75_f32;
        let sectors = 28_usize;
        let rings = 18_usize;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let r_inv = 1.0 / radius;
        let pi = std::f32::consts::PI;

        for r in 0..=rings {
            let v = r as f32 / rings as f32;
            let phi = v * pi;

            for s in 0..=sectors {
                let u = s as f32 / sectors as f32;
                let theta = u * 2.0 * pi;

                let x = radius * phi.sin() * theta.cos();
                let y = radius * phi.cos();
                let z = radius * phi.sin() * theta.sin();

                let normal = [x * r_inv, y * r_inv, z * r_inv];

                let color = [
                    0.4 + 0.6 * ((y / radius) + 1.0) * 0.5,
                    0.3 + 0.7 * (theta.sin() * 0.5 + 0.5),
                    0.8 + 0.2 * (theta.cos() * 0.5 + 0.5),
                ];

                vertices.push(Vertex {
                    position: [x, y, z],
                    normal,
                    color,
                });
            }
        }

        for r in 0..rings {
            for s in 0..sectors {
                let first = (r * (sectors + 1) + s) as u16;
                let second = first + (sectors as u16) + 1;

                indices.push(first);
                indices.push(second);
                indices.push(first + 1);

                indices.push(second);
                indices.push(second + 1);
                indices.push(first + 1);
            }
        }

        Self { vertices, indices }
    }

    /// Generates a smooth 3D Torus (Donut) mesh.
    pub fn torus() -> Self {
        let major_r = 0.65_f32;
        let minor_r = 0.24_f32;
        let radial_segs = 28_usize;
        let tubular_segs = 18_usize;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let pi = std::f32::consts::PI;

        for i in 0..=radial_segs {
            let u = i as f32 / radial_segs as f32 * 2.0 * pi;
            for j in 0..=tubular_segs {
                let v = j as f32 / tubular_segs as f32 * 2.0 * pi;

                let x = (major_r + minor_r * v.cos()) * u.cos();
                let y = minor_r * v.sin();
                let z = (major_r + minor_r * v.cos()) * u.sin();

                let center_x = major_r * u.cos();
                let center_z = major_r * u.sin();
                let norm = glam::Vec3::new(x - center_x, y, z - center_z)
                    .normalize()
                    .to_array();

                let color = [1.0, 0.4 + 0.6 * v.sin().abs(), 0.3 + 0.7 * u.cos().abs()];

                vertices.push(Vertex {
                    position: [x, y, z],
                    normal: norm,
                    color,
                });
            }
        }

        for i in 0..radial_segs {
            for j in 0..tubular_segs {
                let first = (i * (tubular_segs + 1) + j) as u16;
                let second = ((i + 1) * (tubular_segs + 1) + j) as u16;

                indices.push(first);
                indices.push(second);
                indices.push(first + 1);

                indices.push(second);
                indices.push(second + 1);
                indices.push(first + 1);
            }
        }

        Self { vertices, indices }
    }

    /// Generates an Architectural Cylinder with smooth sides and flat caps.
    pub fn cylinder() -> Self {
        let radius = 0.55_f32;
        let height = 1.0_f32;
        let segments = 24_usize;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let half_h = height * 0.5;
        let pi = std::f32::consts::PI;

        // Sides
        for i in 0..segments {
            let next = (i + 1) % segments;
            let theta1 = i as f32 / segments as f32 * 2.0 * pi;
            let theta2 = next as f32 / segments as f32 * 2.0 * pi;

            let x1 = radius * theta1.cos();
            let z1 = radius * theta1.sin();
            let x2 = radius * theta2.cos();
            let z2 = radius * theta2.sin();

            let n1 = [theta1.cos(), 0.0, theta1.sin()];
            let n2 = [theta2.cos(), 0.0, theta2.sin()];

            let base = vertices.len() as u16;
            let col1 = [0.15, 0.7, 0.95];
            let col2 = [0.3, 0.9, 1.0];

            vertices.push(Vertex {
                position: [x1, -half_h, z1],
                normal: n1,
                color: col1,
            });
            vertices.push(Vertex {
                position: [x1, half_h, z1],
                normal: n1,
                color: col2,
            });
            vertices.push(Vertex {
                position: [x2, half_h, z2],
                normal: n2,
                color: col2,
            });
            vertices.push(Vertex {
                position: [x2, -half_h, z2],
                normal: n2,
                color: col1,
            });

            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 3);
        }

        // Top cap
        let top_center = vertices.len() as u16;
        vertices.push(Vertex {
            position: [0.0, half_h, 0.0],
            normal: [0.0, 1.0, 0.0],
            color: [0.5, 0.95, 1.0],
        });
        for i in 0..segments {
            let theta = i as f32 / segments as f32 * 2.0 * pi;
            vertices.push(Vertex {
                position: [radius * theta.cos(), half_h, radius * theta.sin()],
                normal: [0.0, 1.0, 0.0],
                color: [0.4, 0.85, 0.95],
            });
        }
        for i in 0..segments {
            let curr = top_center + 1 + i as u16;
            let next = top_center + 1 + ((i + 1) % segments) as u16;
            indices.push(top_center);
            indices.push(curr);
            indices.push(next);
        }

        // Bottom cap
        let bot_center = vertices.len() as u16;
        vertices.push(Vertex {
            position: [0.0, -half_h, 0.0],
            normal: [0.0, -1.0, 0.0],
            color: [0.15, 0.5, 0.7],
        });
        for i in 0..segments {
            let theta = i as f32 / segments as f32 * 2.0 * pi;
            vertices.push(Vertex {
                position: [radius * theta.cos(), -half_h, radius * theta.sin()],
                normal: [0.0, -1.0, 0.0],
                color: [0.15, 0.5, 0.7],
            });
        }
        for i in 0..segments {
            let curr = bot_center + 1 + i as u16;
            let next = bot_center + 1 + ((i + 1) % segments) as u16;
            indices.push(bot_center);
            indices.push(next);
            indices.push(curr);
        }

        Self { vertices, indices }
    }

    /// Generates a subdivided ground plane.
    pub fn subdivided_plane() -> Self {
        let size = 2.4_f32;
        let subdivisions = 8_usize;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let half_s = size * 0.5;
        let step = size / subdivisions as f32;

        for i in 0..=subdivisions {
            let z = -half_s + i as f32 * step;
            for j in 0..=subdivisions {
                let x = -half_s + j as f32 * step;
                let is_even = (i + j) % 2 == 0;
                let color = if is_even {
                    [0.4, 0.5, 0.7]
                } else {
                    [0.2, 0.3, 0.45]
                };
                vertices.push(Vertex {
                    position: [x, 0.0, z],
                    normal: [0.0, 1.0, 0.0],
                    color,
                });
            }
        }

        for i in 0..subdivisions {
            for j in 0..subdivisions {
                let first = (i * (subdivisions + 1) + j) as u16;
                let second = ((i + 1) * (subdivisions + 1) + j) as u16;

                indices.push(first);
                indices.push(first + 1);
                indices.push(second);

                indices.push(second);
                indices.push(first + 1);
                indices.push(second + 1);
            }
        }

        Self { vertices, indices }
    }

    /// Generates a Blender-Style 3D Ground Grid Floor with colored axes.
    pub fn grid() -> Self {
        let size = 8.0_f32;
        let divisions = 16_usize;
        let y = -0.85_f32;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let half = size * 0.5;
        let step = size / divisions as f32;
        let half_w = 0.007_f32; // width of thin quad line

        let normal = [0.0, 1.0, 0.0];

        // Draw grid lines along X
        for i in 0..=divisions {
            let z = -half + i as f32 * step;
            let is_axis = z.abs() < step * 0.4;
            let color = if is_axis {
                [0.95, 0.25, 0.25] // X-axis Red
            } else {
                [0.22, 0.28, 0.38] // Subtle grid line
            };

            let base = vertices.len() as u16;
            vertices.push(Vertex {
                position: [-half, y, z - half_w],
                normal,
                color,
            });
            vertices.push(Vertex {
                position: [half, y, z - half_w],
                normal,
                color,
            });
            vertices.push(Vertex {
                position: [half, y, z + half_w],
                normal,
                color,
            });
            vertices.push(Vertex {
                position: [-half, y, z + half_w],
                normal,
                color,
            });

            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 3);
        }

        // Draw grid lines along Z
        for i in 0..=divisions {
            let x = -half + i as f32 * step;
            let is_axis = x.abs() < step * 0.4;
            let color = if is_axis {
                [0.25, 0.55, 0.98] // Z-axis Blue
            } else {
                [0.22, 0.28, 0.38] // Subtle grid line
            };

            let base = vertices.len() as u16;
            vertices.push(Vertex {
                position: [x - half_w, y, -half],
                normal,
                color,
            });
            vertices.push(Vertex {
                position: [x + half_w, y, -half],
                normal,
                color,
            });
            vertices.push(Vertex {
                position: [x + half_w, y, half],
                normal,
                color,
            });
            vertices.push(Vertex {
                position: [x - half_w, y, half],
                normal,
                color,
            });

            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 3);
        }

        Self { vertices, indices }
    }
}
