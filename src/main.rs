mod camera;
mod constants;
mod debug;
mod math;
mod orbit;
mod planet;
mod sun;

use bevy::{
    color::palettes::css::{BLUE, GRAY},
    math::DVec2,
    prelude::*,
    window::WindowResolution,
};

use crate::{
    camera::CameraPlugin,
    constants::{
        EARTH_MASS, EARTH_POS_X, EARTH_RADIUS, EARTH_VEL_Y, VENUS_MASS, VENUS_POS_X, VENUS_RADIUS,
        VENUS_VEL_Y,
    },
    debug::DebugPlugin,
    orbit::OrbitPlugin,
    planet::{PlanetBundle, PlanetPlugin},
    sun::SunPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Solar System Simulation".to_string(),
                resolution: WindowResolution::new(1920, 1080),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            CameraPlugin,
            DebugPlugin,
            SunPlugin,
            PlanetPlugin,
            OrbitPlugin,
        ))
        .add_systems(Startup, create_planets)
        .run();
}

fn create_planets(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn_batch([
        // earth
        PlanetBundle::new(
            &mut meshes,
            &mut materials,
            EARTH_MASS,
            DVec2::new(EARTH_POS_X, 0.0),
            DVec2::new(0.0, EARTH_VEL_Y),
            Color::from(BLUE),
            EARTH_RADIUS,
        ),
        // venus
        PlanetBundle::new(
            &mut meshes,
            &mut materials,
            VENUS_MASS,
            DVec2::new(VENUS_POS_X, 0.0),
            DVec2::new(0.0, VENUS_VEL_Y),
            Color::from(GRAY),
            VENUS_RADIUS,
        ),
    ]);
}
