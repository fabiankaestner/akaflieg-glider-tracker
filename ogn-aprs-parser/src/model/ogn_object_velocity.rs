use crate::ogn::utils::{fpm_to_m_s, knots_to_m_s, turn_rate_to_rad_s};

#[derive(Debug, PartialEq)]
pub struct OGNObjectVelocity {
    // Velocity in m/s
    pub horizontal: f32,
    // Altitude rate m/s
    pub vertical: Option<f32>,
    // Rotation in half-rotations/minute
    pub rotation: Option<f32>,
}

impl OGNObjectVelocity {
    // expects speed in knots, altitude rate in feet per minute and rotation rate in half-turns per minute
    pub fn new(speed: u32, altitude_rate: Option<i32>, rotation_rate: Option<f32>) -> Self {
        OGNObjectVelocity {
            horizontal: knots_to_m_s(speed as f32),
            vertical: match altitude_rate {
                Some(rate) => Some(fpm_to_m_s(rate as f32)),
                None => None,
            },
            rotation: match rotation_rate {
                Some(rate) => Some(turn_rate_to_rad_s(rate)),
                None => None,
            },
        }
    }
}
