use std::collections::HashMap;

pub type Coord = (usize, usize);
pub type Velocity = (isize, isize);
pub type Input<'a> = (Vec<Vec<bool>>, Coord);

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    let mut guard = (0, 0);
    let mut map = Vec::new();

    for l in input.lines() {
        let mut m = Vec::new();

        for c in l.as_bytes() {
            if *c == b'^' { guard = (m.len(), map.len()); }
            m.push(*c == b'#');
        }

        map.push(m);
    }

    (map, guard)
}

pub fn gen_stat(map: &[Vec<bool>], mut guard: Coord) -> Option<HashMap<Coord, Vec<Velocity>>> {
    let mut locs: HashMap<Coord, Vec<Velocity>> = HashMap::new();
    let mut gv = (0, -1);

    while let Some(c) = map.get((guard.1 as isize + gv.1) as usize).and_then(|y| y.get((guard.0 as isize + gv.0) as usize)) {
        if *c {
            gv = (-gv.1, gv.0);
        } else {
            guard.0 = (guard.0 as isize + gv.0) as usize;
            guard.1 = (guard.1 as isize + gv.1) as usize;
        }

        if let Some(l) = locs.get_mut(&guard) {
            if l.contains(&gv) {
                return None;
            }

            l.push(gv.clone());
        } else {
            locs.insert(guard.clone(), vec![gv.clone()]);
        }
    }

    Some(locs)
}
