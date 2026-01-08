
#import bevy_sprite::mesh2d_vertex_output::VertexOutput


@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<storage, read> id: array<u32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> width: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> height: u32;


@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> selected_id: u32;




@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    let x = min(u32(in.uv.x * f32(width)),  width - 1u);
    let y = min(u32(in.uv.y * f32(height)), height - 1u);
    let acc: u32 = y * width + x;

    if id[acc] == selected_id{
            return vec4f(0.0, 1.0, 0.0, 1.0);
    }
    
    return vec4f(1.0, 1.0, 1.0, 1.0);

    // switch id[acc] {
    //     case 0: {
    //         return vec4f(0.0, 1.0, 0.0, 1.0);
    //     }
    //     case 1: {
    //         return vec4f(1.0, 1.0, 1.0, 1.0);
    //     }
    //     case 2: {
    //         return vec4f(1.0, 1.0, 1.0, 1.0);
    //     }
    //     case 3: {
    //         return vec4f(1.0, 1.0, 1.0, 1.0);
    //     }
    //     default {
    //         return vec4f(0.0, 0.0, 0.0, 1.0);
    //     }
    // }


}