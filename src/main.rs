mod background;
mod can;
mod component;
mod postprocess;

use std::f32::consts::TAU;
use std::time::Duration;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::WindowResized;
use background::*;
use postprocess::*;
use can::CanPlugin;
use component::*;

use bevy_kira_audio::prelude::*;


//static mut SCROLL: f32 = 0.0;

#[derive(Resource)]
struct BackgroundAudio;

#[derive(Resource)]
struct SiteRes
{
    scroll: f32
}


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
            }))
            .insert_resource(SiteRes{ scroll: 0. })
            .add_systems(Startup, (setup, start_background_audio))
            .add_systems(Update, (
                mouse_scroll
                , camera_move
                , debug_text
                , update_settings
            ))
            .add_plugins(BackgroundPlugin {})
            .add_plugins(AudioPlugin)

            .add_plugins(CanPlugin {})
            .add_plugins(PostProcessPlugin {})
            .add_event::<ScrollEvent>()
            .run();
}

#[derive(Component)]
struct ColorText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((SiteCamera, Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, PostProcessSettings {
    intensity: 0.02,
    ..default()
    }));





    // text
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "M E I S T",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/Inversionz.ttf"),
                font_size: 100.0,
                ..default()
            },
        ) // Set the justification of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            }),
        ColorText,
    ));


    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            ..default()
        },
        ..default()
    })
        .with_children(|root| {
            // Text where we display current resolution
            root.spawn((
                TextBundle::from_section(
                    "Resolution",
                    TextStyle {
                        font_size: 50.0,
                        ..default()
                    },
                ),
                DebugText,
            ));
        });

}




fn mouse_scroll(
    mut site: ResMut<SiteRes>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut scroll_event: EventWriter<ScrollEvent>,
) {
    for mouse_wheel_event in mouse_wheel_events.read()
    {
        let dy = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => mouse_wheel_event.y,
            MouseScrollUnit::Pixel => mouse_wheel_event.y / 100.,
        };
        site.scroll += dy;
        site.scroll = site.scroll.clamp(-500., 0.);
        scroll_event.send(ScrollEvent(site.scroll));
    }
}


fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/intro.ogg")).looped();
}

fn debug_text(
    site: ResMut<SiteRes>,
    mut q: Query<&mut Text, With<DebugText>>,
) {
    let mut text = q.single_mut();

    text.sections[0].value = format!("{:.1}", site.scroll);

}

#[derive(Component)]
struct DebugText;


fn camera_move(
    site: ResMut<SiteRes>,
    mut query: Query<(&SiteCamera, &mut Transform)>,
) {
    let (camera, mut transform) = query.single_mut();

    transform.translation.y = site.scroll;
}



fn update_settings(mut settings: Query<&mut PostProcessSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        let mut intensity = time.elapsed_seconds().sin();
        // Make it loop periodically
        intensity = intensity.sin();
        // Remap it to 0..1 because the intensity can't be negative
        intensity = intensity * 0.5 + 0.5;
        // Scale it to a more reasonable level
        intensity *= 0.015;

        // Set the intensity.
        // This will then be extracted to the render world and uploaded to the gpu automatically by the [`UniformComponentPlugin`]
        setting.intensity = intensity;
    }
}
