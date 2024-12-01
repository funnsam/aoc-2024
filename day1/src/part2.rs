use crate::common::*;

pub fn solve((l, r): Input<'_>) -> isize {
    l.into_iter().map(|l| l * r.iter().filter(|r| **r == l).count()).sum::<usize>() as _
}
