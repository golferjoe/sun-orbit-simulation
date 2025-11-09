mod camera;
mod constants;
mod math;
mod orbit;
mod planet;
mod sun;
mod ui;

use bevy::{math::DVec2, prelude::*, window::WindowResolution};

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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planets = [
        (
            MERCURY_MASS,
            MERCURY_POS_X,
            MERCURY_VEL_Y,
            MERCURY_RADIUS,
            "textures/mercury.jpg",
        ),
        (
            VENUS_MASS,
            VENUS_POS_X,
            VENUS_VEL_Y,
            VENUS_RADIUS,
            "textures/venus.jpg",
        ),
        (
            EARTH_MASS,
            EARTH_POS_X,
            EARTH_VEL_Y,
            EARTH_RADIUS,
            "textures/earth.jpg",
        ),
        (
            MARS_MASS,
            MARS_POS_X,
            MARS_VEL_Y,
            MARS_RADIUS,
            "textures/mars.jpg",
        ),
        (
            JUPITER_MASS,
            JUPITER_POS_X,
            JUPITER_VEL_Y,
            JUPITER_RADIUS,
            "textures/jupiter.jpg",
        ),
        (
            SATURN_MASS,
            SATURN_POS_X,
            SATURN_VEL_Y,
            SATURN_RADIUS,
            "textures/saturn.jpg",
        ),
        (
            URANUS_MASS,
            URANUS_POS_X,
            URANUS_VEL_Y,
            URANUS_RADIUS,
            "textures/uranus.jpg",
        ),
        (
            NEPTUNE_MASS,
            NEPTUNE_POS_X,
            NEPTUNE_VEL_Y,
            NEPTUNE_RADIUS,
            "textures/neptune.jpg",
        ),
    ];

    for (mass, pos, vel, radius, texture) in planets {
        cmds.spawn(PlanetBundle::new(
            &asset_server,
            &mut meshes,
            &mut materials,
            mass,
            DVec2::new(pos, 0.0),
            DVec2::new(0.0, vel),
            radius,
            texture,
        ));
    }
}
