use crate::{
    frame::{Drawable, Frame, Pixel},
    NUM_ROWS,
};

#[derive(Default)]
pub struct Board {
    score: usize,
    blocks_score: usize,
}

impl Board {
    pub fn update(&mut self, score: usize, blocks_score: usize) {
        self.score = score;
        self.blocks_score = blocks_score;
    }
}

impl Drawable for Board {
    fn draw(&self, frame: &mut Frame) {
        for (pos, grapheme) in "Score".chars().enumerate() {
            frame[2 + pos][NUM_ROWS - 6] = Pixel {
                grapheme,
                color: crossterm::style::Color::White,
                ..Pixel::default()
            };
        }
        for (pos, grapheme) in format!("{}", self.score).chars().enumerate() {
            frame[2 + pos][NUM_ROWS - 5] = Pixel {
                grapheme,
                color: crossterm::style::Color::White,
                ..Pixel::default()
            };
        }
        for (pos, grapheme) in "Blocks".chars().enumerate() {
            frame[2 + pos][NUM_ROWS - 3] = Pixel {
                grapheme,
                color: crossterm::style::Color::White,
                ..Pixel::default()
            };
        }
        for (pos, grapheme) in format!("{}", self.blocks_score).chars().enumerate() {
            frame[2 + pos][NUM_ROWS - 2] = Pixel {
                grapheme,
                color: crossterm::style::Color::White,
                ..Pixel::default()
            };
        }
    }
}
