struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(1) @binding(0)
var map_texture: texture_2d<f32>;

@group(1) @binding(1)
var map_sampler: sampler;

@group(1) @binding(2)
var id_texture: texture_2d<f32>;

@group(1) @binding(3)
var id_sampler: sampler;

@group(1) @binding(4)
var<uniform> selected_color: vec4<f32>;

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(vertex.position, 1.0);
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = textureSample(map_texture, map_sampler, in.uv);
    let province_color = textureSample(id_texture, id_sampler, in.uv);

    // Compare province color with selected color
    let is_selected =
        all(abs(province_color.rgb - selected_color.rgb) < vec3<f32>(0.01));

    if (is_selected) {
        return vec4<f32>(base_color.rgb * 1.3, base_color.a);
    }

    return base_color;
}
