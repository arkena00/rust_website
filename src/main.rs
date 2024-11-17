mod background;
mod can;
mod component;

use std::time::Duration;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use background::*;
use can::CanPlugin;

use bevy_kira_audio::prelude::*;


//static mut SCROLL: f32 = 0.0;

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
            }))
            .add_systems(Startup, (setup, start_background_audio))
            .add_systems(Update, mouse_scroll)
            // .add_plugins(BackgroundPlugin {})
            .add_plugins(AudioPlugin)

            .add_plugins(CanPlugin {})
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });


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
}




fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut scroll_event: EventWriter<ScrollEvent>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {

        let dy = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => mouse_wheel_event.y,
            MouseScrollUnit::Pixel => mouse_wheel_event.y,
        };
        //SCROLL += dy;
        let v = dy;
        scroll_event.send(ScrollEvent(dy));
    }
}


fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/intro.ogg")).looped();
}