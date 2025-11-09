use bevy::{math::DVec2, prelude::*};

use crate::{
    math::physics::{scale_distance_to_bevy, velocity_verlet},
    ui::egui::Gui,
};

#[derive(Clone, Component)]
pub struct Planet {
    pub mass: f64,
    pub position: DVec2,
    pub previous_position: DVec2, // used for visual lerping
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
                previous_position: position,
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
        app.insert_resource(Time::<Fixed>::from_hz(90.0))
            .add_systems(FixedUpdate, update_planet_physics)
            .add_systems(Update, update_planet_transforms);
    }
}

fn update_planet_physics(settings: Res<Gui>, time: Res<Time<Fixed>>, planets: Query<&mut Planet>) {
    let time_scale = settings.time_scale.to_seconds();

    for mut planet in planets {
        planet.previous_position = planet.position;

        let dt = time.delta_secs_f64() * time_scale;
        let (pos_new, vel_new) = velocity_verlet(dt, planet.position, planet.velocity, planet.mass);
        planet.position = pos_new;
        planet.velocity = vel_new;
    }
}

fn update_planet_transforms(
    fixed_time: Res<Time<Fixed>>,
    planets: Query<(&mut Transform, &Planet)>,
) {
    for (mut transform, planet) in planets {
        let interpolated = scale_distance_to_bevy(
            planet
                .previous_position
                .lerp(planet.position, fixed_time.overstep_fraction_f64()),
        );

        transform.translation.x = interpolated.x;
        transform.translation.z = interpolated.y;
    }
}
