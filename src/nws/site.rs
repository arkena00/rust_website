use bevy::math::Vec2;
use derivative::Derivative;
use bevy::prelude::{Resource, Transform};

#[derive(Resource, Derivative)]
#[derivative(Default)]
pub struct Camera
{
    #[derivative(Default(value = "Transform::from_xyz(0.0, 0.0, 1400.0)"))]
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
    pub prev_page_index: u8,
    pub page_index: u8,
    pub mouse: Vec2,
    pub window_size: Vec2,
}

impl Default for Site {
    fn default() -> Self {
        Site {
            camera: Camera::default(),
            mouse: Vec2::ZERO,
            state: State::default(),
            scroll: Scroll::default(),
            prev_page_index: 0,
            page_index: 0,
            window_size: Vec2::new(1920., 1080.),
        }
    }
}
