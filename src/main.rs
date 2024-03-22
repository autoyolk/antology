use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

const COLOR_BG: Color = Color::rgb(0.2, 0.8, 0.2);

const COLOR_ANT_QUEEN: Color = Color::rgb(0.8, 0.2, 0.2);
const COLOR_ANT_WORKER: Color = Color::rgb(0.2, 0.2, 0.2);
const COLOR_MOVEMENT: Color = Color::rgb(0.0, 0.0, 0.0);

// const SPEED_ANT_QUEEN: f32 = 0.5;
const SPEED_ANT_WORKER: f32 = 25.0;

const SIZE_ANT_QUEEN: f32 = 20.0;
const SIZE_ANT_WORKER: f32 = 10.0;

const NUM_ANT_WORKERS: i32 = 12;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(COLOR_BG))
        .add_systems(Startup, setup)
        .add_systems(Update, register_movement)
        .add_systems(Update, draw_movements)
        .add_systems(FixedUpdate, update_movements)
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

    commands.spawn(AntQueenBundle::new(&mut meshes, &mut materials));

    // Spawn the Workers

    for n in 0..NUM_ANT_WORKERS {
        let angle = Vec2::from_angle(n as f32 * (PI * 2.0 / NUM_ANT_WORKERS as f32));
        let position = angle * SIZE_ANT_QUEEN * 3.0;

        commands.spawn(AntWorkerBundle::new(position, &mut meshes, &mut materials));
    }
}

fn register_movement(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_mouse: Res<ButtonInput<MouseButton>>,
    q_movements: Query<&Movement>,
    q_ants: Query<(EntityRef, &Transform, &Children), With<AntWorker>>,
) {
    if q_mouse.just_pressed(MouseButton::Left) {
        if let Some(cursor) = q_window.single().cursor_position() {
            // 1. Convert click to world coordinates

            // Note: when many systems rely on click coordinates, it's easier to
            // keep it as a resouce: https://bevy-cheatbook.github.io/cookbook/cursor2world.html

            let (camera, camera_transform) = q_camera.single();
            let click = camera
                .viewport_to_world(camera_transform, cursor)
                .map(|ray| ray.origin.truncate());

            if let Some(click) = click {

                println!("Click at {}", click);

                // 2. Find the closest ant without a movement

                let mut min_distance = f32::INFINITY;
                let mut min_distance_ant = None;

                for (ant, transform, children) in q_ants.iter() {
                    println!("Searching for ants");

                    if children.iter().find(|e| q_movements.contains(**e)).is_some() {
                        continue;
                    }

                    let distance = transform.translation.distance(click.extend(0.0));
                    if distance < min_distance {
                        min_distance = distance;
                        min_distance_ant = Some(ant.id());
                    }
                }

                // 3. If there is the closest ant, spawn the movement and add it as a child of ant

                if let Some(ant) = min_distance_ant {

                    println!("Found an ant for this movement");

                    let movement = commands.spawn(Movement {
                        to: click,
                    }).id();

                    let mut ant = commands.get_entity(ant).unwrap();
                    ant.push_children(&[movement]);
                }
            }
        }
    }
}

fn update_movements(
    mut commands: Commands,
    time: Res<Time>,
    q_movements: Query<(EntityRef, &Movement, &Parent), Without<AntWorker>>,
    mut q_ants: Query<&mut Transform, With<AntWorker>>,
) {
    for (movement_ref, movement, parent) in q_movements.iter() {
        let mut ant = q_ants.get_mut(parent.get()).unwrap();

        let destination = movement.to.extend(0.0);
        let direction = destination - ant.translation;

        // Move as far as we can
        ant.translation = direction.clamp_length(0.0, time.delta_seconds() * SPEED_ANT_WORKER);

        let distance = destination - ant.translation;

        // To avoid float comparison, jump to the end when close enough
        if distance.length() < 1.0 {
            ant.translation = destination;
            // And remove the things we are done with
            commands.entity(movement_ref.id()).despawn();
            commands.entity(parent.get()).remove_children(&[movement_ref.id()]);
        }
    }
}

fn draw_movements(
    mut gizmos: Gizmos,
    q_movements: Query<(&Movement, &Parent)>,
    q_ants: Query<&Transform, With<AntWorker>>,
) {
    for (movement, parent) in q_movements.iter() {
        let ant = q_ants.get(parent.get()).unwrap();

        let from = ant.translation.truncate();
        let to = movement.to;

        let segment = Segment2d::from_points(from, to);

        gizmos.primitive_2d(segment.0, segment.1, 0.0, COLOR_MOVEMENT);
    }
}

#[derive(Component)]
struct AntQueen;

#[derive(Bundle)]
struct AntQueenBundle {
    ant: AntQueen,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl AntQueenBundle {
    fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        return Self {
            ant: AntQueen,
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(COLOR_ANT_QUEEN),
                transform: Transform::from_translation(Vec3::splat(0.0))
                    .with_scale(Vec3::splat(SIZE_ANT_QUEEN)),
                ..Default::default()
            },
        };
    }
}

#[derive(Component)]
struct AntWorker;

#[derive(Bundle)]
struct AntWorkerBundle {
    ant: AntWorker,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl AntWorkerBundle {
    fn new(
        position: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        return Self {
            ant: AntWorker,
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(COLOR_ANT_WORKER),
                transform: Transform::from_translation(position.extend(0.0))
                    .with_scale(Vec3::splat(SIZE_ANT_WORKER)),
                ..Default::default()
            },
        };
    }
}

#[derive(Component)]
struct Movement {
    to: Vec2,
}
