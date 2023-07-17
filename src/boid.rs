use bevy::prelude::*;

use crate::util::signed_angle;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_follow, update_avoidance, update_position).chain(),
        );
    }
}

#[derive(Bundle)]
pub struct BoidBundle {
    pub boid: Boid,
    pub speed: Speed,
    pub turn_rate: TurnRate,
    pub follow: Follow,
    pub follow_velocity: FollowVelocity,
    pub avoidance: Avoidance,
    pub avoid_velocity: AvoidVelocity,
    pub spatial: SpatialBundle,
}

impl Default for BoidBundle {
    fn default() -> Self {
        Self {
            boid: Boid,
            speed: Speed(50.0),
            turn_rate: TurnRate(45.0),
            avoidance: Avoidance { radius: 10.0 },
            avoid_velocity: Default::default(),
            follow: Follow,
            follow_velocity: Default::default(),
            spatial: SpatialBundle::default(),
        }
    }
}

#[derive(Component)]
pub struct Boid;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct TurnRate(pub f32);

#[derive(Component)]
pub struct FollowTarget;

#[derive(Component)]
pub struct Follow;

#[derive(Component, Default)]
pub struct FollowVelocity(Vec3);

#[derive(Component)]
pub struct Avoidance {
    pub radius: f32,
}

#[derive(Component, Default)]
pub struct AvoidVelocity(Vec3);

fn update_follow(
    target_query: Query<&GlobalTransform, With<FollowTarget>>,
    mut follower_query: Query<(&GlobalTransform, &mut FollowVelocity), With<Follow>>,
) {
    let Ok(target_global_tx) = target_query.get_single() else {
        return;
    };

    for (global_tx, mut velocity) in follower_query.iter_mut() {
        let offset = target_global_tx.translation() - global_tx.translation();
        velocity.0 = offset.normalize();
    }
}

fn update_avoidance(
    boid_query: Query<(Entity, &GlobalTransform), With<Boid>>,
    mut query: Query<(Entity, &GlobalTransform, &Avoidance, &mut AvoidVelocity)>,
) {
    let positions: Vec<(Entity, Vec3)> = boid_query
        .iter()
        .map(|(entity, global_tx)| (entity, global_tx.translation()))
        .collect();

    for (entity, global_tx, avoidance, mut velocity) in query.iter_mut() {
        let mut desired_velocity = Vec3::ZERO;

        for (other_entity, other_position) in positions.iter() {
            if entity == *other_entity {
                continue;
            }

            if global_tx.translation().distance(*other_position) < avoidance.radius {
                desired_velocity += global_tx.translation() - *other_position;
            }
        }

        velocity.0 = desired_velocity.normalize_or_zero();
    }
}

fn update_position(
    mut query: Query<(
        &Speed,
        &TurnRate,
        &FollowVelocity,
        &AvoidVelocity,
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    for (Speed(speed), TurnRate(turn_rate), FollowVelocity(follow), AvoidVelocity(avoid), mut tx) in
        query.iter_mut()
    {
        let move_dir = tx.up();

        tx.translation += move_dir * *speed * time.delta_seconds();

        let desired_velocity = (*follow + *avoid).normalize();

        let a = move_dir;
        let b = desired_velocity.normalize_or_zero();
        let angle = signed_angle(Vec2::new(a.x, a.y), Vec2::new(b.x, b.y));

        if angle.abs() > 0.0 {
            let angle_dir = angle.signum();
            tx.rotate_z(angle_dir * turn_rate.to_radians() * time.delta_seconds());
        }
    }
}
