extern crate rand;

use crate::{constants::TOTAL_PUZZLE_SIZE, direction::Direction};

pub struct SolvedState {
    pub puzzle_state: [i8; TOTAL_PUZZLE_SIZE],
    pub moves: Vec<Direction>,
}

impl SolvedState {
    pub fn new(puzzle_state: [i8; TOTAL_PUZZLE_SIZE], moves: Vec<Direction>) -> SolvedState {
        SolvedState {
            puzzle_state,
            moves,
        }
    }
}

pub trait Solve {
    fn solve(&mut self, puzzle_state: &mut [i8; TOTAL_PUZZLE_SIZE]) -> SolvedState;
}