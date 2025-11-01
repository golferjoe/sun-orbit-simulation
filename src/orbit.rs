use core::{f32, f64};

use bevy::{
    app::{App, Plugin, PostStartup, Startup, Update},
    asset::Assets,
    camera::visibility::RenderLayers,
    color::{
        Color,
        palettes::css::{GREEN, MAGENTA, RED, YELLOW},
    },
    ecs::{
        component::Component,
        system::{Commands, Query, ResMut},
    },
    gizmos::{
        AppGizmoBuilder,
        config::{DefaultGizmoConfigGroup, GizmoConfig, GizmoLineConfig},
        gizmos::Gizmos,
    },
    log::info,
    math::{DVec2, Vec2, Vec3, primitives::Rectangle},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
    window::Window,
};

use crate::{
    constants::DISTANCE_SCALE,
    math::{drect::DRect, physics::velocity_verlet},
    planet::Planet,
};

const MAX_POINTS: usize = 128;
const LINE_WIDTH: f32 = 2.0;
const LINE_ALPHA: f32 = 0.05;
const SHOW_QUARTERS: bool = false;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_gizmo_config(
            DefaultGizmoConfigGroup,
            GizmoConfig {
                render_layers: RenderLayers::layer(1),
                line: GizmoLineConfig {
                    width: LINE_WIDTH,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        app.add_systems(Startup, create_quarters);
        app.add_systems(PostStartup, create_orbit_points);
        app.add_systems(Update, draw_orbit_gizmos);
    }
}

#[derive(Component)]
struct Quarter(DRect);

#[derive(Component)]
struct PlanetOrbit(Vec<Vec2>);

// the thing im doing here is not ideal. if the planet's orbit is further than rectangles then we will have unknown quarters
fn create_quarters(
    windows: Query<&Window>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single().unwrap();
    let window_width = window.resolution.width() as f64;
    let window_height = window.resolution.height() as f64;

    // create four rectangles with sun at the intersection
    // all rects start from top-left point to bottom-right point
    let top_right = DRect::new(
        DVec2::new(0.0, f64::INFINITY),
        DVec2::new(f64::INFINITY, 0.0),
    );
    let top_left = DRect::new(
        DVec2::new(-f64::INFINITY, f64::INFINITY),
        DVec2::new(0.0, 0.0),
    );
    let bottom_left = DRect::new(
        DVec2::new(-f64::INFINITY, 0.0),
        DVec2::new(0.0, -f64::INFINITY),
    );
    let bottom_right = DRect::new(
        DVec2::new(0.0, 0.0),
        DVec2::new(f64::INFINITY, -f64::INFINITY),
    );

    // display those rectangles
    let positions = [top_right, top_left, bottom_left, bottom_right];
    let colors = [
        Color::from(RED),
        Color::from(GREEN),
        Color::from(MAGENTA),
        Color::from(YELLOW),
    ];
    assert_eq!(positions.len(), colors.len());

    for (rect, color) in positions.into_iter().zip(colors) {
        let x = rect.min.x.clamp(-window_width, window_width) as f32;
        let y = rect.min.y.clamp(-window_height, window_height) as f32;

        let width = x.abs().max(window_width as f32);
        let height = y.abs().max(window_height as f32);
        info!("NEW QUARTER CREATED | position: ({x}, {y}) ; size: {width} x {height}");

        cmds.spawn((
            Mesh2d(meshes.add(Rectangle::new(width, height))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x + width / 2.0, y - height / 2.0, 0.0)
                .with_scale(Vec3::splat(if SHOW_QUARTERS { 1.0 } else { 0.0 })),
            Quarter(rect),
            RenderLayers::layer(1),
        ));
    }
}

fn create_orbit_points(mut cmds: Commands, planets: Query<&Planet>, quarters: Query<&Quarter>) {
    let quarters_vec = quarters.into_iter().collect::<Vec<_>>();
    for planet in planets {
        let orbit_points = compute_orbit(planet, &quarters_vec);
        let orbit_points_scaled = orbit_points
            .iter()
            .map(|p| (p / DISTANCE_SCALE).as_vec2())
            .collect::<Vec<_>>();

        cmds.spawn(PlanetOrbit(orbit_points_scaled));
    }
}

fn draw_orbit_gizmos(mut gizmos: Gizmos, orbits: Query<&PlanetOrbit>) {
    for orbit in orbits {
        gizmos.linestrip_2d(
            orbit.0.clone(),
            Color::linear_rgba(1.0, 1.0, 1.0, LINE_ALPHA),
        );
    }
}

fn compute_orbit(planet: &Planet, quarters: &[&Quarter]) -> Vec<DVec2> {
    let mut planet = planet.clone();
    let mut last_quarter = 0; // we start in the first quarter

    let mut points = vec![];
    loop {
        // compute next position
        let dt = 60.0 * 60.0; // 1 simulated hour
        let (pos_new, vel_new) = velocity_verlet(dt, planet.position, planet.velocity, planet.mass);
        planet.position = pos_new;
        planet.velocity = vel_new;
        points.push(pos_new);

        // check in which quarter we are right now
        let mut current_quarter = 0;
        for (i, quarter) in quarters.iter().enumerate() {
            if quarter.0.contains(pos_new) {
                current_quarter = i;
                break;
            }
        }

        // check if we made full revolution
        if last_quarter != current_quarter && current_quarter == 0 {
            break;
        }
        last_quarter = current_quarter;
    }

    // lets say we want to take only 400 points, we need to calculate which nth element we want to take in each iteration
    let take_nth = points.len() / MAX_POINTS;
    info!("ORBIT POINTS | we take every {take_nth}th element");

    let mut filtered = every_nth_element(points, take_nth);
    // add starting point again so the line is closed loop
    filtered.push(filtered[0]);
    filtered
}

fn every_nth_element(mut values: Vec<DVec2>, n: usize) -> Vec<DVec2> {
    let mut first = true;
    let mut c = 0;

    values.retain(|_| {
        // always take the first element
        if first {
            first = false;
            return true;
        }

        c += 1;
        c % n == 0
    });

    values
}
