
#import bevy_sprite::mesh2d_vertex_output::VertexOutput


@group(#{MATERIAL_BIND_GROUP}) @binding(1) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var texture_sampler: sampler;



struct test{
    a: u32,
    b: test2,
}

struct test2{
    a: u32,
    b: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<storage, read> id: array<test>;



@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    let acc: u32 = min(u32(floor(in.uv.x * 9.0)), 8u);

    if (id[acc].b.a == 2) {
        return vec4f(1.0, 1.0, 0.0, 1.0);
    }

    return textureSample(texture, texture_sampler, in.uv);
}