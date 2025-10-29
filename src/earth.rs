use bevy::{color::palettes::css::BLUE, math::DVec2, prelude::*};

use crate::constants::{DISTANCE_SCALE, EARTH_SIZE_PX, TIME_SCALE};

const G: f64 = 6.6743e-11; // gravitational force
const SUN_MASS: f64 = 1.9885e30;
const EARTH_MASS: f64 = 5.972168e24;

const EARTH_POS_X: f64 = 1.496e11; // 1 AU
const EARTH_VEL_Y: f64 = 29_780.0; // m/s

const SUN_POS_X: f64 = 0.0;
const SUN_POS_Y: f64 = 0.0;

pub struct EarthPlugin;

impl Plugin for EarthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (update_by_velocity, calculate_velocity));
    }
}

#[derive(Component)]
struct Earth;

#[derive(Component)]
struct Position(DVec2);

#[derive(Component)]
struct Velocity(DVec2);

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
) {
    let position = Position(DVec2::new(EARTH_POS_X, 0.0));

    let transform = Transform::default()
        .with_translation(Vec3::new(position.0.x as f32, position.0.y as f32, 0.0))
        .with_scale(Vec3::splat(EARTH_SIZE_PX as f32));

    cmds.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(mats.add(Color::from(BLUE))),
        transform,
        position,
        Velocity(DVec2::new(0.0, EARTH_VEL_Y)),
        Earth,
    ));
}

fn update_by_velocity(query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query {
        transform.translation.x = (position.0.x / DISTANCE_SCALE) as f32;
        transform.translation.y = (position.0.y / DISTANCE_SCALE) as f32;
    }
}

fn compute_acceleration(earth_pos: DVec2) -> DVec2 {
    // calculate the distance to earth from sun's center
    let d_x = SUN_POS_X - earth_pos.x; // can be shortened to -position.0.x
    let d_y = SUN_POS_Y - earth_pos.y;
    let r = (d_x * d_x + d_y * d_y).sqrt();

    // calculate gravitational force
    let f_x = (G * ((SUN_MASS * EARTH_MASS) / (r * r))) * (d_x / r);
    let f_y = (G * ((SUN_MASS * EARTH_MASS) / (r * r))) * (d_y / r);

    // calculate acceleration: F=ma -> a=F/m
    let a_x = f_x / EARTH_MASS;
    let a_y = f_y / EARTH_MASS;

    return DVec2::new(a_x, a_y);
}

fn calculate_velocity(
    time: Res<Time>,
    query: Query<(&mut Velocity, &mut Position), With<Earth>>,
) {
    for (mut velocity, mut position) in query {
        let dt = time.delta_secs_f64() * TIME_SCALE;

        // velocity verlet
        let a1 = compute_acceleration(position.0);
        let x = position.0.x + velocity.0.x * dt + 0.5 * a1.x * dt * dt;
        let y = position.0.y + velocity.0.y * dt + 0.5 * a1.y * dt * dt;

        let a2 = compute_acceleration(DVec2::new(x, y));
        velocity.0.x += 0.5 * (a1.x + a2.x) * dt;
        velocity.0.y += 0.5 * (a1.y + a2.y) * dt;

        position.0.x = x;
        position.0.y = y;
    }
}
