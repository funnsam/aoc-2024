use crate::common::*;

pub fn solve(input: Input<'_>) -> usize {
    input.into_iter()
        .filter(|i| is_valid(&i))
        .count()
}
