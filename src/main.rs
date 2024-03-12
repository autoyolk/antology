use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const COLOR_BG: Color = Color::rgb(0.2, 0.8, 0.2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(COLOR_BG))
        .add_systems(Startup, setup)
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
