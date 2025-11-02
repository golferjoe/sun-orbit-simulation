#![allow(dead_code)]

pub const G: f64 = 6.6743e-11;
pub const SUN_MASS: f64 = 1.9885e30;

pub const EARTH_MASS: f64 = 5.972168e24; // kg
pub const EARTH_POS_X: f64 = 1.496e11; // 1 AU
pub const EARTH_VEL_Y: f64 = 29_782.7; // m/s

pub const VENUS_MASS: f64 = 4.8675e24;
pub const VENUS_POS_X: f64 = 1.0821e11; // 0.723332 AU
pub const VENUS_VEL_Y: f64 = 35020.0;

pub const DISTANCE_SCALE: f64 = 1.0e10;

pub const TO_1_DAY: f64 = 86_400.0;
pub const TO_1_MONTH: f64 = 2.6298e6;
pub const TO_1_YEAR: f64 = 3.15576e7;

pub const TIME_SCALE: f64 = TO_1_MONTH;

// values for rendering, they're not used for physics
pub const SUN_RADIUS: f32 = 3.0;
pub const EARTH_RADIUS: f32 = 1.5;
pub const VENUS_RADIUS: f32 = EARTH_RADIUS * 0.9499;
