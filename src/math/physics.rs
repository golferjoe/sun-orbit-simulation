use bevy::math::{DVec2, Vec2};

use crate::constants::{DISTANCE_SCALE, G, SUN_MASS};

const A: f64 = 7e-25;
const CUT_OUT: f64 = 249261000000.0;
const MIN_VALUE: f64 = 1.0;

pub fn compute_acceleration(pos: DVec2, mass: f64) -> DVec2 {
    // calculate the distance to earth from sun's center
    // but because our sun is at (0.0, 0.0) we can omit it from equation
    let d = DVec2::new(-pos.x, -pos.y);
    let r = (d.x * d.x + d.y * d.y).sqrt(); // distance between sun's and earth's center (in meters)

    // calculate gravitational forces (F) for each axis
    let f = (G * ((SUN_MASS * mass) / (r * r))) * (d / r);

    // calculate acceleration: F=ma -> a=F/m
    f / mass
}

pub fn velocity_verlet(
    dt: f64,
    mut position: DVec2,
    mut velocity: DVec2,
    mass: f64,
) -> (DVec2, DVec2) {
    let a1 = compute_acceleration(position, mass); // first acceleration
    let v_temp = velocity; // temporary velocity only for calculations
    position += v_temp * dt + 0.5 * a1 * dt * dt;

    let a2 = compute_acceleration(position, mass); // second acceleration
    velocity += 0.5 * (a1 + a2) * dt;

    (position, velocity)
}

fn compute_planet_distance_scale(total_dist: f64) -> f64 {
    let c = -(A * CUT_OUT * CUT_OUT);

    let scale = if total_dist > -CUT_OUT && total_dist < CUT_OUT {
        MIN_VALUE
    } else {
        ((A * total_dist * total_dist + c) + MIN_VALUE).powf(0.3)
    };

    DISTANCE_SCALE * scale
}

pub fn scale_distance_to_bevy(pos: DVec2) -> Vec2 {
    let r = (pos.x * pos.x + pos.y * pos.y).sqrt();
    let scale = compute_planet_distance_scale(r);
    (pos / scale).as_vec2()
}
