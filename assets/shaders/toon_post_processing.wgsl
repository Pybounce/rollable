#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

#import bevy_pbr::{
    mesh_view_bindings::globals,
    prepass_utils,
    forward_io::VertexOutput,
}
#import bevy_render::view::View
#import bevy_pbr::view_transformations::uv_to_ndc;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct ToonPostProcessSettings {
    depth_threshold: f32,
    depth_threshold_depth_mul: f32,  // If something is further away, it should require more depth
    depth_normal_threshold: f32, // If at a glazing angle, depth threshold should be harsher
    depth_normal_threshold_mul: f32, // If at a glazing angle, depth threshold should be harsher
    normal_threshold: f32,
    colour_threshold: f32,
    sampling_scale: f32,

}
@group(0) @binding(2) var<uniform> settings: ToonPostProcessSettings;
@group(0) @binding(3) var depth_prepass_texture: texture_depth_2d;
@group(0) @binding(4) var normal_prepass_texture: texture_2d<f32>;
@group(0) @binding(5) var<uniform> view: View;


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
    
    let _edge_depth_threshold = settings.depth_threshold;
    
    let depth0 = prepass_depth(uv_to_pos(bl_uv));
    let depth1 = prepass_depth(uv_to_pos(tr_uv));
    let depth2 = prepass_depth(uv_to_pos(br_uv));
    let depth3 = prepass_depth(uv_to_pos(tl_uv));

    let depth_finite_diff_0 = depth1 - depth0;
    let depth_finite_diff_1 = depth3 - depth2;

    let depth_threshold = _edge_depth_threshold * (depth0 * settings.depth_threshold_depth_mul) * normal_threshold;

    var edge_depth = sqrt(pow(depth_finite_diff_0, 2.0) + pow(depth_finite_diff_1, 2.0)) * 100.0;

    if edge_depth > depth_threshold { edge_depth = 1.0; }
    else { edge_depth = 0.0; }

    return edge_depth;
}

fn normal_buffer_edge_depth(bl_uv: vec2f, tr_uv: vec2f, br_uv: vec2f, tl_uv: vec2f) -> f32 {
    let _normal_threshold = settings.normal_threshold;

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

fn detect_edge_colour(bl_uv: vec2f, tr_uv: vec2f, br_uv: vec2f, tl_uv: vec2f) -> f32 {
    let _colour_threshold = settings.colour_threshold;

    let c0 = textureSample(screen_texture, texture_sampler, bl_uv).rgb;
    let c1 = textureSample(screen_texture, texture_sampler, tr_uv).rgb;
    let c2 = textureSample(screen_texture, texture_sampler, br_uv).rgb;
    let c3 = textureSample(screen_texture, texture_sampler, tl_uv).rgb;

    let finite_diff_0 = c1 - c0;
    let finite_diff_1 = c3 - c2;

    var edge = sqrt(dot(finite_diff_0, finite_diff_0) + dot(finite_diff_1, finite_diff_1));
    if edge > _colour_threshold { edge = 1.0; }
    else { edge = 0.0; }

    return edge;
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

fn get_sampling_scale(pos: vec2f) -> f32 {
    let depth = 1.0 - prepass_depth(pos);
    //if depth > 0.999 { return 1.0; }
    //if depth > 0.998 { return 2.0; }
    return mix(settings.sampling_scale, settings.sampling_scale, depth);
    //return 0.1;
}

fn position_ndc_to_world(ndc_pos: vec2<f32>, depth: f32) -> vec3<f32> {
    let world_pos = view.world_from_clip * vec4(ndc_pos, depth, 1.0);
    return world_pos.xyz / world_pos.w;
}

fn worldspace_camera_view_direction(uv: vec2f) -> vec3f {
    let ndc = uv_to_ndc(uv);
    let ray_point = position_ndc_to_world(ndc, prepass_depth(uv_to_pos(uv)));
    let worldspace_view_dir = normalize(ray_point - view.world_position).xyz;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    let _scale = get_sampling_scale(in.position.xy);
    let texel_size = texel_size();

    let half_scale_floor = floor(_scale * 0.5);
    let half_scale_ceil = ceil(_scale * 0.5);

    let bl_uv = in.uv - vec2f(texel_size.x, texel_size.y) * half_scale_floor;
    let tr_uv = in.uv + vec2f(texel_size.x, texel_size.y) * half_scale_ceil;  
    let br_uv = in.uv + vec2f(texel_size.x * half_scale_ceil, -texel_size.y * half_scale_floor);
    let tl_uv = in.uv + vec2f(-texel_size.x * half_scale_floor, texel_size.y * half_scale_ceil);

    //who the fuck knows

    let cam_view_dir = worldspace_camera_view_direction(in.uv);
    let normal0 = prepass_normal(uv_to_pos(in.uv)).rgb;
    let view_normal = normal0 * 2 - 1;
    let NdotV = 1 - dot(view_normal, -cam_view_dir);

    let _depth_normal_threshold = settings.depth_normal_threshold;
    let _depth_normal_threshold_scale = settings.depth_normal_threshold_mul;
    
    let normal_threshold0 = saturate((NdotV - _depth_normal_threshold) / (1.0 - _depth_normal_threshold));
    let normal_threshold = normal_threshold0 * _depth_normal_threshold_scale + 1;

    ////////

    let edge_depth_0 = depth_buffer_edge_depth(normal_threshold, bl_uv, tr_uv, br_uv, tl_uv);
    let edge_depth_1 = normal_buffer_edge_depth(bl_uv, tr_uv, br_uv, tl_uv);
    let colour_depth = detect_edge_colour(bl_uv, tr_uv, br_uv, tl_uv);
    let edge_depth = max(colour_depth, max(edge_depth_0, edge_depth_1));
    

    var c = toon_colour(in.uv);

    if edge_depth > 0.5 {
        c = vec4(0.1, 0.1, 0.1, 1.0);
    }
    
    return vec4f(c);
}
