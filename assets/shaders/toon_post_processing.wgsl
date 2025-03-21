#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

#import bevy_pbr::{
    mesh_view_bindings::globals,
    prepass_utils,
    forward_io::VertexOutput,
}
#import bevy_render::view::View


@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;
@group(0) @binding(3) var depth_prepass_texture: texture_depth_2d;
@group(0) @binding(4) var normal_prepass_texture: texture_2d<f32>;
@group(0) @binding(5) var<uniform> view: View;


fn view_space_dir(uv: vec2f) -> vec3f {
    return (view.view_from_clip * vec4f(uv_to_pos(uv), 0.0, 1.0)).xyz;
}
fn depth_ndc_to_view_z(ndc_depth: f32) -> f32 {
#ifdef VIEW_PROJECTION_PERSPECTIVE
    return perspective_camera_near() / ndc_depth;
#else ifdef VIEW_PROJECTION_ORTHOGRAPHIC
    return -(view.projection[3][2] - ndc_depth) / view.projection[2][2];
#else
    let view_pos = view.view_from_clip * vec4(0.0, 0.0, ndc_depth, 1.0);
    return view_pos.z / view_pos.w;
#endif
}

fn depth_ndc_to_view(ndc_depth: f32) -> vec3f {
    let view_pos = view.view_from_clip * vec4(0.0, 0.0, ndc_depth, 1.0);
    return (view_pos / view_pos.w).xyz;
}

fn uv_to_ndc(uv: vec2f) -> vec2f {
    return uv * vec2f(2.0, -2.0) + vec2f(-1.0, 1.0);
}

fn position_ndc_to_view(ndc_pos: vec3f) -> vec3f {
    let view_pos = view.view_from_clip * vec4f(ndc_pos, 1.0);
    return view_pos.xyz / view_pos.w;
}

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

fn depth_buffer_edge_depth(normal_threshold: f32, bl_uv: vec2f, tr_uv: vec2f, br_uv: vec2f, tl_uv: vec2f) -> f32 {
    
    let _edge_depth_threshold = 1.5;
    
    let depth0 = prepass_depth(uv_to_pos(bl_uv));
    let depth1 = prepass_depth(uv_to_pos(tr_uv));
    let depth2 = prepass_depth(uv_to_pos(br_uv));
    let depth3 = prepass_depth(uv_to_pos(tl_uv));

    let depth_finite_diff_0 = depth1 - depth0;
    let depth_finite_diff_1 = depth3 - depth2;

    let depth_threshold = _edge_depth_threshold * depth0 * normal_threshold;

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
    return vec4<f32>(
        new_c.r,
        new_c.g,
        new_c.b,
        1.0
    );
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    let view_space_dir = (view.view_from_clip * vec4f(in.position.xy, 0.0, 1.0)).xyz;

    let _scale = 3.0;
    let texel_size = texel_size();

    let half_scale_floor = floor(_scale * 0.5);
    let half_scale_ceil = ceil(_scale * 0.5);

    let bl_uv = in.uv - vec2f(texel_size.x, texel_size.y) * half_scale_floor;
    let tr_uv = in.uv + vec2f(texel_size.x, texel_size.y) * half_scale_ceil;  
    let br_uv = in.uv + vec2f(texel_size.x * half_scale_ceil, -texel_size.y * half_scale_floor);
    let tl_uv = in.uv + vec2f(-texel_size.x * half_scale_floor, texel_size.y * half_scale_ceil);

    //who the fuck knows
    let normal0 = prepass_normal(uv_to_pos(bl_uv)).rgb;
    let view_normal = normal0 * 2 - 1;
    let NdotV = 1 - dot(view_normal, -view_space_dir);

    let _depth_normal_threshold = 0.3;
    let _depth_normal_threshold_scale = 7.0;
    
    let normal_threshold0 = saturate((NdotV - _depth_normal_threshold) / (1.0 - _depth_normal_threshold));
    let normal_threshold = normal_threshold0 * _depth_normal_threshold_scale + 1;

    ////////

    let edge_depth_0 = depth_buffer_edge_depth(normal_threshold, bl_uv, tr_uv, br_uv, tl_uv);
    let edge_depth_1 = normal_buffer_edge_depth(bl_uv, tr_uv, br_uv, tl_uv);
    let edge_depth = max(edge_depth_0, edge_depth_1);
    

    var c = toon_colour(in.uv);

    if edge_depth > 0.5 {
        c = vec4(edge_depth, edge_depth, edge_depth, 1.0);
    }

    return vec4f(c);
}
