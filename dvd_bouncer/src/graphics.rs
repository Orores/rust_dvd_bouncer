use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Mesh, Rect, Text, DrawParam, DrawMode};
use ggez::mint::Point2;

/// Creates a set of meshes for the DVD logo with red and blue colors.
pub fn create_logo_meshes(ctx: &mut Context) -> GameResult<Vec<Mesh>> {
    // Define red and blue colors for the logo
    let colors = vec![
        Color::from_rgb(255, 0, 0),   // Red
        Color::from_rgb(0, 0, 255),   // Blue
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

/// Renders the velocity input box on the screen.
pub fn draw_velocity_input_box(ctx: &mut Context, input: &str) -> GameResult<()> {
    // Draw the input box background
    let input_box = Mesh::new_rectangle(
        ctx,
        DrawMode::stroke(1.0),
        Rect::new(10.0, 50.0, 200.0, 30.0),
        Color::WHITE,
    )?;
    graphics::draw(ctx, &input_box, DrawParam::default())?;

    // Draw the current input text
    let input_display = Text::new(format!("Velocity: {}", input));
    graphics::draw(
        ctx,
        &input_display,
        DrawParam::default().dest(Point2 { x: 15.0, y: 55.0 }),
    )
    .map_err(|e| e.into()) // Ensure a proper GameResult is returned
}

/// Renders the apply button on the screen.
pub fn draw_apply_button(ctx: &mut Context) -> GameResult<()> {
    // Draw the button background
    let button = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        Rect::new(220.0, 50.0, 80.0, 30.0),
        Color::from_rgb(0, 128, 0), // Green button
    )?;
    graphics::draw(ctx, &button, DrawParam::default())?;

    // Draw the button text
    let button_text = Text::new("Apply");
    graphics::draw(
        ctx,
        &button_text,
        DrawParam::default().dest(Point2 { x: 230.0, y: 55.0 }),
    )
    .map_err(|e| e.into()) // Ensure a proper GameResult is returned
}