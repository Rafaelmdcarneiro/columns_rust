use crossterm::{
    event::{self, poll, Event, KeyCode},
    Result,
};
use rust_columns::{
    board::Board,
    column::Column,
    frame::{new_frame, Drawable, Frame},
    pit::Pit,
    renderer, terminal,
};
use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    renderer::assert_screen_size().expect("Failed when asserting the screen size requirements");
    // Drop guard for terminal setup and cleanup
    let mut _t = terminal::TerminalGuard::create();
    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel::<Frame>();
    let render_handle = thread::spawn(move || -> Result<()> {
        let mut stdout = io::stdout();
        let mut last_frame = new_frame();
        renderer::init(&mut stdout)?;
        while let Ok(curr_frame) = render_rx.recv() {
            renderer::render(&mut stdout, &last_frame, &curr_frame)?;
            last_frame = curr_frame;
        }
        Ok(())
    });

    let fps_duration = Duration::from_nanos(1_000_000_000 / 60); // 60 fps duration ~16ms
    let mut instant = Instant::now();
    let mut board = Board::default();
    let mut pit = Pit::default();
    let mut column = Column::new();
    let mut upcoming_column = Column::new();
    upcoming_column.stand_by = true;

    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        while poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        break 'gameloop;
                    }
                    KeyCode::Left => {
                        column.move_left(&pit.heap);
                    }
                    KeyCode::Right => {
                        column.move_right(&pit.heap);
                    }
                    KeyCode::Down => {
                        column.move_down(&pit.heap);
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        column.cycle();
                    }
                    _ => {}
                }
            }
        }

        let (score, blocks_score) = pit.update(&mut column, delta);
        // move column down if dropping, otherwise create a new one
        if pit.stable() {
            let dropping = column.update(&pit.heap, delta);
            // if the column landed already, renew it
            if !dropping {
                column = upcoming_column;
                column.stand_by = false;
                upcoming_column = Column::new();
                upcoming_column.stand_by = true;
            }
        }
        // keep track of scores, etc. in the board
        board.update(score, blocks_score);
        // draw elements on the current frame
        board.draw(&mut curr_frame);
        pit.draw(&mut curr_frame);
        column.draw(&mut curr_frame);
        upcoming_column.draw(&mut curr_frame);
        // render
        render_tx
            .send(curr_frame)
            .expect("Failed sending curr_frame to the render thread");

        if pit.topped_up() {
            // lose game
            break;
        }

        thread::sleep(fps_duration.saturating_sub(instant.elapsed()));
    }

    // Hygene
    drop(render_tx);
    render_handle.join().unwrap()?;

    Ok(())
}
