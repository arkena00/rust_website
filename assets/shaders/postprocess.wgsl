#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    scroll: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength
    let scroll = settings.scroll;

/*    var f = in.uv;
    var o = textureSample(screen_texture, texture_sampler, in.uv);
    f.y += o.r / o.g + scroll * .5;
    o -= o - textureSample(screen_texture, texture_sampler, f);

    return o;*/

return vec4<f32>();
    //return vec4<f32>(offset_strength, offset_strength, offset_strength, 1.0) * textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r;

}
