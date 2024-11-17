use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        render_resource::{
            AsBindGroup, ShaderRef,
        },
    },
};

use std::f32::consts::TAU;
use bevy::color::palettes::basic::WHITE;
use bevy::math::vec3;
use crate::component::{Rotatable};
use crate::{ScrollEvent, SiteRes};

#[derive(Component)]
struct Curve(CubicCurve<Vec3>);

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CanMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material for CanMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/can.wgsl".into()
    }
}

pub struct CanPlugin;

impl Plugin for CanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<CanMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (update, animate));
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<CanMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let can_mesh: Handle<Mesh> = asset_server.load("meshes/can.glb#Mesh0/Primitive0");

    let animation_intro = [[
        vec3(-6., 2., 0.),
        vec3(12., 8., 0.),
        vec3(-12., 8., 0.),
        vec3(6., 2., 0.),

    ]];

    // Make a CubicCurve
    let bezier = CubicBezier::new(animation_intro).to_curve();

    commands.spawn((MaterialMeshBundle {
        mesh: can_mesh,
        material: materials.add(CanMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("image.png")),
        }),

        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }, Rotatable { speed: 0.3 }
        , Curve(bezier)
    ));
}

fn update(mut cubes: Query<(&mut Transform, &Rotatable)>
          , mut scroll_event: EventReader<ScrollEvent>
          , timer: Res<Time>)
{
    for (mut transform, cube) in &mut cubes {
        transform.rotate_y(cube.speed * TAU * timer.delta_seconds());

        for ev in scroll_event.read() {
            transform.rotate_x(0.01 * TAU * ev.0);
        }
    }
}

fn animate(time: Res<Time>,
           mut site: ResMut<SiteRes>,
           mut query: Query<(&mut Transform, &Curve)>,
           mut gizmos: Gizmos) {
    let t = (time.elapsed_seconds().sin() + 1.) / 2.;

    for (mut transform, cubic_curve) in &mut query {
        // Draw the curve
        gizmos.linestrip(cubic_curve.0.iter_positions(50), WHITE);
        // position takes a point from the curve where 0 is the initial point
        // and 1 is the last point

        transform.translation = cubic_curve.0.position(site.scroll.abs() / 10.);
    }
}

