use crate::common::*;

pub fn solve(input: &str) -> isize {
    let (mut l, mut r) = parse(input);

    l.sort_unstable();
    r.sort_unstable();

    l.into_iter().zip(r).map(|(l, r)| l.abs_diff(r)).sum::<usize>() as _
}
