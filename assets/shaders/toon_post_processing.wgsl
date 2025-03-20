#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength
    let offset_strength = settings.intensity;
    let c = textureSample(screen_texture, texture_sampler, in.uv).rgb;
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
