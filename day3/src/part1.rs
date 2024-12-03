use crate::common::*;

pub fn solve(input: Input<'_>) -> usize {
    let re = regex::Regex::new("mul\\(\\d+,\\d+\\)").unwrap();

    re.find_iter(input).map(|m| {
        let (_, a) = input[m.start()..].split_once('(').unwrap();
        let (a, b) = a.split_once(',').unwrap();
        let (b, _) = b.split_once(')').unwrap();

        a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
    }).sum()
}
