/// Determines whether the color should change based on the new position.
pub fn should_change_color(new_x: f32, new_y: f32, width: f32, height: f32) -> bool {
    new_x > width || new_y > height
}

/// Calculates the next position using modulo logic for the given box size.
pub fn calculate_next_position(
    x: f32, 
    y: f32, 
    velocity: f32, 
    angle: f32, 
    width: f32, 
    height: f32, 
    invert_x: f32, 
    invert_y: f32
) -> (f32, f32, f32, f32) {
    // Calculate new position based on velocity and angle
    let new_x = x + velocity * angle.cos();
    let new_y = y + velocity * angle.sin();

    // Determine if direction should be inverted
    let new_invert_x = if new_x <= 0.0 || new_x >= width { -invert_x } else { invert_x };
    let new_invert_y = if new_y <= 0.0 || new_y >= height { -invert_y } else { invert_y };
    println!("Invert x: {}", new_invert_x);
    println!("new x: {}", new_x);
    (new_x.rem_euclid(width), new_y.rem_euclid(height), new_invert_x, new_invert_y)
}

/// Validates and parses the velocity input.
pub fn parse_velocity_input(input: &str) -> Option<f32> {
    match input.trim().parse::<f32>() {
        Ok(velocity) if velocity > 0.0 => Some(velocity),
        _ => None, // Return None if the input is invalid or non-positive
    }
}

/// Checks if a given point is within a rectangular area.
pub fn is_point_in_rect(point: (f32, f32), rect_pos: (f32, f32), rect_size: (f32, f32)) -> bool {
    let (px, py) = point;
    let (rx, ry) = rect_pos;
    let (rw, rh) = rect_size;

    px >= rx && px <= rx + rw && py >= ry && py <= ry + rh
}

