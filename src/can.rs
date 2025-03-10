﻿use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        render_resource::{
            AsBindGroup, ShaderRef,
        },
    },
};

use bevy::{
    animation::{animate_targets, RepeatAnimation},
    pbr::CascadeShadowConfigBuilder,
};

use std::f32::consts::TAU;
use std::ops::Add;
use std::time::Duration;
use bevy::color::Color::Srgba;
use bevy::color::palettes::basic::WHITE;
use bevy::math::vec3;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin};
use bevy::render::render_resource::ShaderType;
use crate::component::{Rotatable};
use crate::{nws, ScrollEvent};

use bevy::prelude::*;
use bevy::render::render_resource::{Shader};
use bevy::prelude::*;
use bevy_easings::*;

#[derive(Component, Default)]
pub struct CanEntity
{
    material: Handle<ExtendedMaterial<StandardMaterial, CanMaterial>>,
    position: f32,
    target_position: f32,
    rotation: Quat,
    target_rotation: Quat,
}

#[derive(Component)]
struct Curve(CubicCurve<Vec3>);

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct TimeUniform {
    intensity: f32,
    // WebGL2 structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    _webgl2_padding: Vec3,
}


#[derive(Component, Default, Asset, TypePath, AsBindGroup, Clone)]
struct CanMaterial {
    #[uniform(100)]
    scroll: f32,
    page: f32,
    _webgl2_padding: Vec2,
    #[texture(200)]
    #[sampler(201)]
    can_fingerpint: Option<Handle<Image>>,
    #[texture(202)]
    #[sampler(203)]
    can_texture0: Option<Handle<Image>>,
    #[texture(204)]
    #[sampler(205)]
    can_texture1: Option<Handle<Image>>,
}

#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}


impl MaterialExtension for CanMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/can.wgsl".into()
    }
    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/can.wgsl".into()
    }
}

pub struct CanPlugin;

impl Plugin for CanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, CanMaterial>>::default())
            //.add_plugins(UniformComponentPlugin::<TimeUniform>::default())
            .add_systems(Startup, (setup))
            .add_systems(Update, smooth_update)
            .add_systems(Update, (
                update,
                update_shader,
                //update_settings,
                /*setup_animation.before(animate_targets)*/));
    }
}

/*#[derive(Default, Component)]
struct LerpComponent(f32);
impl Lerp for LerpComponent {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        LerpComponent(interpolation::lerp(&self.0, &other.0, scalar))
    }
}*/

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CanMaterial>>>,
    mut stdmaterials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
/*    commands.spawn(SceneBundle {
        transform: Transform::from_xyz(-1024.0 / 2., 0.0, 400.0).with_scale(Vec3::splat(100.)),
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("meshes/can.glb")),
        ..default()
    });*/



        let can_mesh0: Handle<Mesh> = asset_server.load("meshes/can.glb#Mesh0/Primitive0");
        let can_mesh1: Handle<Mesh> = asset_server.load("meshes/can.glb#Mesh0/Primitive1");

        let material_handle = materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: LinearRgba::WHITE.into(),
                base_color_texture: Some(asset_server.load("textures/can0.png")),
                metallic: 0.8,
                metallic_roughness_texture: Some(asset_server.load("textures/can_metal_specular.png")),
                ior: 1450.,
                ..Default::default()
            },
            extension: CanMaterial {
                scroll: 0.,
                page: 0.,
                can_fingerpint: Some(asset_server.load("textures/can_finger.png")),
                can_texture0: Some(asset_server.load("textures/can0.png")),
                can_texture1: Some(asset_server.load("textures/can1.png")),
                ..default()
            },
        });




    commands.spawn((MaterialMeshBundle {
            mesh: can_mesh0,
            material: material_handle.clone(),
            transform: Transform::from_xyz(-256.0 / 2., 0.0, 200.0)
                .with_scale(Vec3::splat(120.)),
            ..default()
        }, CanEntity{
            material: material_handle.clone(), ..default()
        }
        )).with_children(|parent| {
            parent.spawn((MaterialMeshBundle {
                mesh: can_mesh1,
                material: stdmaterials.add(StandardMaterial{
                    base_color: bevy::color::Srgba::hex("B3B3B3").unwrap().into(),
                    metallic: 0.721,
                    ..default()
                }),
                ..default()
            }));
        });



    let test: Handle<Scene> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("animations/can.glb"));
    /*
    let animation_intro = [[
        vec3(-6., 2., 0.),
        vec3(12., 8., 0.),
        vec3(-12., 8., 0.),
        vec3(6., 2., 0.),

    ]];

    let bezier = CubicBezier::new(animation_intro).to_curve();

    commands.spawn((MaterialMeshBundle {
        mesh: can_mesh,
        material: materials.add(CanMaterial {
            color: LinearRgba::WHITE,
            scroll: 1.0,
            color_texture0: Some(asset_server.load("textures/can0.png")),
            color_texture1: Some(asset_server.load("textures/can1.png")),
        }),

        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }, Rotatable { speed: 0.3 }
                    , Curve(bezier)
    ));

    let clip = asset_server.load( GltfAssetLabel::Animation(0).from_asset("animations/can.glb") );

    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [
                clip,
            ]
                .into_iter(),
            1.0,
            graph.root,
        )
        .collect();

    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("animations/can.glb")),
        ..default()
    });
*/


}

fn update(
    site: ResMut<nws::site::Site>,
    timer: Res<Time>,
    mut query: Query<(&mut Transform, &mut CanEntity)>,
    mut scroll_event: EventReader<ScrollEvent>)
{
    let (mut transform, mut can) = query.single_mut();

    can.position = transform.translation.y;
    can.target_position = site.scroll.value;

    transform.rotation = Quat::from_euler(EulerRot::XYZ, transform.translation.y * -0.01, 0., -90_f32.to_radians());
}


fn setup_animation(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}

fn update_shader(
    site: ResMut<nws::site::Site>,
    timer: Res<Time>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CanMaterial>>>,
    mut query: Query<&mut CanEntity>)
{
    let mut can_entity = query.single_mut();
    if let Some(material) = materials.get_mut(&can_entity.material) {
        material.extension.scroll = site.scroll.percent;
        material.extension.page = site.page_index.into();
    }

    //materials.get_mut(&can_entity.material).unwrap().extension.scroll = site.scroll.value;
}


fn smooth_update(
    timer: Res<Time>,
    mut query: Query<(&mut Transform, &CanEntity)>,
) {
    for (mut transform, can) in &mut query {
        let speed = 0.2;
        let ease = simple_easing::circ_in(speed);
        transform.translation.y = can.position.lerp(can.target_position, ease);

        transform.rotation = can.rotation.lerp(can.target_rotation, ease);
    }
}