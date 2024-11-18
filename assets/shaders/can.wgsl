#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals

// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> scroll: f32;;
@group(2) @binding(2) var base_color_texture0: texture_2d<f32>;
@group(2) @binding(3) var base_color_sampler0: sampler;
@group(2) @binding(4) var base_color_texture1: texture_2d<f32>;
@group(2) @binding(5) var base_color_sampler1: sampler;

struct Uniforms {
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    var s = sin(globals.time * 2);



    var can0 = textureSample(base_color_texture0, base_color_sampler0, mesh.uv).rgba;
    var can1 = textureSample(base_color_texture1, base_color_sampler1, mesh.uv).rgba;

    var out = mix(can0, can1, s);

    return out;
}
