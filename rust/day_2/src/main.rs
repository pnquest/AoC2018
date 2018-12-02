extern crate common;
#[macro_use]extern crate itertools;

use std::collections::HashMap;

#[derive(Clone)]
struct Counts {
    two_count: u32,
    three_count: u32
}

fn main() {
    let items = common::file_to_vector("./input.txt").unwrap();

    part_1(&items);
    part_2(&items);
}

fn part_1(items:&Vec<String>) {
    let mut totals = Counts{two_count:0,three_count:0};

    items.into_iter()
        .map(|v| {
            let mut grouped: HashMap<char, usize> = HashMap::new();

            v.chars().into_iter().fold(&mut grouped, |acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

            let (twos_and_threes, _):(Vec<usize>,_) = grouped.values().partition(|&c| *c == 2 || *c == 3);
            
            let two_count = match twos_and_threes.clone().into_iter().any(|c| c == 2){
                true => 1,
                false => 0
            };
            let three_count = match twos_and_threes.into_iter().any(|c| c == 3){
                true => 1,
                false => 0
            };

            (two_count, three_count)
        })
        .fold(&mut totals, |acc, c| {
            let (two, three) = c;
            acc.two_count += two;
            acc.three_count += three;

            acc
        });

    let cloned = totals.clone();

    println!("The checksum is {}", cloned.two_count * cloned.three_count);
}

fn part_2(items:&Vec<String>) {
    let items2 = items.clone().into_iter();

    let (sku1, sku2) = iproduct!(items.into_iter(), items2)
        .find(|(s1, s2)| {
            let chars1 = s1.chars();
            let chars2 = s2.chars();

            let diff = chars1
                .zip(chars2)
                .filter(|(c1, c2)| c1 != c2)
                .count();

            diff == 1
        }).unwrap();

    println!("The skus are {} and {}", sku1, sku2);

    let matching:String = sku1.chars().zip(sku2.chars()).filter(|(c1, c2)| c1 == c2).map(|(c1, _)| c1).collect();

    println!("The matching portion is {}", matching);
}