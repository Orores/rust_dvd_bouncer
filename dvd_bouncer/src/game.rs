use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam};
use nalgebra as na;
use ggez::mint::Point2; // Corrected import to use ggez's re-exported mint
use crate::graphics::create_logo_meshes;
use crate::utils::generate_random_index;

pub struct DVDLogo {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
    color_index: usize,
    logo_meshes: Vec<graphics::Mesh>,
}

impl DVDLogo {
    pub fn new(ctx: &mut Context) -> GameResult<DVDLogo> {
        let position = na::Point2::new(100.0, 100.0);
        let velocity = na::Vector2::new(2.5, 2.0);

        // Create a set of precomputed meshes with different colors
        let logo_meshes = create_logo_meshes(ctx)?;

        Ok(DVDLogo {
            position,
            velocity,
            color_index: 0,
            logo_meshes,
        })
    }

    pub fn update_position(&mut self, ctx: &mut Context) {
        self.position += self.velocity;
        let screen_bounds = graphics::screen_coordinates(ctx);

        if self.position.x < screen_bounds.left() || self.position.x + 50.0 > screen_bounds.right() {
            self.velocity.x = -self.velocity.x;
            self.change_color();
        }

        if self.position.y < screen_bounds.top() || self.position.y + 30.0 > screen_bounds.bottom() {
            self.velocity.y = -self.velocity.y;
            self.change_color();
        }
    }

    fn change_color(&mut self) {
        // Change the color by updating the color index
        self.color_index = generate_random_index(self.logo_meshes.len());
    }
}

impl EventHandler for DVDLogo {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.update_position(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        graphics::draw(
            ctx,
            &self.logo_meshes[self.color_index],
            DrawParam::default().dest(Point2 { x: self.position.x, y: self.position.y }),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}