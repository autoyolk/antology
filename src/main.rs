use bevy::{
    prelude::*, render::{settings::{Backends, WgpuSettings}, RenderPlugin}, sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

const COLOR_BG: Color = Color::rgb(0.2, 0.8, 0.2);

fn main() {
    let render_plugin = RenderPlugin {
        render_creation: WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..default()
        }
        .into(),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(render_plugin))
        .insert_resource(ClearColor(COLOR_BG))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mesh = Mesh2dHandle(meshes.add(Rectangle::new(10.0, 10.0)));
    let color = Color::rgb(0.8, 0.2, 0.2);

    commands.spawn(MaterialMesh2dBundle {
        mesh,
        material: materials.add(color),
        ..Default::default()
    });
}
