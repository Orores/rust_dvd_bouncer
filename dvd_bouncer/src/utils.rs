use rand::Rng;

/// Generates a random index for selecting a color from a list.
pub fn generate_random_index(max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max) // Generate a random number within the range
}

/// Validates and parses the velocity input.
pub fn parse_velocity_input(input: &str) -> Option<f32> {
    match input.trim().parse::<f32>() {
        Ok(velocity) if velocity > 0.0 => Some(velocity),
        _ => None, // Return None if the input is invalid or non-positive
    }
}

/// Calculates the next position using modulo logic for a 500x500 box.
pub fn calculate_next_position(x: f32, y: f32, velocity: f32, angle: f32) -> (f32, f32) {
    // Calculate new position based on velocity and angle
    let new_x = (x + velocity * angle.cos()).rem_euclid(500.0);
    let new_y = (y + velocity * angle.sin()).rem_euclid(500.0);
    (new_x, new_y)
}

/// Checks if a given point is within a rectangular area.
pub fn is_point_in_rect(point: (f32, f32), rect_pos: (f32, f32), rect_size: (f32, f32)) -> bool {
    let (px, py) = point;
    let (rx, ry) = rect_pos;
    let (rw, rh) = rect_size;

    px >= rx && px <= rx + rw && py >= ry && py <= ry + rh
}

