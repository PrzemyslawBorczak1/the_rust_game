
#import bevy_sprite::mesh2d_vertex_output::VertexOutput


@group(#{MATERIAL_BIND_GROUP}) @binding(1) var id_text: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var id_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(3) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var texture_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> selected: vec4<u32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<storage, read> id: array<u32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> width: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> height: u32;



@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let x = min(u32(uv.x * f32(width)),  width - 1u);
    let y = min(u32(uv.y * f32(height)), height - 1u);

    let base = (x + y * width) * 4u;

    var matches = true;
    for (var i: u32 = 0u; i < 4u; i = i + 1u) {
        if (id[base + i] != selected[i]) {
            matches = false;
            break;
        }
    }

    if (matches) {
        return vec4f(0.0, 0.0, 0.0, 0.0);
    }

    return textureSample(texture, texture_sampler, uv);
}