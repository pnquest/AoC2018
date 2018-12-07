extern crate stopwatch;

use stopwatch::Stopwatch;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut sw = Stopwatch::start_new();
    let mut f = File::open("input.txt").expect("there was an error");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("there was an error");

    let input: Vec<char> = contents.chars().collect();

    part_1(&input);
    part_2(&input);
    sw.stop();
    println!("runtime: {}", sw.elapsed_ms());
}

fn part_2(input: &Vec<char>) {
    let mut min_len = 999999999;

//ascii A..Z
    for ci in 65..90 {
        let c = std::char::from_u32(ci).unwrap();
        let mut v = input.iter()
            .filter(|e| e.to_ascii_lowercase() != c.to_ascii_lowercase())
            .map(|e| *e).collect();

        let r = reduce(&v);

        if r.len() < min_len {
            min_len = r.len();
        }
    }

    println!("part 2: {}", min_len);
}

fn reduce_internal(input: &Vec<char>) -> Vec<char>{
    let mut res = vec![];
    let mut prev:Option<char> = None;
    for c in input {
        if let Some(ch) = prev {
            if c.to_ascii_lowercase() == ch.to_ascii_lowercase()
                && *c != ch {
                res.pop();
                prev = None;
            }
            else {
                res.push(*c);
                prev = Some(*c);
            }
        }
        else {
            res.push(*c);
            prev = Some(*c);
        }
    }

    res
}

fn reduce(input: &Vec<char>) -> Vec<char> {
    let mut res = input.clone();
    loop {
        let tmp = reduce_internal(&res);

        if tmp.len() < res.len() {
            res = tmp;
        }
        else {
            break;
        }
    }
    
    res
}

fn part_1(input: &Vec<char>) {
    let res = reduce(input);
    println!("part 1: {}", res.len());
}
