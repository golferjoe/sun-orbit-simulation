mod camera;
mod constants;
mod math;
mod orbit;
mod planet;
mod sun;
mod ui;

use bevy::{
    color::palettes::{
        css::{BLUE, GRAY, ORANGE, ORANGE_RED, WHITE_SMOKE, YELLOW_GREEN},
        tailwind::BLUE_300,
    },
    math::DVec2,
    prelude::*,
    window::WindowResolution,
};

use crate::{
    camera::CameraPlugin,
    constants::{
        EARTH_MASS, EARTH_POS_X, EARTH_RADIUS, EARTH_VEL_Y, JUPITER_MASS, JUPITER_POS_X,
        JUPITER_RADIUS, JUPITER_VEL_Y, MARS_MASS, MARS_POS_X, MARS_RADIUS, MARS_VEL_Y,
        MERCURY_MASS, MERCURY_POS_X, MERCURY_RADIUS, MERCURY_VEL_Y, NEPTUNE_MASS, NEPTUNE_POS_X,
        NEPTUNE_RADIUS, NEPTUNE_VEL_Y, SATURN_MASS, SATURN_POS_X, SATURN_RADIUS, SATURN_VEL_Y,
        URANUS_MASS, URANUS_POS_X, URANUS_RADIUS, URANUS_VEL_Y, VENUS_MASS, VENUS_POS_X,
        VENUS_RADIUS, VENUS_VEL_Y,
    },
    orbit::OrbitPlugin,
    planet::{PlanetBundle, PlanetPlugin},
    sun::SunPlugin,
    ui::plugin::UiPlugin,
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
        .add_plugins((UiPlugin, CameraPlugin, SunPlugin, PlanetPlugin, OrbitPlugin))
        .add_systems(Startup, create_planets)
        .run();
}

fn create_planets(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planets = [
        (
            MERCURY_MASS,
            MERCURY_POS_X,
            MERCURY_VEL_Y,
            GRAY,
            MERCURY_RADIUS,
        ),
        (
            VENUS_MASS,
            VENUS_POS_X,
            VENUS_VEL_Y,
            WHITE_SMOKE,
            VENUS_RADIUS,
        ),
        (EARTH_MASS, EARTH_POS_X, EARTH_VEL_Y, BLUE, EARTH_RADIUS),
        (MARS_MASS, MARS_POS_X, MARS_VEL_Y, ORANGE, MARS_RADIUS),
        (
            JUPITER_MASS,
            JUPITER_POS_X,
            JUPITER_VEL_Y,
            ORANGE_RED,
            JUPITER_RADIUS,
        ),
        (
            SATURN_MASS,
            SATURN_POS_X,
            SATURN_VEL_Y,
            YELLOW_GREEN,
            SATURN_RADIUS,
        ),
        (
            URANUS_MASS,
            URANUS_POS_X,
            URANUS_VEL_Y,
            WHITE_SMOKE,
            URANUS_RADIUS,
        ),
        (
            NEPTUNE_MASS,
            NEPTUNE_POS_X,
            NEPTUNE_VEL_Y,
            BLUE_300,
            NEPTUNE_RADIUS,
        ),
    ];

    for (mass, pos, vel, color, radius) in planets {
        cmds.spawn(PlanetBundle::new(
            &mut meshes,
            &mut materials,
            mass,
            DVec2::new(pos, 0.0),
            DVec2::new(0.0, vel),
            Color::from(color),
            radius,
        ));
    }
}
