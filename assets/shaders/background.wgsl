#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals

// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;

struct Uniforms {
    mouse: vec2<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

fn opSmoothUnion(d1: f32, d2: f32, k: f32) -> f32 {
	var h: f32 = clamp(0.5 + 0.5 * (d2 - d1) / k, 0., 1.);
	return mix(d2, d1, h) - k * h * (1. - h);
}

fn sdSphere(p: vec3<f32>, s: f32) -> f32 {
	return length(p) - s;
}



@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    var frag_color = vec4<f32> (sin(globals.time), 0, 1, 1);

    return frag_color;

    //return material_color * textureSample(base_color_texture, base_color_sampler, mesh.uv);

    //return fragColor;
}
