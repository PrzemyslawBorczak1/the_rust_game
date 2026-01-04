
#import bevy_sprite::mesh2d_vertex_output::VertexOutput


@group(#{MATERIAL_BIND_GROUP}) @binding(1) var id_text: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var id_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(3) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var texture_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> selected: vec4<f32>;



@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color: vec4<f32> = textureSample(texture, texture_sampler, in.uv);

    let eps: vec4<f32> = vec4<f32>(2.0 / 255.0);

    

    if (all(abs(base_color - selected) <= eps)) {
        return vec4<f32>(1.0);
    }

    return base_color;
}