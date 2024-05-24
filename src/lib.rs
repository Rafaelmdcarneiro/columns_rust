pub mod block;
pub mod board;
pub mod column;
pub mod frame;
pub mod pit;
pub mod renderer;
pub mod terminal;
pub mod timer;

const NUM_COLS: usize = 6;
const NUM_ROWS: usize = 13;
const PIT_STARTING_X: usize = 10;
const WIDTH: usize = NUM_COLS + PIT_STARTING_X;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        Point { x: $x, y: $y }
    };
}
