#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

#import bevy_pbr::{
    mesh_view_bindings::globals,
    prepass_utils,
    forward_io::VertexOutput,
}
@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;
@group(0) @binding(3) var depth_prepass_texture: texture_depth_2d;
@group(0) @binding(4) var normal_prepass_texture: texture_2d<f32>;

fn prepass_depth(frag_coord: vec2f) -> f32 {
    return textureLoad(depth_prepass_texture, vec2i(frag_coord), 0);
}

fn prepass_normal(frag_coord: vec2f) -> vec3f {
    return textureLoad(normal_prepass_texture, vec2i(frag_coord), 0).xyz;
}

fn texel_size() -> vec2f {
    return vec2f(1.0, 1.0) / vec2<f32>(textureDimensions(screen_texture));
}

fn uv_to_pos(uv: vec2f) -> vec2f {
    return uv * vec2<f32>(textureDimensions(screen_texture));
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    
    let _scale = 1.0;
    let texel_size = texel_size();

    let half_scale_floor = floor(_scale * 0.5);
    let half_scale_ceil = ceil(_scale * 0.5);

    let bottom_left_uv = in.uv - vec2f(texel_size.x, texel_size.y) * half_scale_floor;
    let top_right_uv = in.uv + vec2f(texel_size.x, texel_size.y) * half_scale_ceil;  
    let bottom_right_uv = in.uv + vec2f(texel_size.x * half_scale_ceil, -texel_size.y * half_scale_floor);
    let top_left_uv = in.uv + vec2f(-texel_size.x * half_scale_floor, texel_size.y * half_scale_ceil);

    let depth0 = prepass_depth(uv_to_pos(bottom_left_uv));
    let depth1 = prepass_depth(uv_to_pos(top_right_uv));
    let depth2 = prepass_depth(uv_to_pos(bottom_right_uv));
    let depth3 = prepass_depth(uv_to_pos(top_left_uv));

    let depthFiniteDifference0 = depth1 - depth0;
    let depthFiniteDifference1 = depth3 - depth2;

    return vec4(abs(depthFiniteDifference1) * 100, abs(depthFiniteDifference1) * 100, abs(depthFiniteDifference1) * 100, 1.0);
    //return vec4(depth0, depth0, depth0, 1.0);

    //let d = prepass_normal(in.position.xy);
    //return vec4(d.r, d.g, d.b, 1.0);

    //let offset_strength = settings.intensity;
    //let c = textureSample(screen_texture, texture_sampler, in.uv).rgb;
    //let i = length(c);
    //let new_i = floor(i * 15.0) / 15.0;
    //let new_c = normalize(c) * new_i;
    //// Sample each color channel with an arbitrary shift
    //return vec4<f32>(
    //    new_c.r,
    //    new_c.g,
    //    new_c.b,
    //    1.0
    //);
}
