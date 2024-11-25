use std::cell::Cell;
use std::path::Path;
use bevy::color::palettes;
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, RenderPipeline, ShaderRef};
use bevy::text::Text2dBounds;
use bevy_mod_billboard::BillboardTextBundle;
use bevy_mod_billboard::prelude::BillboardPlugin;
use bevy_mod_billboard::text::BillboardTextBounds;
use crate::can::CanEntity;
use crate::component::{UIText};
use crate::material::{MoonComponent, MoonMaterial};
use crate::{nws, ScrollEvent};

pub struct ContentPlugin;
impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<FrameMaterial>::default())
            .add_plugins(MaterialPlugin::<MoonMaterial>::default())
            .add_plugins(BillboardPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (update, update_text, update_shader));
    }
}

#[derive(Component, Default)]
struct TitleText
{
    pub position: Vec2,
    pub target_position: Vec2,
}


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
    mut extmaterials: ResMut<Assets<MoonMaterial>>,
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

    let font_title = asset_server.load("fonts/DrukTextWide-Bold.ttf");
    let font_text = asset_server.load("fonts/Inter_18pt-Medium.ttf");
    let mut add_text = |cmds: &mut Commands, text: &str, background_color: Srgba, fsize: f32, pos: Vec2, font: Handle<Font>|
    {
        let top = pos.y;
        let left = pos.x * 1.2;

        cmds.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: fsize,
                    color: background_color.into(),
                    ..default()
                },
            )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(top),
                    left: Val::Px(left),
                    ..default()
                }),
            UIText{ pos, fixed_pos: pos, target_pos: pos },
        ));
    };


    // let frame = nws::content::Frame::new();
    add_frame(&mut commands, Srgba::WHITE, 1024., Path::new("textures/background0.png"), Path::new("textures/image.png"));
    add_frame(&mut commands, Srgba::hex("121316").unwrap(), 1534., Path::new("textures/background1.png"), Path::new("textures/blank.png"));
    add_frame(&mut commands, Srgba::hex("F4CC81").unwrap(), 1024., Path::new("textures/background2.png"), Path::new("textures/image.png"));
    add_frame(&mut commands, Srgba::hex("9FAAC3").unwrap(), 1024., Path::new("textures/background3.png"), Path::new("textures/blank.png"));
    add_frame(&mut commands, Srgba::hex("81A3F4").unwrap(), 1024., Path::new("textures/background4.png"), Path::new("textures/blank.png"));
    add_frame(&mut commands, Srgba::hex("121316").unwrap(), 663., Path::new("textures/blank.png"), Path::new("textures/blank.png"));


    let text_left = 739.;

    add_text(&mut commands, "La marque de soda qui va vous désaltérer tout en vous offrant
des saveurs uniques au monde. Comme si vous dégustiez un met
exquis dans un restaurant luxueux, appréciez la complexité des
arômes spéciaux des boissons MOIST.", Srgba::hex("121316").unwrap(), 20., Vec2::new(128., 843.), font_text.clone());

    add_text(&mut commands, "DES BULLES\nDE BONHEUR", Srgba::hex("F4CC81").unwrap(), 64., Vec2::new(text_left, 1221.), font_title.clone());
    add_text(&mut commands, "DES INGRÉDIENTS 100% NATURELS", Srgba::hex("F4CC81").unwrap(), 16., Vec2::new(text_left, 1604.), font_title.clone());
    add_text(&mut commands, "Nous tenons particulièrement à ce que toutes nos boissons
soient composés des ingrédients les plus purs possibles.
Pas d’additif, pas d’édulcorant, pas de colorant artificiels,
que des saveurs extraites à la pulpe de la terre.", Srgba::hex("FFFFFF").unwrap(), 16., Vec2::new(text_left, 1604. + 47.), font_text.clone());

    add_text(&mut commands, "MADE IN FRANCE. C’EST TOUT.", Srgba::hex("F4CC81").unwrap(), 16., Vec2::new(text_left, 1827.), font_title.clone());
    add_text(&mut commands, "De nos fournisseurs à nos usines, nous choisissons
précautionneusement nos partenaires en France pour une
confection qui ne dépasse jamais nos frontières.", Srgba::hex("FFFFFF").unwrap(), 16., Vec2::new(text_left, 1827. + 47.), font_text.clone());





    //add_text(&mut commands, "SOLAR BURN", Srgba::hex("121316").unwrap(), 96. - 10., Vec2::new(471., 192.), 1534., Transform::from_xyz(750., 161., 1.));
    //add_text(&mut commands, "MOON DROP", Srgba::hex("121316").unwrap(), 96. - 10., Vec2::new(471., 192.), 1024., Transform::from_xyz(750., 161., 1.));

}





fn title_text(
    timer: Res<Time>,
    site: ResMut<nws::site::Site>,
    mut q: Query<(&mut Transform, &mut Text), With<TitleText>>,
) {
    //let (mut transform, mut text) = q.single_mut();

    //transform.rotate_y(0.1 * 2. * timer.delta_seconds());
}

fn update(
    site: ResMut<nws::site::Site>,
    mut scroll_event: EventReader<ScrollEvent>,
    mut query: Query<(&mut Style, &mut UIText)>)
{
    for e in scroll_event.read() {
        for (mut style, mut text) in &mut query {
            text.target_pos.y = text.fixed_pos.y + site.scroll.value * 0.95;
            //text.pos.y =  text.target_pos.y - 100.0;
        }
    }
}


fn update_text(
    site: Res<nws::site::Site>,
    time: Res<Time>,
    mut query: Query<(&mut Style, &mut UIText)>,
) {
    for (mut style, mut text) in &mut query {
        // transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
        // style.top = Val::Px(text.top + site.scroll.value * 1.1);

        text.pos.y = text.pos.y.lerp(text.target_pos.y, 0.1);
        style.top = Val::Px( text.pos.y );

        //style.top = Val::Px( text.target_pos.y );
    }
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

fn update_shader(site: ResMut<nws::site::Site>,
                 timer: Res<Time>,
                 mut materials: ResMut<Assets<MoonMaterial>>,
                 mut query: Query<&mut MoonComponent>)
{
/*    let mut moon_component = query.single_mut();
    if let Some(moon_material) = materials.get_mut(&moon_component.material) {
        moon_material.mouse = site.mouse;
    }*/
}