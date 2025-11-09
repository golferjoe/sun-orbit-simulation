use core::{f32, f64};
use std::collections::VecDeque;

use bevy::{math::DVec2, prelude::*};

use crate::{
    math::{
        drect::DRect,
        physics::{scale_distance_to_bevy, velocity_verlet},
    },
    planet::Planet,
    ui::egui::Gui,
};

const UPDATE_INTERVAL: f32 = 0.5; // every how many seconds should we compute next orbit in queue
const MAX_POINTS: usize = 128;
const LINE_WIDTH: f32 = 2.0;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_gizmo_config(
            DefaultGizmoConfigGroup,
            GizmoConfig {
                line: GizmoLineConfig {
                    width: LINE_WIDTH,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .insert_resource(OrbitUpdateTimer::new())
        .add_systems(Startup, create_quarters)
        .add_systems(PostStartup, init_orbits)
        .add_systems(Update, (update_orbits, draw_orbit_gizmos));
    }
}

#[derive(Resource)]
struct UpdateQueue(VecDeque<Entity>);

#[derive(Resource)]
struct OrbitUpdateTimer(Timer);

impl OrbitUpdateTimer {
    fn new() -> Self {
        Self(Timer::from_seconds(UPDATE_INTERVAL, TimerMode::Repeating))
    }
}

#[derive(PartialEq, Component)]
struct Quarter {
    // Y direction (-1, 1) in which the current point will be ahead of starting position
    direction: DVec2,
    bounds: DRect,
}

fn create_quarters(mut cmds: Commands) {
    let quarters = [
        // top right
        Quarter {
            direction: DVec2::new(-1.0, 0.0),
            bounds: DRect::new(
                DVec2::new(0.0, f64::INFINITY),
                DVec2::new(f64::INFINITY, 0.0),
            ),
        },
        // bottom right
        Quarter {
            direction: DVec2::new(0.0, 1.0),
            bounds: DRect::new(
                DVec2::new(0.0, 0.0),
                DVec2::new(f64::INFINITY, -f64::INFINITY),
            ),
        },
        // bottom left
        Quarter {
            direction: DVec2::new(1.0, 0.0),
            bounds: DRect::new(
                DVec2::new(-f64::INFINITY, 0.0),
                DVec2::new(0.0, -f64::INFINITY),
            ),
        },
        // top left
        Quarter {
            direction: DVec2::new(0.0, -1.0),
            bounds: DRect::new(
                DVec2::new(-f64::INFINITY, f64::INFINITY),
                DVec2::new(0.0, 0.0),
            ),
        },
    ];

    for quarter in quarters {
        cmds.spawn(quarter);
    }
}

fn init_orbits(
    mut cmds: Commands,
    planets: Query<(Entity, &mut Planet)>,
    quarters: Query<&Quarter>,
) {
    // first we compute first orbit for each planet and then insert them into the queue for next updates
    let mut queue = VecDeque::new();
    let quarters_vec = quarters.into_iter().collect::<Vec<_>>();

    for (entity, mut planet) in planets {
        let orbit_points = compute_orbit(&planet, &quarters_vec);
        let orbit_points_scaled = orbit_points
            .into_iter()
            .map(|p| scale_distance_to_bevy(p))
            .map(|p| Vec3::new(p.x, 0.0, p.y))
            .collect::<Vec<_>>();

        planet.orbit_points = orbit_points_scaled;

        queue.push_back(entity);
    }

    cmds.insert_resource(UpdateQueue(queue));
}

fn update_orbits(
    time: Res<Time>,
    mut timer: ResMut<OrbitUpdateTimer>,
    mut queue: ResMut<UpdateQueue>,
    mut planets: Query<&mut Planet>,
    quarters: Query<&Quarter>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let quarters_vec = quarters.into_iter().collect::<Vec<_>>();

        if let Some(entity) = queue.0.pop_front()
            && let Ok(mut planet) = planets.get_mut(entity)
        {
            let orbit_points = compute_orbit(&planet, &quarters_vec);
            let orbit_points_scaled = orbit_points
                .into_iter()
                .map(|p| scale_distance_to_bevy(p))
                .map(|p| Vec3::new(p.x, 0.0, p.y))
                .collect::<Vec<_>>();

            planet.orbit_points = orbit_points_scaled;
            queue.0.push_back(entity);
        }
    }
}

fn draw_orbit_gizmos(settings: Res<Gui>, mut gizmos: Gizmos, planets: Query<&Planet>) {
    if settings.show_orbits {
        for planet in planets {
            gizmos.linestrip(
                planet.orbit_points.clone(),
                Color::linear_rgba(0.05, 0.05, 0.05, 1.0),
            );
        }
    }
}

fn find_quarter_idx(pos: DVec2, quarters: &[&Quarter]) -> usize {
    for (i, quarter) in quarters.iter().enumerate() {
        if quarter.bounds.contains(pos) {
            return i;
        }
    }
    assert!(false, "point not inside any quarter");
    0
}

fn compute_orbit(planet: &Planet, quarters: &[&Quarter]) -> Vec<DVec2> {
    let sub_quarter_overflow = |idx: usize| -> usize {
        if idx > 0 && idx != 1 {
            idx - 1
        } else {
            quarters.len() - 1
        }
    };
    let add_quarter_overflow = |idx: usize| -> usize {
        if idx >= quarters.len() - 1 {
            0
        } else {
            idx + 1
        }
    };

    let mut planet = planet.clone();
    let starting_position = planet.position;

    let starting_quarter = find_quarter_idx(planet.position, quarters);
    let mut last_quarter = starting_quarter;
    let mut first = true;

    let mut points = vec![];
    loop {
        // compute next position
        let dt = 60.0 * 60.0; // 1 simulated hour
        let (pos_new, vel_new) = velocity_verlet(dt, planet.position, planet.velocity, planet.mass);
        planet.position = pos_new;
        planet.velocity = vel_new;
        points.push(pos_new);

        // check if we made full revolution
        let current_quarter = find_quarter_idx(pos_new, quarters);
        if current_quarter == starting_quarter
            && last_quarter == add_quarter_overflow(starting_quarter)
        {
            first = false;
        }

        last_quarter = current_quarter;
        if !first && current_quarter == sub_quarter_overflow(starting_quarter) {
            break;
        }
    }

    let mut last_point = None;
    for (i, point) in points.clone().into_iter().enumerate().rev() {
        let quarter = find_quarter_idx(point, quarters);

        if quarter == last_quarter {
            points.remove(i);
        } else if quarter == starting_quarter {
            let delta = point - starting_position;
            let direction = quarters[quarter].direction;

            let x_positive = delta.x.is_sign_positive();
            let y_positive = delta.y.is_sign_positive();
            if (x_positive && direction.x == 1.0)
                || (!x_positive && direction.x == -1.0)
                || (y_positive && direction.y == 1.0)
                || (!y_positive && direction.y == -1.0)
            {
                last_point = Some(point);
                points.remove(i);
            }
        } else {
            break;
        }
    }

    let take_nth = points.len() / MAX_POINTS;
    let mut temp = every_nth_element(points, take_nth);
    if let Some(p) = last_point {
        temp.push(p);
    }
    temp
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
