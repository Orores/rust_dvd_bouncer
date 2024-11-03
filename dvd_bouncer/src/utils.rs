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



