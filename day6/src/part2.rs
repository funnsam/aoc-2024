use crate::common::*;

pub fn solve((mut map, guard): Input<'_>) -> usize {
    let locs = gen_stat(&map, guard.clone()).unwrap();
    let mut count = 0;

    for (l, _) in locs.iter() {
        if map[l.1][l.0] { continue; }

        map[l.1][l.0] = true;
        count += gen_stat(&map, guard.clone()).is_none() as usize;
        map[l.1][l.0] = false;
    }

    count
}
