use rand::{thread_rng, seq::SliceRandom};

mod constants;
mod solver;
mod direction;
mod node;
mod astar;
mod utils;

use constants::*;
use utils::create_array_with_increasing_value;
use crate::astar::AStar;
use crate::solver::{SolvedState, Solve};

fn main() {
    let mut puzzle_state: [i8; TOTAL_PUZZLE_SIZE] = create_puzzle_state();
    // let mut puzzle_state: [i8; TOTAL_PUZZLE_SIZE] = [5, 2, 3, 1, 8, 4, 6, 0, 7];

    let mut solver: AStar = AStar::new();
    let solved_state: SolvedState = solver.solve(&mut puzzle_state);

    println!("Puzzle state solved!");
    println!("{:?} moves!", solved_state.moves.len());
    println!("{:?}", solved_state.moves);
    println!("Final puzzle state {:?}!", solved_state.puzzle_state);
}

fn create_puzzle_state() -> [i8; TOTAL_PUZZLE_SIZE] {
    let mut puzzle_state: [i8; TOTAL_PUZZLE_SIZE] = create_array_with_increasing_value();

    let mut rng = thread_rng();
    puzzle_state.shuffle(&mut rng);

    while !check_solvability(&mut puzzle_state) {
        println!("Puzzle state not solvable: {:?}", puzzle_state);
        puzzle_state.shuffle(&mut rng);
    }
    println!("Solvable puzzle state found: {:?}", puzzle_state);
    
    puzzle_state
}

fn check_solvability(puzzle_state: &[i8; TOTAL_PUZZLE_SIZE]) -> bool {
    let temp = puzzle_state.iter().filter(|x| **x != MOVABLE_PIECE).collect::<Vec<_>>();
    let mut inversions: i32 = 0;
    for i in 0..temp.len() {
        for j in (i + 1)..temp.len() {
            if temp[i] > temp[j] {
                inversions += 1;
            }
        }
    }
    inversions % 2 == 0
}
