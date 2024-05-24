use crate::frame::Pixel;
use crossterm::style::Color;

const BLOCK_CHAR: char = '▓';

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockKind {
    Yellow,
    Orange,
    Red,
    Cyan,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Block {
    kind: Option<BlockKind>,
    pub exploding: bool,
}

impl Block {
    pub fn new(kind: Option<BlockKind>) -> Self {
        Self {
            kind,
            exploding: false,
        }
    }

    pub fn to_pixel(&self) -> Pixel {
        use BlockKind::*;
        if self.exploding {
            return Pixel {
                grapheme: '*',
                color: Color::White,
                background: Color::Black,
            };
        }
        match self.kind {
            Some(Yellow) => Pixel {
                grapheme: BLOCK_CHAR,
                color: Color::AnsiValue(226),
                ..Pixel::default()
            },
            Some(Orange) => Pixel {
                grapheme: BLOCK_CHAR,
                color: Color::AnsiValue(214),
                ..Pixel::default()
            },
            Some(Red) => Pixel {
                grapheme: BLOCK_CHAR,
                color: Color::AnsiValue(196),
                ..Pixel::default()
            },
            Some(Cyan) => Pixel {
                grapheme: BLOCK_CHAR,
                color: Color::AnsiValue(51),
                ..Pixel::default()
            },
            None => Pixel::default(),
        }
    }

    pub fn update(&mut self, kind: Option<BlockKind>) {
        self.kind = kind;
    }

    pub fn empty(&self) -> bool {
        self.kind.is_none()
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        assert!(Block::default().empty());
    }
}
