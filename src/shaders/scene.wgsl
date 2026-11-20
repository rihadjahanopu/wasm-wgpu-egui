// 3D Scene Shader with Directional Diffuse + Ambient + Blinn-Phong Specular Lighting
struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    light_dir: vec3<f32>,
    ambient_strength: f32,
    color_tint: vec4<f32>,
    camera_pos: vec3<f32>,
    specular_strength: f32,
    light_color: vec3<f32>,
    shininess: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = uniforms.model * vec4<f32>(in.position, 1.0);
    out.clip_position = uniforms.view_proj * world_pos;
    out.world_pos = world_pos.xyz;
    
    // Normal transformation (assuming uniform scaling or orthonormal matrix)
    let normal_matrix = mat3x3<f32>(
        uniforms.model[0].xyz,
        uniforms.model[1].xyz,
        uniforms.model[2].xyz
    );
    out.world_normal = normalize(normal_matrix * in.normal);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let norm = normalize(in.world_normal);
    let light = normalize(uniforms.light_dir);
    
    // 1. Half-Lambert / Diffuse shading
    let n_dot_l = max(dot(norm, -light), 0.0);
    let diffuse = n_dot_l * 0.75 * uniforms.light_color;
    let ambient = uniforms.ambient_strength * uniforms.light_color;
    
    // 2. Blinn-Phong Specular Reflection
    let view_dir = normalize(uniforms.camera_pos - in.world_pos);
    let half_dir = normalize(-light + view_dir);
    let n_dot_h = max(dot(norm, half_dir), 0.0);
    let specular = pow(n_dot_h, max(uniforms.shininess, 1.0)) * uniforms.specular_strength * uniforms.light_color;
    
    // 3. Combine Shading
    let base_color = in.color * uniforms.color_tint.rgb;
    let final_rgb = (base_color * (ambient + diffuse)) + specular;
    
    return vec4<f32>(final_rgb, uniforms.color_tint.a);
}
