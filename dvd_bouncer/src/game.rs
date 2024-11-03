use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam};
use ggez::input::keyboard::{self, KeyCode};
use nalgebra as na;
use ggez::mint::Point2;
use crate::graphics::{create_logo_meshes, draw_velocity_input};
use crate::utils::{generate_random_index, parse_velocity_input};

pub struct DVDLogo {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
    color_index: usize,
    logo_meshes: Vec<graphics::Mesh>,
    velocity_input: String,
}

impl DVDLogo {
    pub fn new(ctx: &mut Context) -> GameResult<DVDLogo> {
        let position = na::Point2::new(100.0, 100.0);
        let velocity = na::Vector2::new(10.0, 10.0); // Default velocity

        let logo_meshes = create_logo_meshes(ctx)?;

        Ok(DVDLogo {
            position,
            velocity,
            color_index: 0,
            logo_meshes,
            velocity_input: String::new(),
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
        self.color_index = generate_random_index(self.logo_meshes.len());
    }

    fn apply_velocity(&mut self) {
        if let Some(new_velocity) = parse_velocity_input(&self.velocity_input) {
            self.velocity = na::Vector2::new(new_velocity, new_velocity);
        }
    }
}

impl EventHandler for DVDLogo {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.apply_velocity();
        }
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

        // Draw the velocity input field
        draw_velocity_input(ctx, &self.velocity_input)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) {
        if character.is_digit(10) || character == '.' {
            self.velocity_input.push(character);
        }
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: keyboard::KeyMods, _repeat: bool) {
        if keycode == KeyCode::Back {
            self.velocity_input.pop();
        }
    }
}

