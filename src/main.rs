use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const COLOR_BG: Color = Color::rgb(0.2, 0.8, 0.2);

const COLOR_ANT_QUEEN: Color = Color::rgb(0.8, 0.2, 0.2);
const COLOR_ANT_WORKER: Color = Color::rgb(0.2, 0.2, 0.2);

const SPEED_ANT_QUEEN: f32 = 0.5;
const SPEED_ANT_WORKER: f32 = 5.0;

const SIZE_ANT_QUEEN: f32 = 20.0;
const SIZE_ANT_WORKER: f32 = 10.0;

const NUM_ANT_WORKERS: i32 = 12;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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

    // Spawn the Queen

    commands.spawn(AntBundle::new(
        AntKind::Queen,
        Vec3::splat(0.0),
        &mut meshes,
        &mut materials,
    ));

    // Spawn the Workers

    for n in 0..NUM_ANT_WORKERS {
        let angle = Vec2::from_angle(n as f32 * (PI * 2.0 / NUM_ANT_WORKERS as f32));
        let position = angle * SIZE_ANT_QUEEN * 3.0;
        let position = position.extend(0.0);

        commands.spawn(AntBundle::new(
            AntKind::Worker,
            position,
            &mut meshes,
            &mut materials,
        ));
    }
}

#[derive(Component)]
struct Destination(Option<Vec2>);

enum AntKind {
    Queen,
    Worker,
}

#[derive(Component)]
struct Ant {
    kind: AntKind,
}

#[derive(Bundle)]
struct AntBundle {
    ant: Ant,
    destination: Destination,

    mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl AntBundle {
    pub fn new(
        kind: AntKind,
        position: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let mesh = Circle::default();
        let material = match kind {
            AntKind::Queen => COLOR_ANT_QUEEN,
            AntKind::Worker => COLOR_ANT_WORKER,
        };
        let size = match kind {
            AntKind::Queen => SIZE_ANT_QUEEN,
            AntKind::Worker => SIZE_ANT_WORKER,
        };

        Self {
            ant: Ant { kind },
            destination: Destination(None),

            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: materials.add(material),
                transform: Transform::from_translation(position).with_scale(Vec3::splat(size)),
                ..Default::default()
            },
        }
    }
}
