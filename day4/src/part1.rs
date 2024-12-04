use crate::common::*;

pub fn solve(input: Input<'_>) -> usize {
    let mut count = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            macro_rules! m {
                ($xd:expr, $yd:expr) => {{
                    (y as isize + $yd * 3 < input.len() as isize && y as isize + $yd * 3 >= 0 && x as isize + $xd * 3 < input[0].len() as isize && x as isize + $xd * 3 >= 0) && ((
                        input[y][x] == b'X' &&
                        input[(y as isize + $yd * 1) as usize][(x as isize + $xd * 1) as usize] == b'M' &&
                        input[(y as isize + $yd * 2) as usize][(x as isize + $xd * 2) as usize] == b'A' &&
                        input[(y as isize + $yd * 3) as usize][(x as isize + $xd * 3) as usize] == b'S'
                    ) || (
                        input[y][x] == b'S' &&
                        input[(y as isize + $yd * 1) as usize][(x as isize + $xd * 1) as usize] == b'A' &&
                        input[(y as isize + $yd * 2) as usize][(x as isize + $xd * 2) as usize] == b'M' &&
                        input[(y as isize + $yd * 3) as usize][(x as isize + $xd * 3) as usize] == b'X'
                    ))
                }};
            }

            count += m!(1, 0) as usize;
            count += m!(0, 1) as usize;
            count += m!(1, 1) as usize;
            count += m!(1, -1) as usize;
        }
    }

    count
}
