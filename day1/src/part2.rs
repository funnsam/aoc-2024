use crate::common::*;

pub fn solve(input: &str) -> isize {
    let (l, r) = parse(input);
    let mut s = 0;

    for l in l {
        s += l * r.iter().filter(|r| **r == l).count();
    }

    s as _
}
