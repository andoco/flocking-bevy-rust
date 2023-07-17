mod boid;
mod util;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use boid::{Avoidance, BoidBundle, FollowTarget};
use rand::Rng;

use crate::boid::BoidPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BoidPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..default()
        },
        FollowTarget,
    ));

    for _ in 0..100 {
        let radius = 10.0;

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-300.0..300.0);
        let y = rng.gen_range(-300.0..300.0);
        let pos = Vec3::new(x, y, 0.0);

        commands
            .spawn((
                BoidBundle {
                    avoidance: Avoidance {
                        radius: radius * 2.0,
                    },
                    ..default()
                },
                Collider::ball(radius),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
                ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC,
            ))
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(pos),
                ..default()
            });
    }
}

fn input(
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut follow_query: Query<&mut Transform, With<FollowTarget>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = window_query.single();

        let Ok((camera, camera_tx)) = camera_query.get_single() else {
            return;
        };

        if let Some(position) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(camera_tx, position) {
                let world_pos = ray.get_point(0.0);
                if let Ok(mut follow_tx) = follow_query.get_single_mut() {
                    follow_tx.translation = Vec3::new(world_pos.x, world_pos.y, 0.0);
                }
            }
        }
    }
}
