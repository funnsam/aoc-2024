#!/bin/env sh

cargo new "day$1"
cd "day$1"

echo 'mod common;' >| src/main.rs
echo 'mod part1;' >> src/main.rs
echo 'mod part2;' >> src/main.rs
echo '' >> src/main.rs
echo 'fn main() {' >> src/main.rs
echo '    let input = std::fs::read_to_string("input.txt").unwrap();' >> src/main.rs
echo '    let input = common::parse(&input);' >> src/main.rs
echo '' >> src/main.rs
echo '    println!("Part 1: {}", part1::solve(&input));' >> src/main.rs
echo '    println!("Part 2: {}", part2::solve(&input));' >> src/main.rs
echo '}' >> src/main.rs

echo 'use crate::common::*;' >| src/part1.rs
echo '' >> src/part1.rs
echo "pub fn solve(input: &Input<'_>) -> isize {" >> src/part1.rs
echo '    todo!();' >> src/part1.rs
echo '}' >> src/part1.rs

cp src/part1.rs src/part2.rs

echo "pub type Input<'a> = &'a str;" >| src/common.rs
echo '' >> src/common.rs
echo "pub fn parse<'a>(input: &'a str) -> Input<'a> {" >> src/common.rs
echo '    input' >> src/common.rs
echo '}' >> src/common.rs

nvim input.txt src/part1.rs src/common.rs src/part2.rs
