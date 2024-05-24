use crate::{frame::Frame, NUM_COLS, NUM_ROWS, PIT_STARTING_X, WIDTH};
use crossterm::{cursor, style, terminal, QueueableCommand};
use std::io::{Stdout, Write};

#[derive(Debug)]
pub enum RendererError {
    Size,
    MinimumSize(usize, usize),
}

pub fn assert_screen_size() -> Result<(), RendererError> {
    let result = terminal::size().or(Err(RendererError::Size));

    if let Ok((cols, rows)) = result {
        if cols < WIDTH as u16 || rows < NUM_ROWS as u16 {
            return Err(RendererError::MinimumSize(WIDTH, NUM_ROWS));
        }
    } else {
        return Err(result.unwrap_err());
    }

    Ok(())
}

pub fn init(stdout: &mut Stdout) -> crossterm::Result<()> {
    stdout
        .queue(style::SetBackgroundColor(style::Color::AnsiValue(67)))?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(style::SetBackgroundColor(style::Color::Black))?;

    for x in 0..NUM_COLS {
        for y in 0..NUM_ROWS {
            stdout
                .queue(cursor::MoveTo((x + PIT_STARTING_X) as u16, y as u16))?
                .queue(style::Print(' '))?;
        }
    }

    stdout.flush()?;

    Ok(())
}

pub fn render(stdout: &mut Stdout, last_frame: &Frame, frame: &Frame) -> crossterm::Result<()> {
    for (x, col) in frame.iter().enumerate() {
        for (y, cell) in col.iter().enumerate() {
            if last_frame[x][y] == frame[x][y] {
                continue;
            }
            stdout
                .queue(cursor::MoveTo(x as u16, y as u16))?
                .queue(style::SetForegroundColor(cell.color))?
                .queue(style::SetBackgroundColor(cell.background))?
                .queue(style::Print(cell.grapheme))?;
        }
    }

    stdout.flush()?;

    Ok(())
}
