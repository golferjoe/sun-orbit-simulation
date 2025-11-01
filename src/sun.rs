use bevy::{color::palettes::css::YELLOW, prelude::*};

use crate::constants::SUN_SIZE_PX;

pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
) {
    cmds.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(mats.add(Color::from(YELLOW))),
        Transform::default().with_scale(Vec3::splat(SUN_SIZE_PX as f32)),
    ));
}
