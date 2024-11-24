#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals

// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;

@group(2) @binding(3) var image_texture: texture_2d<f32>;
@group(2) @binding(4) var image_sampler: sampler;

struct Uniforms {
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32>
{
    var out = vec4<f32>();
    var anim_uv = vec2<f32>(mesh.uv) + globals.time * 0.1;

    var image = textureSample(image_texture, image_sampler, anim_uv);
    var image2 = textureSample(image_texture, image_sampler, vec2<f32>(mesh.uv) + globals.time * -0.05);

    var mask = textureSample(base_color_texture, base_color_sampler, mesh.uv);


    //out = (1 - mask.r) * (1 - mask.g) * (1 - mask.b) * base_color;
    //out = saturate(base_color - mask.r - mask.g - mask.b);

    //out += image * mask.g;
    //out += image2 * mask.r;
    out = image2 * mask.r;

    return mask;
}
