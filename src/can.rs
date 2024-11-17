use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, ShaderRef,
        },
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use std::f32::consts::TAU;
use crate::component::Rotatable;
use crate::ScrollEvent;

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
            .add_systems(Update, update);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CanMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let can_mesh: Handle<Mesh> = asset_server.load("meshes/can.glb#Mesh0/Primitive0");

    commands.spawn((MaterialMeshBundle {
        mesh: can_mesh,
        material: materials.add(CanMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("image.png")),
        }),

        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },Rotatable { speed: 0.3 }));
}


fn update(mut cubes: Query<(&mut Transform, &Rotatable)>
          , mut scroll_event: EventReader<ScrollEvent>
          , timer: Res<Time>)
{
    for (mut transform, cube) in &mut cubes {
        // The speed is first multiplied by TAU which is a full rotation (360deg) in radians,
        // and then multiplied by delta_seconds which is the time that passed last frame.
        // In other words. Speed is equal to the amount of rotations per second.
        //transform.rotate_x(0.1 * TAU * timer.delta_seconds());
        transform.rotate_y(cube.speed * TAU * timer.delta_seconds());

        for ev in scroll_event.read() {
            transform.rotate_x(0.01 * TAU * ev.0);
        }
    }


}
