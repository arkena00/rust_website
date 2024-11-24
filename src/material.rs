use bevy::asset::{Asset, Handle};
use bevy::color::LinearRgba;
use bevy::math::Vec2;
use bevy::pbr::{ExtendedMaterial, Material, StandardMaterial};
use bevy::prelude::{Component, Image, TypePath};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};


#[derive(Component, Asset, TypePath, AsBindGroup, Default, Clone)]
pub struct MoonMaterial {
    #[uniform(100)]
    pub mouse: Vec2,

}
#[derive(Component)]
pub struct MoonComponent
{
    pub material: Handle<MoonMaterial>
}

impl Material for MoonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/moon.wgsl".into()
    }
}