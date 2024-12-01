use crate::common::*;

pub fn solve(input: Input<'_>) -> isize {
    let (mut l, mut r) = input.clone();

    l.sort_unstable();
    r.sort_unstable();

    l.into_iter().zip(r).map(|(l, r)| l.abs_diff(r)).sum::<usize>() as _
}
