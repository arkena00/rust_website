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

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    var frag_color = vec4<f32> (sin(globals.time * 2), 0, 1, 1);

    return material_color * textureSample(base_color_texture, base_color_sampler, mesh.uv * 1.2);
}
