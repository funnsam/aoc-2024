use crate::common::*;

pub fn solve(input: Input<'_>) -> isize {
    input.into_iter()
        .filter(|i| {
            (0..i.len()).any(|j| {
                let mut i = i.clone();
                i.remove(j);
                is_valid(&i)
            })
        })
        .count() as _
}
