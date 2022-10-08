use std::{cell::RefCell, rc::Rc};

use crate::constants::TOTAL_PUZZLE_SIZE;
use crate::direction::Direction;
use crate::solver::{Solve, SolvedState};
use crate::node::{Node,FindMovableNodes};

pub(crate) struct AStar {
    ending_node: Option<Node>,
    pub is_ending_node_reached: bool,
}

impl AStar {
    pub fn new() -> AStar {
        AStar {
            ending_node: Option::None,
            is_ending_node_reached: false
        }
    }
}

impl Default for AStar {
    fn default() -> Self {
        Self::new()
    }
}

impl Solve for AStar {
    fn solve(&mut self, puzzle_state: &mut [i8; TOTAL_PUZZLE_SIZE]) -> SolvedState {
        let mut open_list: Vec<Rc<RefCell<Node>>> = Vec::new();
        let mut closed_list: Vec<Rc<RefCell<Node>>> = Vec::new();
        let starting_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new(*puzzle_state)));

        open_list.push(starting_node);

        while !self.is_ending_node_reached {
            let (index, _) = open_list.iter().enumerate().min_by(|(_, a), (_, b)| { a.as_ref().borrow().value.cmp(&b.as_ref().borrow().value) }).unwrap();
            
            // Remember that remove has a runtime of O(n) as all elements after the index need to be shifted. 
            // Vec::swap_remove has a runtime of O(1) as it swaps the to-be-removed element with the last one.
            // If the order of elements is not important in your case, use swap_remove instead of remove!
            // https://stackoverflow.com/a/44012406/6134391

            closed_list.push(open_list.swap_remove(index));

            let min_value_node = closed_list.last().unwrap();

            for node in min_value_node.get_possible_nodes().into_iter() {
                if node.is_ending_node {
                    self.is_ending_node_reached = true;
                    self.ending_node = Some(node.clone());
                    break;
                }
                if !open_list.iter().any(|n| n.as_ref().borrow().puzzle_state.eq(&node.puzzle_state))
                && !closed_list.iter().any(|n| n.as_ref().borrow().puzzle_state.eq(&node.puzzle_state)) {
                    open_list.push(Rc::new(RefCell::new(node)));
                }
            }
            
            println!("openlist = {ol:?}| closedlist = {cl:?}", ol = open_list.len(), cl = closed_list.len());
        }

        let final_puzzle_state: [i8; TOTAL_PUZZLE_SIZE] = self.ending_node.as_ref().unwrap().puzzle_state;
        let mut node = Some(Rc::new(RefCell::new(self.ending_node.as_ref().unwrap().clone())));
        let mut moves: Vec<Direction> = Vec::new();

        '_move_loop: while node.as_ref().unwrap().as_ref().borrow().parent_node.is_some() {
            node = {
                let node_ = node.as_ref().unwrap().as_ref().borrow();
                moves.push(node_.direction);
                if node_.parent_node.is_none() {
                    break '_move_loop; 
                }
                node_.parent_node.clone()
            };
        }
        moves.reverse();

        SolvedState::new(final_puzzle_state, moves)
    }
}
