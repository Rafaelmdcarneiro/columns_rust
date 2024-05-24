use crossterm::style::Color;

use crate::{NUM_COLS, NUM_ROWS, PIT_STARTING_X};

#[derive(Clone, Copy, PartialEq)]
pub struct Pixel {
    pub grapheme: char,
    pub color: Color,
    pub background: Color,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            grapheme: ' ',
            color: Color::Black,
            background: Color::Black,
        }
    }
}

pub type Frame = [[Pixel; NUM_ROWS]; NUM_COLS + PIT_STARTING_X];

pub fn new_frame() -> Frame {
    [[Pixel::default(); NUM_ROWS]; NUM_COLS + PIT_STARTING_X]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
