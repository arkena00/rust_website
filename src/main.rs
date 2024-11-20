mod background;
mod can;
mod component;
mod postprocess;
mod nws;

use background::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, SamplerDescriptor};
use bevy::render::texture::{ImageAddressMode, ImageSamplerDescriptor};
use can::CanPlugin;
use component::*;
use postprocess::*;
use bevy::color::palettes;
use bevy_easings::EasingsPlugin;
use bevy_mod_billboard::prelude::*;

use bevy_kira_audio::prelude::*;

#[derive(Resource)]
struct BackgroundAudio;

#[derive(Event)]
struct ScrollEvent(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }).set(
            ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..Default::default()
                },
            }
        ))
        .insert_resource(nws::site::Site::default())
        .add_systems(Startup, (setup, start_background_audio))
        .add_systems(Update, (
            mouse_scroll,
            camera_move,
        ))
        .add_plugins(nws::content::ContentPlugin{})
        .add_plugins(EasingsPlugin)
        .add_plugins(BackgroundPlugin{})
        .add_plugins(AudioPlugin)
        .add_plugins(PostProcessPlugin{})
        .add_event::<ScrollEvent>()
        .add_plugins(CanPlugin{})
        .run();
}



fn setup(
    site: ResMut<nws::site::Site>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    // camera
    commands.spawn((SiteCamera, Camera3dBundle {
        projection: PerspectiveProjection {
            fov: 10.0_f32.to_radians(),
            ..default()
        }.into(),
        transform: site.camera.transform.looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, PostProcessSettings {
        scroll: 0.1,
        ..default()
    }));
}


fn mouse_scroll(
    mut site: ResMut<nws::site::Site>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut scroll_event: EventWriter<ScrollEvent>,
) {
    for mouse_wheel_event in mouse_wheel_events.read()
    {
        let dy = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => mouse_wheel_event.y,
            MouseScrollUnit::Pixel => mouse_wheel_event.y / 100.,
        } * site.scroll.step;

        site.scroll.value = (site.scroll.value + dy).clamp(-site.scroll.max_value, 0.);
        site.scroll.percent = site.scroll.value / -site.scroll.max_value;

        site.page_index = 0;
        if site.scroll.percent > 0.355239779 {
            site.page_index = 1;
        }
        scroll_event.send(ScrollEvent(site.scroll.percent));
    }
}


fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    //audio.play(asset_server.load("sounds/intro.mp3")).looped();
}



fn camera_move(
    site: ResMut<nws::site::Site>,
    mut query: Query<(&SiteCamera, &mut Transform)>,
) {
    let (camera, mut transform) = query.single_mut();

    transform.translation.y = site.scroll.value;
}

