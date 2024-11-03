use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Mesh, Rect};

/// Creates a set of meshes for the DVD logo with different colors.
pub fn create_logo_meshes(ctx: &mut Context) -> GameResult<Vec<Mesh>> {
    // Define a list of colors for the logo
    let colors = vec![
        Color::from_rgb(255, 0, 0),   // Red
        Color::from_rgb(0, 255, 0),   // Green
        Color::from_rgb(0, 0, 255),   // Blue
        Color::from_rgb(255, 255, 0), // Yellow
        Color::from_rgb(0, 255, 255), // Cyan
        Color::from_rgb(255, 0, 255), // Magenta
    ];

    // Create a mesh for each color
    let mut meshes = Vec::new();
    for color in colors {
        let mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, 50.0, 30.0),
            color,
        )?;
        meshes.push(mesh);
    }

    Ok(meshes) // Return the vector of meshes
}

