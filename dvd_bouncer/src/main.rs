mod game;
mod graphics;
mod utils;

use ggez::{ContextBuilder, GameResult};
use ggez::event;
use game::DVDLogo;
use std::env;
use std::path;

fn main() -> GameResult {
    // Add resource path for assets
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Define the actual window size
    let (actual_width, actual_height) = (800.0, 600.0); // Example size, you can adjust as needed

    // Define the rectangle size
    let (rectangle_width, rectangle_height) = (50.0, 30.0); // Based on the rectangle size in graphics.rs

    // Calculate the view size
    let view_width = actual_width + rectangle_width;
    let view_height = actual_height + rectangle_height;

    // Initialize the context and event loop
    let (mut ctx, event_loop) = ContextBuilder::new("dvd_logo", "author")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("DVD Bouncer"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(view_width, view_height))
        .build()?;

    // Create the game state with the actual window dimensions
    let state = DVDLogo::new(&mut ctx, actual_width, actual_height)?;

    // Main event loop
    event::run(ctx, event_loop, state)
}