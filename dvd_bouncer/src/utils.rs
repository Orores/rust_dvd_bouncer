use rand::Rng;

/// Generates a random index for selecting a color from a list.
pub fn generate_random_index(max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max) // Generate a random number within the range
}



