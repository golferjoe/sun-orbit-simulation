use bevy::{math::DVec2, prelude::*};

use crate::constants::{DISTANCE_SCALE, G, SUN_MASS, TIME_SCALE};

#[derive(Component)]
pub struct Planet {
    pub mass: f64,
    pub position: DVec2,
    pub velocity: DVec2,
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}

impl PlanetBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        mass: f64,
        position: DVec2,
        velocity: DVec2,
        color: Color,
        px_size: f32,
    ) -> Self {
        Self {
            planet: Planet { mass, position, velocity },
            mesh: Mesh2d(meshes.add(Circle::default())),
            material: MeshMaterial2d(materials.add(color)),
            transform: Transform::default()
                .with_translation(Vec3::new(position.x as f32, position.y as f32, 0.0))
                .with_scale(Vec3::splat(px_size)),
        }
    }
}

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_planet_physics, update_planet_transforms).chain(),
        );
    }
}

fn update_planet_transforms(planets: Query<(&mut Transform, &Planet)>) {
    for (mut transform, planet) in planets {
        let pos = planet.position;

        // using explicit conversion so we dont mix the precision
        transform.translation.x = (pos.x / DISTANCE_SCALE) as f32;
        transform.translation.y = (pos.y / DISTANCE_SCALE) as f32;
    }
}

fn compute_acceleration(pos: DVec2, mass: f64) -> DVec2 {
    // calculate the distance to earth from sun's center
    // but because our sun is at (0.0, 0.0) we can omit it from equation
    let d = DVec2::new(-pos.x, -pos.y);
    let r = (d.x * d.x + d.y * d.y).sqrt(); // distance between sun's and earth's center (in meters)

    // calculate gravitational forces (F) for each axis
    let f = (G * ((SUN_MASS * mass) / (r * r))) * (d / r);

    // calculate acceleration: F=ma -> a=F/m
    f / mass
}

fn update_planet_physics(
    time: Res<Time>,
    planets: Query<&mut Planet>,
) {
    for mut planet in planets {
        let dt = time.delta_secs_f64() * TIME_SCALE;

        // using velocity verlet
        let a1 = compute_acceleration(planet.position, planet.mass); // first acceleration
        let v_temp = planet.velocity; // temporary velocity only for calculations
        planet.position += v_temp * dt + 0.5 * a1 * dt * dt;

        let a2 = compute_acceleration(planet.position, planet.mass); // second acceleration
        planet.velocity += 0.5 * (a1 + a2) * dt;
    }
}
