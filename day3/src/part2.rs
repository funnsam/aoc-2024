use crate::common::*;

pub fn solve(input: Input<'_>) -> usize {
    let re = regex::Regex::new(r"(mul|do|don\'t)\(\d*,?\d*\)").unwrap();
    let mut en = true;

    re.find_iter(input).map(|m| {
        let (m, a) = input[m.start()..].split_once('(').unwrap();
        let (a, b) = a.split_once(',').unwrap();
        let (b, _) = b.split_once(')').unwrap();

        if m == "mul" {
            en as usize * a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        } else {
            en = m == "do";
            0
        }
    }).sum()
}
