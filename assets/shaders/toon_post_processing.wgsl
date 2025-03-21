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
@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    
    

    let d = prepass_normal(in.position.xy);
    return vec4(d.r, d.g, d.b, 1.0);

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
