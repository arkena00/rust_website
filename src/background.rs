use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        render_resource::{
            AsBindGroup, ShaderRef,
        },
    },
};

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct BackgroundMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}



pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<BackgroundMaterial>::default())
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //let mut rng = rand::thread_rng();
    //let start_time = rng.gen_range(0.0..100.0f32);
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Rectangle::new(5.0, 10.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(BackgroundMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("can.png")),
        }),
        ..default()
    });

}


impl Material for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}
