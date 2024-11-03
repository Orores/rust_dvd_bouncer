mod game;
mod graphics;
mod utils;

use ggez::{ContextBuilder, GameResult};
use game::DVDLogo;
use ggez::event;

fn main() -> GameResult {
    // Initialize the context and event loop
    let (mut ctx, event_loop) = ContextBuilder::new("dvd_logo", "author").build()?;
    
    // Create the game state
    let state = DVDLogo::new(&mut ctx)?;
    
    // Run the game loop
    event::run(ctx, event_loop, state)
}