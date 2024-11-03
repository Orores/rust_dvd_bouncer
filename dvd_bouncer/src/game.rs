use ggez::{Context, GameResult};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawParam};
use ggez::input::keyboard::{self, KeyCode};
use nalgebra as na;
use ggez::mint::Point2;
use crate::graphics::{create_logo_meshes, draw_velocity_input_box, draw_apply_button};
use crate::utils::{should_change_color, parse_velocity_input, is_point_in_rect, calculate_next_position};

pub struct DVDLogo {
    position: na::Point2<f32>,
    velocity: f32,
    angle: f32,
    color_index: usize,
    logo_meshes: Vec<graphics::Mesh>,
    velocity_input: String,
}

impl DVDLogo {
    pub fn new(ctx: &mut Context) -> GameResult<DVDLogo> {
        let position = na::Point2::new(100.0, 100.0);
        let velocity = 10.0; // Default velocity
        let angle = std::f32::consts::FRAC_PI_4; // 1/4 pi, sin = 1, cos = 0

        let logo_meshes = create_logo_meshes(ctx)?;

        Ok(DVDLogo {
            position,
            velocity,
            angle,
            color_index: 0,
            logo_meshes,
            velocity_input: String::new(),
        })
    }

    pub fn update_position(&mut self) {
        let new_x = self.position.x + self.velocity * self.angle.cos();
        let new_y = self.position.y + self.velocity * self.angle.sin();
        
        // Check if the color should change
        if should_change_color(new_x, new_y) {
            self.change_color();
        }

        // Update the position with modulo logic
        self.position.x = new_x.rem_euclid(500.0);
        self.position.y = new_y.rem_euclid(500.0);
    }

    fn change_color(&mut self) {
        // Toggle between red (0) and blue (1)
        self.color_index = 1 - self.color_index;
    }

    fn apply_velocity(&mut self) {
        if let Some(new_velocity) = parse_velocity_input(&self.velocity_input) {
            self.velocity = new_velocity;
        }
    }
}

impl EventHandler for DVDLogo {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update_position();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        graphics::draw(
            ctx,
            &self.logo_meshes[self.color_index],
            DrawParam::default().dest(Point2 { x: self.position.x, y: self.position.y }),
        )?;

        // Draw the velocity input box and apply button
        draw_velocity_input_box(ctx, &self.velocity_input)?;
        draw_apply_button(ctx)?;

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

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            // Check if the click is within the apply button's area
            if is_point_in_rect((x, y), (220.0, 50.0), (80.0, 30.0)) {
                self.apply_velocity();
            }
        }
    }
}