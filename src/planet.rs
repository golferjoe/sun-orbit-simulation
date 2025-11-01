use bevy::{math::DVec2, prelude::*};

use crate::{
    constants::{DISTANCE_SCALE, TIME_SCALE},
    math::physics::velocity_verlet,
};

#[derive(Clone, Component)]
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
            planet: Planet {
                mass,
                position,
                velocity,
            },
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

fn update_planet_physics(time: Res<Time>, planets: Query<&mut Planet>) {
    for mut planet in planets {
        let dt = time.delta_secs_f64() * TIME_SCALE;
        let (pos_new, vel_new) = velocity_verlet(dt, planet.position, planet.velocity, planet.mass);
        planet.position = pos_new;
        planet.velocity = vel_new;
    }
}
