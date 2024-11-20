use std::path::Path;
use bevy::color::palettes;
use bevy::color::palettes::basic::PURPLE;
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
    let mut last_position = 512.;
    let mut x = 0.;
    let mut add_frame = |background_color: Srgba, height, background_image, image|
    {
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Rectangle::from_size(Vec2::new(width, height))),
            transform: Transform::from_xyz(0., last_position - height / 2., 0.0),
            material: materials.add(FrameMaterial {
                color: background_color.into(),
                color_texture: Some(asset_server.load(background_image)),
                image_texture: Some(asset_server.load(image)),
            }),
            ..default()
        });
        last_position -= height;
        x += 200.;
    };

    // let frame = nws::content::Frame::new();
    add_frame(Srgba::WHITE, 1024., Path::new("textures/background0.png"), Path::new("textures/image.png"));
    add_frame(Srgba::hex("121316").unwrap(), 2048., Path::new("textures/background1.png"), Path::new("textures/blank.png"));
    add_frame(Srgba::hex("F4CC81").unwrap(), 1024., Path::new("textures/background2.png"), Path::new("textures/image.png"));
    add_frame(Srgba::hex("9FAAC3").unwrap(), 1024., Path::new("textures/background3.png"), Path::new("textures/blank.png"));
    add_frame(Srgba::hex("81A3F4").unwrap(), 1024., Path::new("textures/background4.png"), Path::new("textures/blank.png"));
    add_frame(Srgba::hex("121316").unwrap(), 663., Path::new("textures/background5.png"), Path::new("textures/blank.png"));


    // text
    let title_font = asset_server.load("fonts/DrukTextWide-Bold.ttf");
    let title_style = TextStyle {
        font_size: 60.0,
        font: title_font.clone(),
        color: Color::WHITE,
    };
    commands.spawn(BillboardTextBundle {
        text_bounds: BillboardTextBounds( Text2dBounds{ size: Vec2::new(50., 200.) }  ),
        transform: Transform::from_xyz(-100., -500.0, 10.),
        text: Text::from_sections([
            TextSection {
                value: "Solar Burn".to_string(),
                style: title_style.clone()
            },
        ])
            .with_justify(JustifyText::Left),
        ..default()
    });

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
