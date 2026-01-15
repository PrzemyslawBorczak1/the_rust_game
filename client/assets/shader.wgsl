
#import bevy_sprite::mesh2d_vertex_output::VertexOutput



@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<storage, read> id: array<u32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> width: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> height: u32;

struct Province{
  	owner_id: u32,
    terrain_type: u32,
}
 @group(#{MATERIAL_BIND_GROUP}) @binding(4) var<storage, read> provinces: array<Province>;

struct Country{
    color: vec4f,
}
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<storage, read> countries: array<Country>;

@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> selected_id: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> draw_mode: u32;




const NO_SELECTED_ID: u32 = 213767u;
const GEOGRAPHICAL_DRAW: u32 = 0u;
const POLITICAL_DRAW: u32 = 1u;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4f {

    let x = min(u32(in.uv.x * f32(width)),  width - 1u);
    let y = min(u32(in.uv.y * f32(height)), height - 1u);
    let acc: u32 = y * width + x;

	let province_id = id[acc];

	if selected_id == province_id && selected_id != NO_SELECTED_ID{
		return  vec4f(1.0, 1.0, 1.0, 1.0);
	}
	
    switch draw_mode{
        case POLITICAL_DRAW:{
            return political(id[acc]);
        }
        case GEOGRAPHICAL_DRAW: {
            return geographical(id[acc]);
        }
        default {
            return vec4f(1.0, 1.0, 1.0, 1.0);
        }
    }
}


fn political(province_id: u32) -> vec4f{
	let country = provinces[province_id].owner_id;
	let color = countries[country].color;
    return color;
}


const terrain_colors = array<vec4f, 4>(
    vec4f(0.2, 0.8, 0.2, 1.0),   
    vec4f(0.4, 0.6, 0.2, 1.0),   
    vec4f(0.6, 0.6, 0.6, 1.0),   
    vec4f(0.2, 0.4, 0.8, 1.0),  
);

fn geographical(province_id: u32) -> vec4f{
    return terrain_colors[provinces[province_id].terrain_type - 1];
}
