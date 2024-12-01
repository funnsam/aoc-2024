pub type Input<'a> = (Vec<usize>, Vec<usize>);

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    let mut l = vec![];
    let mut r = vec![];

    for i in input.lines() {
        l.push(i[..5].parse().unwrap());
        r.push(i[8..].parse().unwrap());
    }

    (l, r)
}
