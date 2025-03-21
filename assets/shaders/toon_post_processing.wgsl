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

fn depth_buffer_edge_depth(bl_uv: vec2f, tr_uv: vec2f, br_uv: vec2f, tl_uv: vec2f) -> f32 {
    
    let _edge_depth_threshold = 1.5;
    
    let depth0 = prepass_depth(uv_to_pos(bl_uv));
    let depth1 = prepass_depth(uv_to_pos(tr_uv));
    let depth2 = prepass_depth(uv_to_pos(br_uv));
    let depth3 = prepass_depth(uv_to_pos(tl_uv));

    let depth_finite_diff_0 = depth1 - depth0;
    let depth_finite_diff_1 = depth3 - depth2;

    let depth_threshold = _edge_depth_threshold * depth0;

    var edge_depth = sqrt(pow(depth_finite_diff_0, 2.0) + pow(depth_finite_diff_1, 2.0)) * 100.0;
    if edge_depth > depth_threshold { edge_depth = 1.0; }
    else { edge_depth = 0.0; }

    return edge_depth;
}

fn normal_buffer_edge_depth(bl_uv: vec2f, tr_uv: vec2f, br_uv: vec2f, tl_uv: vec2f) -> f32 {
    let _normal_threshold = 0.4;

    let normal0 = prepass_normal(uv_to_pos(bl_uv)).rgb;
    let normal1 = prepass_normal(uv_to_pos(tr_uv)).rgb;
    let normal2 = prepass_normal(uv_to_pos(br_uv)).rgb;
    let normal3 = prepass_normal(uv_to_pos(tl_uv)).rgb;

    let normal_finite_diff_0 = normal1 - normal0;
    let normal_finite_diff_1 = normal3 - normal2;

    var edge_normal = sqrt(dot(normal_finite_diff_0, normal_finite_diff_0) + dot(normal_finite_diff_1, normal_finite_diff_1));
    if edge_normal > _normal_threshold { edge_normal = 1.0; }
    else { edge_normal = 0.0; }

    return edge_normal;
}


fn toon_colour(uv: vec2f) -> vec4f {
    let c = textureSample(screen_texture, texture_sampler, uv).rgb;
    let i = length(c);
    let new_i = floor(i * 15.0) / 15.0;
    let new_c = normalize(c) * new_i;
    // Sample each color channel with an arbitrary shift
    return vec4<f32>(
        new_c.r,
        new_c.g,
        new_c.b,
        1.0
    );
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    
    let _scale = 1.0;
    let texel_size = texel_size();

    let half_scale_floor = floor(_scale * 0.5);
    let half_scale_ceil = ceil(_scale * 0.5);

    let bl_uv = in.uv - vec2f(texel_size.x, texel_size.y) * half_scale_floor;
    let tr_uv = in.uv + vec2f(texel_size.x, texel_size.y) * half_scale_ceil;  
    let br_uv = in.uv + vec2f(texel_size.x * half_scale_ceil, -texel_size.y * half_scale_floor);
    let tl_uv = in.uv + vec2f(-texel_size.x * half_scale_floor, texel_size.y * half_scale_ceil);

    let edge_depth_0 = depth_buffer_edge_depth(bl_uv, tr_uv, br_uv, tl_uv);
    let edge_depth_1 = normal_buffer_edge_depth(bl_uv, tr_uv, br_uv, tl_uv);
    let edge_depth = max(edge_depth_0, edge_depth_1);
    

    var c = toon_colour(in.uv);

    if edge_depth > 0.5 {
        c = vec4(edge_depth, edge_depth, edge_depth, 1.0);
    }

    return c;
}
