use std::{rc::Rc, cell::RefCell};

use crate::filter_movable_nodes;
use crate::direction::Direction;
use crate::constants::{TOTAL_PUZZLE_SIZE, PUZZLE_SIZE, MOVABLE_PIECE};
use crate::utils::create_array_with_increasing_value;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Node {
    pub puzzle_state: [i8; TOTAL_PUZZLE_SIZE],
    pub length: i32,
    pub distance: i32,
    pub value: i32,
    pub direction: Direction,
    pub parent_node: Option<Rc<RefCell<Node>>>, 
    pub is_movable: bool,
    pub is_ending_node: bool,
}

impl Node {
    pub fn new(puzzle_state: [i8; TOTAL_PUZZLE_SIZE]) -> Self {
        let distance: i32 = Node::manhattan_distance(&puzzle_state);
        Self {
            puzzle_state,
            length: 0,
            distance,
            value: distance, // value is the distance + length
            direction: Direction::None,
            parent_node: None,
            is_movable: false,
            is_ending_node: false,
        }
    }

    pub fn new_with_parent(node: Rc<RefCell<Self>>, direction: Direction) -> Self {
        Self {
            puzzle_state: {let x = node.as_ref().borrow().puzzle_state; x},
            length: {let x = node.as_ref().borrow().length + 1; x},
            distance: 0,
            value: 0, // value is the distance + length
            direction,
            parent_node: Some(node),
            is_movable: false,
            is_ending_node: false,
        }
    }

    pub fn manhattan_distance(puzzle_state: &[i8; TOTAL_PUZZLE_SIZE]) -> i32 {
        let mut distance: i32 = 0;
        for (num, current_num) in puzzle_state.iter().enumerate() {
            let current_num_in_arr: i8 = num as i8;
            if current_num_in_arr != *current_num {
                let a: i32 = i32::abs(
                    (current_num_in_arr % PUZZLE_SIZE) as i32 - (*current_num % PUZZLE_SIZE) as i32,
                );
                let b: i32 = i32::abs(
                    (current_num_in_arr / PUZZLE_SIZE) as i32 - (*current_num / PUZZLE_SIZE) as i32,
                );
                distance += a;
                distance += b;
            }
        }
        distance
    }

    fn update(&mut self) {
        self.distance = Node::manhattan_distance(&self.puzzle_state);
        self.value = self.distance + self.length;
        self.is_movable = true;
    }
}

pub(crate) trait FindMovableNodes {
    fn get_possible_nodes(&self) -> Vec<Node>;
    fn left(&self) -> Node;
    fn right(&self) -> Node;
    fn up(&self) -> Node;
    fn down(&self) -> Node;
    fn is_out_of_bounds(current_position: usize, direction: Direction) -> bool;
    fn check_completion(node: &Node) -> bool;
}

impl FindMovableNodes for Rc<RefCell<Node>> {
    fn get_possible_nodes(&self) -> Vec<Node> {
        match self.as_ref().borrow().direction {
            Direction::Left => {
                filter_movable_nodes!([self.left(), self.up(), self.down()])
            },
            Direction::Right => {
                filter_movable_nodes!([self.right(), self.up(), self.down()])
            }
            Direction::Up => {
                filter_movable_nodes!([self.left(), self.right(), self.up()])
            },
            Direction::Down => {
                filter_movable_nodes!([self.left(), self.right(), self.down()])
            },
            Direction::None => {
                filter_movable_nodes!([self.left(), self.right(), self.up(), self.down()])
            },
        }
    }

    fn left(&self) -> Node {
        let mut left_node = Node::new_with_parent(self.clone(), Direction::Left);
        
        let movable_piece_location = left_node.puzzle_state.iter().position(|value| *value == MOVABLE_PIECE).unwrap();
        let left_of_movable_location: i8 = movable_piece_location as i8 - 1;

        if !Self::is_out_of_bounds(movable_piece_location, Direction::Left) {
            left_node.puzzle_state.swap(movable_piece_location, left_of_movable_location as usize);
            left_node.update();
        }

        if Self::check_completion(&left_node) {
            left_node.is_ending_node = true;
        }
        
        left_node
    }

    fn right(&self) -> Node {
        let mut right_node = Node::new_with_parent(self.clone(), Direction::Right);
        
        let movable_piece_location = right_node.puzzle_state.iter().position(|value| *value == MOVABLE_PIECE).unwrap();
        let right_of_movable_location: i8 = movable_piece_location as i8 + 1;

        if !Self::is_out_of_bounds(movable_piece_location, Direction::Right) {
            right_node.puzzle_state.swap(movable_piece_location, right_of_movable_location as usize);
            right_node.update();
        }

        if Self::check_completion(&right_node) {
            right_node.is_ending_node = true;
        }
        
        right_node
    }

    fn up(&self) -> Node {
        let mut up_node = Node::new_with_parent(self.clone(), Direction::Up);
        
        let movable_piece_location = up_node.puzzle_state.iter().position(|value| *value == MOVABLE_PIECE).unwrap();
        let up_of_movable_location: i8 = movable_piece_location as i8 - PUZZLE_SIZE;

        if !Self::is_out_of_bounds(movable_piece_location, Direction::Up) {
            up_node.puzzle_state.swap(movable_piece_location, up_of_movable_location as usize);
            up_node.update();
        }

        if Self::check_completion(&up_node) {
            up_node.is_ending_node = true;
        }
        
        up_node
    }

    fn down(&self) -> Node {
        let mut down_node = Node::new_with_parent(self.clone(), Direction::Down);
        
        let movable_piece_location = down_node.puzzle_state.iter().position(|value| *value == MOVABLE_PIECE).unwrap();
        let down_of_movable_location: i8 = movable_piece_location as i8 + PUZZLE_SIZE;

        if !Self::is_out_of_bounds(movable_piece_location, Direction::Down) {
            down_node.puzzle_state.swap(movable_piece_location, down_of_movable_location as usize);
            down_node.update();
        }

        if Self::check_completion(&down_node) {
            down_node.is_ending_node = true;
        }
        
        down_node
    }

    fn is_out_of_bounds(current_position: usize, direction: Direction) -> bool {
        let mut column = current_position as i8 % PUZZLE_SIZE;
        let mut row = current_position as i8 / PUZZLE_SIZE;
        match direction {
            Direction::Left => {
                column -= 1;
            },
            Direction::Right => {
                column += 1;
            },
            Direction::Up => {
                row -= 1;
            },
            Direction::Down => {
                row += 1;
            },
            Direction::None => {}
        };
        // !(0..PUZZLE_SIZE).contains(&column) || !(0..PUZZLE_SIZE).contains(&row) // stupid compiler prefers this (it is the same as below, on opt-level 3)
        column < 0 || column >= PUZZLE_SIZE || row < 0 || row >= PUZZLE_SIZE
    }

    fn check_completion(node: &Node) -> bool {
        node.puzzle_state.eq(&create_array_with_increasing_value())
    }
}


#[macro_export]
macro_rules! filter_movable_nodes {
    ($nodes:expr) => {{
        $nodes.into_iter().filter(|n| n.is_movable).collect::<Vec<_>>()
    }}
}
