use derivative::Derivative;
use bevy::prelude::{Resource, Transform};

#[derive(Resource, Derivative)]
#[derivative(Default)]
pub struct Camera
{
    #[derivative(Default(value = "Transform::from_xyz(0.0, 0.0, 5000.0)"))]
    pub transform: Transform,
}

#[derive(Resource, Derivative)]
#[derivative(Default)]
pub struct Scroll
{
    #[derivative(Default(value = "100.0"))]
    pub step: f32,
    #[derivative(Default(value = "0.0"))]
    pub percent: f32, // range: 0 (top) to 1 (bottom)
    #[derivative(Default(value = "0.0"))]
    pub value: f32,
    #[derivative(Default(value = "5630.0"))]
    pub max_value: f32,
}


pub enum State
{
    Loading,
    Intro,
    Normal,
}
impl Default for State {
    fn default() -> Self { State::Loading }
}

#[derive(Resource)]
pub struct Site
{
    pub camera: Camera,
    pub state: State,
    pub scroll: Scroll,
    pub page_index: u8,
}

impl Default for Site {
    fn default() -> Self {
        Site {
            camera: Camera::default(),
            state: State::default(),
            scroll: Scroll::default(),
            page_index: 0,
        }
    }
}
