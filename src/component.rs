use bevy::prelude::Component;
use derivative::Derivative;

#[derive(Component, Derivative)]
pub struct Rotatable {
    #[derivative(Default(value = "0.05"))]
    pub speed: f32,
}

#[derive(Component)]
pub struct SiteCamera;