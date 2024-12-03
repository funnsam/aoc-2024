pub type Input<'a> = Vec<Vec<usize>>;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    input.lines().map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect()
}

pub fn is_valid(i: &[usize]) -> bool {
    let dir = i[0] < i[1];
    !i.windows(2).any(|j| {
        (j[0] < j[1]) != dir || !(1..=3).contains(&(j[0].abs_diff(j[1])))
    })
}
