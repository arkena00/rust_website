#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

@group(2) @binding(200) var can_fingerprint: texture_2d<f32>;
@group(2) @binding(201) var can_fingerprint_sampler: sampler;
@group(2) @binding(202) var can_texture0: texture_2d<f32>;
@group(2) @binding(203) var can_texture0_sampler: sampler;
@group(2) @binding(204) var can_texture1: texture_2d<f32>;
@group(2) @binding(205) var can_texture1_sampler: sampler;

struct Parameters {
    scroll: f32,
    page: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec2<f32>
#endif
}
@group(2) @binding(100) var<uniform> params: Parameters;

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // we can optionally modify the input before lighting and alpha_discard is applied
    //pbr_input.material.base_color.b = pbr_input.material.base_color.r;

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    var fingerprint = textureSample(can_fingerprint, can_fingerprint_sampler, in.uv);
    out.color = mix(out.color , fingerprint, 0.05);

    return out;
}