use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

const COLOR_BG: Color = Color::rgb(0.2, 0.8, 0.2);

const COLOR_ANT_QUEEN: Color = Color::rgb(0.8, 0.2, 0.2);
const COLOR_ANT_WORKER: Color = Color::rgb(0.2, 0.2, 0.2);

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
        .add_systems(Update, update_destinations)
        .add_systems(FixedUpdate, update_ants)
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

    commands.spawn((
        AntQueen,
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(COLOR_ANT_QUEEN),
            transform: Transform::from_translation(Vec3::splat(0.0))
                .with_scale(Vec3::splat(SIZE_ANT_QUEEN)),
            ..Default::default()
        },
    ));

    // Spawn the Workers

    for n in 0..NUM_ANT_WORKERS {
        let angle = Vec2::from_angle(n as f32 * (PI * 2.0 / NUM_ANT_WORKERS as f32));
        let position = angle * SIZE_ANT_QUEEN * 3.0;
        let position = position.extend(0.0);

        commands.spawn((
            AntWorker,
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(COLOR_ANT_WORKER),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(SIZE_ANT_WORKER)),
                ..Default::default()
            },
        ));
    }
}

fn update_destinations(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    query: Query<(EntityRef, &AntWorker), Without<Destination>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor) = window.single().cursor_position() {
            // 1. Convert click to world coordinates

            // Note: when many systems rely on click coordinates, it's easier to
            // keep it as a resouce: https://bevy-cheatbook.github.io/cookbook/cursor2world.html

            let (camera, camera_transform) = camera.single();
            let click = camera
                .viewport_to_world(camera_transform, cursor)
                .map(|ray| ray.origin.truncate());

            if let Some(click) = click {
                // 2. Find the closest ant

                let mut min_distance = f32::INFINITY;
                let mut min_distance_ant = None;

                for ant in query.iter() {
                    let transform = ant
                        .0
                        .get::<Transform>()
                        .expect("All ants should have Transform");
                    let distance = transform.translation.distance(click.extend(0.0));
                    if distance < min_distance {
                        min_distance = distance;
                        min_distance_ant = Some(ant.0.id());
                    }
                }

                // 3. Assign the destination to the closest ant if found

                if let Some(ant) = min_distance_ant {
                    commands.entity(ant).insert(Destination(click));
                }
            }
        }
    }
}

fn update_ants(
    mut commands: Commands,
    query: Query<(EntityRef, &Destination, &AntWorker)>,
    time: Res<Time>,
) {
    for ant in query.iter() {
        let transform = ant
            .0
            .get::<Transform>()
            .expect("All ants should have Transform");
        let position = transform.translation;
        let destination = ant.1.0;
        let direction = destination - position.xy();

        let mut ant = commands.entity(ant.0.id());

        if direction.length() < 1.0 {
            // To avoid float comparison, jump to the end when close enough
            ant.insert(transform.with_translation(destination.extend(position.z)));
            // And remove the destination as we reached it
            ant.remove::<Destination>();
        } else {
            // Otherwise just move as far as we can
            let movement = direction.clamp_length_max(time.delta_seconds() * SPEED_ANT_WORKER);
            ant.insert(transform.with_translation(position + movement.extend(0.0)));
        };
    }
}

#[derive(Component)]
struct AntQueen;

#[derive(Component)]
struct AntWorker;

#[derive(Component)]
struct Destination(Vec2);
