mod constants;
mod debug;
mod planet;
mod sun;

use bevy::{color::palettes::css::{BLUE, GRAY}, math::DVec2, prelude::*, window::WindowResolution};

use crate::{constants::{EARTH_MASS, EARTH_POS_X, EARTH_SIZE_PX, EARTH_VEL_Y, VENUS_MASS, VENUS_POS_X, VENUS_SIZE_PX, VENUS_VEL_Y}, debug::DebugPlugin, planet::{PlanetBundle, PlanetPlugin}, sun::SunPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My First Window".to_string(),
                resolution: WindowResolution::new(1280, 720),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((DebugPlugin, SunPlugin, PlanetPlugin))
        .add_systems(Startup, create_planets)
        .run();
}

fn create_planets(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmds.spawn(Camera2d);

    cmds.spawn_batch([
        // earth
        PlanetBundle::new(
            &mut meshes, &mut materials, EARTH_MASS,
            DVec2::new(EARTH_POS_X, 0.0), DVec2::new(0.0, EARTH_VEL_Y),
            Color::from(BLUE), EARTH_SIZE_PX,
        ),
        // venus
        PlanetBundle::new(
            &mut meshes, &mut materials,
            VENUS_MASS, DVec2::new(VENUS_POS_X, 0.0), DVec2::new(0.0, VENUS_VEL_Y),
            Color::from(GRAY), VENUS_SIZE_PX,
        )
    ]);
}
