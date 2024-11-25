use bevy::math::Vec2;
use bevy::prelude::Component;
use derivative::Derivative;

#[derive(Component, Derivative)]
pub struct Rotatable {
    #[derivative(Default(value = "0.05"))]
    pub speed: f32,
}

#[derive(Component, Default)]
pub struct SiteCamera
{
    pub position: f32,
    pub target_position: f32,
}
#[derive(Component, Default)]
pub struct UIText
{
    pub fixed_pos: Vec2,
    pub pos: Vec2,
    pub target_pos: Vec2,
}