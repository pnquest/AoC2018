extern crate common;
use std::collections::HashSet;

fn main() {
    let rows = common::file_to_vector("input.txt")
        .unwrap();

    part_1(&rows);
    part_2(&rows);
}

fn part_1(values: &Vec<isize>) {
    let sum:isize = values.into_iter().sum();
    println!("part one result: {}", sum);
}

fn part_2(values: &Vec<isize>) {
    let mut set:HashSet<isize> = HashSet::new();
    let result = values
        .into_iter()
        .cycle()
        .scan(0, |state, &v| {
            *state = *state + v;
            Some(*state)
        })
        .find(|&v| !set.insert(v));

    match result {
        Some(v) => println!("part two result: {}", v),
        None => println!("There was an error")
    };
}