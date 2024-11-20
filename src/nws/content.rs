use std::cell::Cell;
use std::path::Path;
use bevy::color::palettes;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::text::Text2dBounds;
use bevy_mod_billboard::BillboardTextBundle;
use bevy_mod_billboard::prelude::BillboardPlugin;
use bevy_mod_billboard::text::BillboardTextBounds;
use crate::nws;

pub struct ContentPlugin;
impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<FrameMaterial>::default())
            .add_plugins(BillboardPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, update_text);
    }
}

#[derive(Component)]
struct TitleText;


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct FrameMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    image_texture: Option<Handle<Image>>,
}

impl Material for FrameMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/frame.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<FrameMaterial>>,
    mut stdmaterials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // frames
    let width = 1440.;
    let initial_offset = 512.;
    let mut last_position_y = initial_offset;

    let mut add_frame = |cmds: &mut Commands, background_color: Srgba, height, background_image, image|{
        cmds.spawn(MaterialMeshBundle {
            mesh: meshes.add(Rectangle::from_size(Vec2::new(width, height))),
            transform: Transform::from_xyz(0., last_position_y - height / 2., 0.0),
            material: materials.add(FrameMaterial {
                color: background_color.into(),
                color_texture: Some(asset_server.load(background_image)),
                image_texture: Some(asset_server.load(image)),
            }),
            ..default()
        });
        last_position_y -= height;
    };

    // text
    let mut last_txtposition_y = initial_offset;
    let mut add_text = |cmds: &mut Commands, text: &str, background_color: Srgba, fsize: f32, size: Vec2, offset: f32, mut transform: Transform|
    {
        let mut rel_transform = transform;
        rel_transform.translation.x += size.x / 2. - width / 2.;
        //rel_transform.translation.y += transform.translation.y -offset + last_txtposition_y - size.y / 2.;

        //rel_transform.translation.x = -transform.translation.x - size.x / 2. - width / 2.;
        rel_transform.translation.y = -transform.translation.y - offset + last_txtposition_y - size.y / 2.;

        cmds.spawn(BillboardTextBundle {
            text_bounds: BillboardTextBounds( Text2dBounds{ size }  ),
            transform: rel_transform,
            text: Text::from_sections([
                TextSection {
                    value: text.to_string(),
                    style: TextStyle {
                        font_size: fsize,
                        font: asset_server.load("fonts/DrukTextWide-Bold.ttf"),
                        color: background_color.into(),
                    }
                },
            ]),
            ..default()
        });
        last_txtposition_y -= offset;
    };


    // let frame = nws::content::Frame::new();
    add_frame(&mut commands, Srgba::WHITE, 1024., Path::new("textures/background0.png"), Path::new("textures/image.png"));
    add_frame(&mut commands, Srgba::hex("121316").unwrap(), 1534., Path::new("textures/background1.png"), Path::new("textures/blank.png"));
    add_frame(&mut commands, Srgba::hex("F4CC81").unwrap(), 1024., Path::new("textures/background2.png"), Path::new("textures/image.png"));
    add_frame(&mut commands, Srgba::hex("9FAAC3").unwrap(), 1024., Path::new("textures/background3.png"), Path::new("textures/blank.png"));
    add_frame(&mut commands, Srgba::hex("81A3F4").unwrap(), 1024., Path::new("textures/background4.png"), Path::new("textures/blank.png"));
    add_frame(&mut commands, Srgba::hex("121316").unwrap(), 663., Path::new("textures/blank.png"), Path::new("textures/blank.png"));



    add_text(&mut commands, "DES BULLES DE BONHEUR", Srgba::hex("F4CC81").unwrap(), 64., Vec2::new(621., 112.), 1024., Transform::from_xyz(742., 197., 1.));
    add_text(&mut commands, "SOLAR BURN", Srgba::hex("121316").unwrap(), 96., Vec2::new(471., 192.), 1534., Transform::from_xyz(750., 161., 1.));
    add_text(&mut commands, "MOON DROP", Srgba::hex("121316").unwrap(), 96., Vec2::new(471., 192.), 1024., Transform::from_xyz(750., 161., 1.));

}





fn title_text(
    timer: Res<Time>,
    site: ResMut<nws::site::Site>,
    mut q: Query<(&mut Transform, &mut Text), With<TitleText>>,
) {
    //let (mut transform, mut text) = q.single_mut();

    //transform.rotate_y(0.1 * 2. * timer.delta_seconds());
}


fn update_text(
    site: Res<nws::site::Site>,
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>)>,
) {
/*    for mut transform in &mut query {
        //transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
        transform.translation.y = site.scroll.value * 0.008;
    }*/
}


#[derive(Default)]
pub struct Frame {
    width: f32,
    height: f32,
    color: Srgba,
    position: Vec2,
}


impl Frame {
    // This method will help users to discover the builder
    pub fn add_text(&mut self) -> &mut Frame {
        self
    }
}
