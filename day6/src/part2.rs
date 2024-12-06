use crate::common::*;

pub fn solve((mut map, guard): Input<'_>) -> usize {
    let locs = gen_stat(&map, guard.clone()).unwrap();
    let mut count = 0;

    for (l, _) in locs.iter() {
        if map[l.1][l.0] { panic!() }
        map[l.1][l.0] = true;

        if gen_stat(&map, guard.clone()).is_none() {
            println!("{l:?}");
            count += 1;
        }

        map[l.1][l.0] = false;
    }

    count
}
