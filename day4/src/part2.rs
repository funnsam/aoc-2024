use crate::common::*;

pub fn solve(input: Input<'_>) -> usize {
    let mut count = 0;

    for y in 1..input.len() - 1 {
        for x in 1..input[0].len() - 1 {
            count += (input[y][x] == b'A' && (
                (input[y - 1][x - 1] == b'M' && input[y + 1][x + 1] == b'S') ||
                (input[y - 1][x - 1] == b'S' && input[y + 1][x + 1] == b'M')
            ) && (
                (input[y + 1][x - 1] == b'M' && input[y - 1][x + 1] == b'S') ||
                (input[y + 1][x - 1] == b'S' && input[y - 1][x + 1] == b'M')
            )) as usize;
        }
    }

    count
}
