pub type Input<'a> = (Vec<(usize, usize)>, Vec<Vec<usize>>);

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    let (a, b) = input.split_once("\n\n").unwrap();
    let mut av = Vec::new();
    let mut bv = Vec::new();

    for l in a.lines() {
        let (a, b) = l.split_once('|').unwrap();
        av.push((a.parse().unwrap(), b.parse().unwrap()));
    }

    for l in b.lines() {
        bv.push(l.split(',').map(|i| i.parse().unwrap()).collect());
    }

    (av, bv)
}

pub fn incorrect_pos(rules: &[(usize, usize)], ord: &[usize]) -> Option<(usize, usize)> {
    for (a, b) in rules.iter() {
        let Some(ap) = ord.iter().position(|i| a == i) else { continue };
        let Some(bp) = ord.iter().position(|i| b == i) else { continue };

        if ap > bp {
            return Some((ap, bp));
        }
    }

    None
}
