pub fn parse(i: &str) -> (Vec<usize>, Vec<usize>) {
    let mut l = vec![];
    let mut r = vec![];

    for i in i.lines() {
        l.push(i[..5].parse().unwrap());
        r.push(i[8..].parse().unwrap());
    }

    (l, r)
}
