#![allow(dead_code)]

pub const G: f64 = 6.6743e-11;
pub const SUN_MASS: f64 = 1.9885e30;

pub const MERCURY_MASS: f64 = 3.3011e23;
pub const MERCURY_POS_X: f64 = 69_820_000_000.0; // m
pub const MERCURY_VEL_Y: f64 = 38_860.0;

pub const VENUS_MASS: f64 = 4.8675e24;
pub const VENUS_POS_X: f64 = 1.0821e11;
pub const VENUS_VEL_Y: f64 = 35020.0;

pub const EARTH_MASS: f64 = 5.972168e24; // kg
pub const EARTH_POS_X: f64 = 1.496e11; // 1 AU
pub const EARTH_VEL_Y: f64 = 29_782.7; // m/s

pub const MARS_MASS: f64 = 6.4171e23;
pub const MARS_POS_X: f64 = 227_939_366.0 * 1000.0;
pub const MARS_VEL_Y: f64 = 24_070.0;

pub const JUPITER_MASS: f64 = 1.8982e27;
pub const JUPITER_POS_X: f64 = 7.78479e11;
pub const JUPITER_VEL_Y: f64 = 13_060.0;

pub const SATURN_MASS: f64 = 5.6834e26;
pub const SATURN_POS_X: f64 = 1.433530e12;
pub const SATURN_VEL_Y: f64 = 9_680.0;

pub const URANUS_MASS: f64 = 8.6810e25;
pub const URANUS_POS_X: f64 = 2.870972e12;
pub const URANUS_VEL_Y: f64 = 6_800.0;

pub const NEPTUNE_MASS: f64 = 1.02409e26;
pub const NEPTUNE_POS_X: f64 = 4.5e12;
pub const NEPTUNE_VEL_Y: f64 = 5_450.0;

pub const DISTANCE_SCALE: f64 = 0.5e10;

pub const TO_1_DAY: f64 = 86_400.0;
pub const TO_1_MONTH: f64 = 2.6298e6;
pub const TO_1_YEAR: f64 = 3.15576e7;

pub const TIME_SCALE: f64 = TO_1_MONTH;

// values for rendering, they're not used for physics
pub const SUN_RADIUS: f32 = 3.0;
pub const EARTH_RADIUS: f32 = 1.5;
pub const MERCURY_RADIUS: f32 = EARTH_RADIUS * 0.3829;
pub const VENUS_RADIUS: f32 = EARTH_RADIUS * 0.9499;
pub const MARS_RADIUS: f32 = EARTH_RADIUS * 0.533;
pub const JUPITER_RADIUS: f32 = 5.0;
pub const SATURN_RADIUS: f32 = 3.5;
pub const URANUS_RADIUS: f32 = 3.0;
pub const NEPTUNE_RADIUS: f32 = 2.5;
