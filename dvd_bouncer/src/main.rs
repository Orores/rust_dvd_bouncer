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

    // Define the window size
    let (width, height) = (800.0, 600.0); // Example size, you can adjust as needed

    // Initialize the context and event loop
    let (mut ctx, event_loop) = ContextBuilder::new("dvd_logo", "author")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("DVD Bouncer"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(width, height))
        .build()?;

    // Create the game state with the window dimensions
    let state = DVDLogo::new(&mut ctx, width, height)?;

    // Main event loop
    event::run(ctx, event_loop, state)
}