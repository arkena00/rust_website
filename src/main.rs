mod background;
mod can;
mod component;
mod postprocess;
mod nws;
mod material;

use background::*;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, SamplerDescriptor};
use bevy::render::texture::{ImageAddressMode, ImageSamplerDescriptor};
use can::CanPlugin;
use component::*;
use postprocess::*;
use bevy::color::palettes;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::window::{PrimaryWindow, WindowResized, WindowResolution};
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
                resolution:  (1920., 1080.).into(),
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
            update_site,
        ))
        .add_plugins(nws::content::ContentPlugin{})
        .add_plugins(EasingsPlugin)
        //.add_plugins(BackgroundPlugin{})
        .add_plugins(AudioPlugin)
        //.add_plugins(PostProcessPlugin{})
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
            transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    // camera
    commands.spawn((SiteCamera::default(), Camera3dBundle {
        tonemapping: Tonemapping::None,
        projection: PerspectiveProjection {
            fov: 30.0_f32.to_radians(),
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
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut site: ResMut<nws::site::Site>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut scroll_event: EventWriter<ScrollEvent>,
    mut query: Query<(&mut SiteCamera, &mut Transform)>,
) {
    let (mut camera, transform) = query.single_mut();
    camera.position = transform.translation.y;


    for mouse_wheel_event in mouse_wheel_events.read()
    {
        let dy = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => mouse_wheel_event.y,
            MouseScrollUnit::Pixel => mouse_wheel_event.y / 100.,
        } * site.scroll.step;

        site.scroll.value = (site.scroll.value + dy).clamp(-site.scroll.max_value, 0.);
        site.scroll.percent = site.scroll.value / -site.scroll.max_value;

        site.page_index = 0;
        let offset = 512.;
        if site.scroll.value < offset + -1024. { site.page_index = 1; }
        if site.scroll.value < offset + -1534. { site.page_index = 2; }
        if site.scroll.value < offset + -2558. { site.page_index = 3; }

        scroll_event.send(ScrollEvent(site.scroll.percent));

        if site.page_index == 3 && site.prev_page_index < site.page_index { audio.play(asset_server.load("sounds/fire.mp3")); }
        site.prev_page_index = site.page_index;


        camera.target_position = site.scroll.value;
    }
}

fn update_site(mut site: ResMut<nws::site::Site>,
               mut mouse_move_event: EventReader<MouseMotion>,
               mut resize_event: EventReader<WindowResized>,
               q_windows: Query<&Window, With<PrimaryWindow>>)
{
    for e in resize_event.read() {
        site.window_size.x = e.width;
        site.window_size.y = e.height;
    }

    for ev in mouse_move_event.read() {
        if let Some(position) = q_windows.single().cursor_position() {
            site.mouse = position;
        } else {
        }
    }
}


fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    //audio.play(asset_server.load("sounds/intro.mp3")).looped();
}



fn camera_move(
    site: ResMut<nws::site::Site>,
    mut query: Query<(&mut SiteCamera, &mut Transform)>,
) {
    let (mut camera, mut transform) = query.single_mut();

    transform.translation.y = camera.position.lerp(camera.target_position, 0.1);
}
