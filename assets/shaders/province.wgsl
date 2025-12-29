#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var map_texture: texture_2d<f32>;

@group(2) @binding(1)
var map_sampler: sampler;

@group(2) @binding(2)
var id_texture: texture_2d<f32>;

@group(2) @binding(3)
var id_sampler: sampler;

@group(2) @binding(4)
var<uniform> selected_color: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = textureSample(map_texture, map_sampler, in.uv);
    let province_color = textureSample(id_texture, id_sampler, in.uv);

    // Check if a province is selected (not the default black)
    let has_selection = selected_color.r > 0.01 || selected_color.g > 0.01 || selected_color.b > 0.01;
    
    if (has_selection) {
        // DEBUG: Visualize the color difference
        let color_diff = abs(province_color.rgb - selected_color.rgb);
        let max_diff = max(max(color_diff.r, color_diff.g), color_diff.b);
        
        // If difference is very small (less than 1/255 = 0.004), turn white
        if (max_diff < 0.006) {
            return vec4<f32>(1.0, 1.0, 1.0, 1.0);
        }
        
        // DEBUG: Show red where colors are close but not exact match
        if (max_diff < 0.02) {
            return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
    }

    return base_color;
}
