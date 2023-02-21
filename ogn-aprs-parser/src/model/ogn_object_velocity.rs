#[derive(Debug, PartialEq)]
pub struct OGNObjectVelocity {
    // Velocity in knots
    horizontal: usize,
    // Altitude rate in feet/minute
    vertical: f32,
    // Rotation in half-rotations/minute
    rotation: usize,
}
