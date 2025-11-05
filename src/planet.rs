use bevy::{math::DVec2, prelude::*};

use crate::{
    constants::TIME_SCALE,
    math::physics::{scale_distance_to_bevy, velocity_verlet},
};

#[derive(Clone, Component)]
pub struct Planet {
    pub mass: f64,
    pub position: DVec2,
    pub velocity: DVec2,
    pub orbit_points: Vec<Vec3>,
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
}

impl PlanetBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        mass: f64,
        position: DVec2,
        velocity: DVec2,
        color: Color,
        radius: f32,
    ) -> Self {
        Self {
            planet: Planet {
                mass,
                position,
                velocity,
                orbit_points: vec![],
            },
            mesh: Mesh3d(meshes.add(Sphere::new(radius))),
            material: MeshMaterial3d(materials.add(color)),
            transform: Transform::default().with_translation(Vec3::new(
                position.x as f32,
                position.y as f32,
                0.0,
            )),
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
        let pos = scale_distance_to_bevy(planet.position);
        transform.translation.x = pos.x;
        transform.translation.z = pos.y;
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
