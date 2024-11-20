use bevy::{
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

#[derive(Component)]
pub struct CanEntity
{
    material: Handle<ExtendedMaterial<StandardMaterial, CanMaterial>>
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


#[derive(Component, Asset, TypePath, AsBindGroup, Clone)]
struct CanMaterial {
    #[uniform(100)]
    scroll: f32,
    #[uniform(101)]
    page: f32,
    #[texture(200)]
    #[sampler(201)]
    can_texture0: Option<Handle<Image>>,
    #[texture(202)]
    #[sampler(203)]
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
}

pub struct CanPlugin;

impl Plugin for CanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, CanMaterial>,
            >::default())
            //.add_plugins(UniformComponentPlugin::<TimeUniform>::default())
            .add_systems(Startup, (setup))
            .add_systems(Update, (
                update,
                update_shader,
                //update_settings,
                scroll_animate,
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
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    let can_mesh: Handle<Mesh> = asset_server.load("meshes/can.glb#Mesh0/Primitive0");
    let can_mesh2: Handle<Mesh> = asset_server.load("meshes/can.glb#Mesh0/Primitive0");

    let material_handle = materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: LinearRgba::WHITE.into(),
            base_color_texture: Some(asset_server.load("textures/can0.png")),
            ..Default::default()
        },
        extension: CanMaterial {
            scroll: 0.,
            page: 0.,
            can_texture0: Some(asset_server.load("textures/can0.png")),
            can_texture1: Some(asset_server.load("textures/can1.png")),
        },
    });

    commands.spawn((MaterialMeshBundle {
        mesh: can_mesh,
        material: material_handle.clone(),
        transform: Transform::from_xyz(-1240.0 / 2., 0.0, 100.0)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0., 0., 90_f32.to_radians()))
            .with_scale(Vec3::splat(100.)),
        ..default()
    }, CanEntity{
        material: material_handle,
    }
    ));

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
    mut query: Query<(&mut Transform, &CanEntity)>,
    mut scroll_event: EventReader<ScrollEvent>)
{
    let (mut transform, _) = query.single_mut();

    transform.translation.y = site.scroll.value;
    let can_rotation = transform.rotation.to_euler(EulerRot::XYZ);
    transform.rotation = Quat::from_euler(EulerRot::XYZ, site.scroll.value * -0.05, can_rotation.1, can_rotation.2);


    //transform.translation.y += site.scroll.value;

/*    for (mut transform, cube) in &mut cubes {
        transform.rotate_y(cube.speed * TAU * timer.delta_seconds());

        for ev in scroll_event.read() {
            transform.rotate_x(0.01 * TAU * ev.0);
        }
    }*/
}

fn scroll_animate(time: Res<Time>,
                  mut site: ResMut<nws::site::Site>,
                  mut query: Query<(&mut Transform, &Curve)>,
                  mut gizmos: Gizmos) {
    let t = (time.elapsed_seconds().sin() + 1.) / 2.;

    for (mut transform, cubic_curve) in &mut query {
        // Draw the curve
        gizmos.linestrip(cubic_curve.0.iter_positions(50), WHITE);
        // position takes a point from the curve where 0 is the initial point
        // and 1 is the last point

        //transform.translation = cubic_curve.0.position(site.scroll.step.abs() / 10.);
    }
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