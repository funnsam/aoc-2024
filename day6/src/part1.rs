use crate::common::*;

pub fn solve(input: Input<'_>) -> usize {
    gen_stat(&input.0, input.1).unwrap().len()
}
