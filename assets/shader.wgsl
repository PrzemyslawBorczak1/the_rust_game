
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


@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> time: f32;


const TERRAIN_FLAT: u32 = 0u;
const TERRAIN_WATER: u32 = 1u;
const TERRAIN_MOUNTAIN: u32 = 2u;
 const TERRAIN_FOREST: u32 = 3u;



const NO_SELECTED_ID: u32 = 213767u;
const GEOGRAPHICAL_DRAW: u32 = 0u;
const POLITICAL_DRAW: u32 = 1u;
const ATTACK_DRAW: u32 = 2u;



const NO_OWNER: u32 = 213767;

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
        case ATTACK_DRAW: {
            return attack_draw(id[acc]);
        }
        default {
            return vec4f(1.0, 1.0, 1.0, 1.0);
        }
    }
}


fn political(province_id: u32) -> vec4f{
    if provinces[province_id].terrain_type == TERRAIN_WATER{
        return vec4f(0.005f, 0.005f, 0.005f, 1f);
    }

	let country = provinces[province_id].owner_id;
	let color = countries[country].color;
    return color;
}


const terrain_colors = array<vec4f, 4>(
    vec4f(0.2, 0.8, 0.2, 1.0),  // 0: FLAT
    vec4f(0.2, 0.4, 0.8, 1.0),  // 1: WATER
    vec4f(0.6, 0.6, 0.6, 1.0),  // 2: MOUNTAIN
    vec4f(0.4, 0.6, 0.2, 1.0),  // 3: FOREST 
);

fn geographical(province_id: u32) -> vec4f{
    return terrain_colors[provinces[province_id].terrain_type];
}

const attack_color : vec4f = vec4f(0.15, 0.78, 0.72, 0.45);
fn attack_draw(province_id: u32) -> vec4f{
    if provinces[province_id].terrain_type == TERRAIN_WATER{
        return vec4f(0.005f, 0.005f, 0.005f, 1f);
    }

    if provinces[selected_id].owner_id == provinces[province_id].owner_id{
        return vec4f(0.01f, 0.01f, 0.01f, 1f);
    }

    let country = provinces[province_id].owner_id;
    if country != NO_OWNER{
        let color = countries[country].color;
        return mix(color, attack_color, 0.7);
    }

    
    if provinces[selected_id].owner_id == NO_OWNER {
        return vec4f(0.01f, 0.01f, 0.01f, 1f);
    }
    else{
        return attack_color * (0.40 + 0.60 * (0.5 + 0.5 * sin(time * 2.5)));
    }


}


