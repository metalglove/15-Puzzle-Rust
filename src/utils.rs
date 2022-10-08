use crate::constants::TOTAL_PUZZLE_SIZE;

pub(crate) fn create_array_with_increasing_value() -> [i8; TOTAL_PUZZLE_SIZE] {
    core::array::from_fn(|i: usize| i as i8)
}