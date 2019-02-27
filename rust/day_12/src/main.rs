#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct Pot {
    index: i64,
    value: char
}

fn main() {
    let mut f = File::open("input.txt").expect("Could not open file");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("could not read file");
    let inp = contents;

    part_1(&inp);
    part_2();
    
}

fn part_1(inp: &str) {
    let (rest, st) = first_line(CompleteStr::from(&inp[..])).unwrap();
    let mut state: Vec<Pot> = st.chars().enumerate().map(|(i, c)| Pot {index: i as i64, value: c}).collect();

    let (_,map) = rows(rest).unwrap();

    print_pots(0, &state);
    for i in 0..20 {

        pad_pots(&mut state);

        let mut cp = state.clone();

        for idx in 2..cp.len() - 3 {
            let slc: String = (&state[idx - 2..idx + 3]).iter().map(|p| p.value).collect();
            if map.contains_key(&slc) {
                let r = map[&slc];
                cp[idx].value = r;
            }
            else {
                cp[idx].value = '.';
            }
        }

        state = cp;
        trim_pots(&mut state);
        print_pots(i + 1, &state);
    }
    let count = score_pots(&state);
    println!("part 1: {}", count);
}

fn part_2() {
    let start_count = 21010;
    let start_score = 1303275;
    let score_rate = 62; //if you let the loop run long enough it stabilizes at +62 per generation

    let score_diff = (50000000000i64 - start_count) * score_rate;
    let score = start_score + score_diff;
    println!("part 2: {}", score);
}

fn score_pots(pots: &Vec<Pot>) -> i64 {
    pots.iter().filter_map(|p| {
        if p.value == '#' {
            return Some(p.index)
        }

        None
    }).sum()
}

fn trim_pots(pots: &mut Vec<Pot>) {
    loop {
        if pots.first().unwrap().value == '.' {
            pots.remove(0);
        }
        else {
            break;
        }
    }

    loop {
        if pots.last().unwrap().value == '.' {
            pots.remove(pots.len() - 1);
        }
        else {
            break;
        }
    }
}

fn print_pots(iter: i64, pots: &Vec<Pot>) {
    let score = score_pots(pots);
    print!("{} ({}): ", iter, score);
    for p in pots {
        print!("{}", p.value);
    }

    println!("");
}

fn pad_pots(pots: &mut Vec<Pot>) {
    let f;
    let l;

    {
        f = pots.first().unwrap().index;
        l = pots.last().unwrap().index;
    }

    let mut id = f - 1;

    for _ in 0..4 {
        pots.insert(0, Pot {value: '.', index: id});
        id -= 1;
    }

    for i in l + 1 .. l + 5 {
        pots.push(Pot {value: '.', index: i});
    }
}

named!(first_line<CompleteStr, CompleteStr>, 
    preceded!(tag!("initial state: "), 
    take_until_and_consume!("\r\n")));

named!(inputs<CompleteStr, (CompleteStr, CompleteStr, CompleteStr, CompleteStr)>, 
    ws!(permutation!(take!(5), eat_separator!(" => "), take!(1), eat_separator!("\r\n"))));

named!(rows<CompleteStr, HashMap<String,char>>, map!(many1!(inputs), |v| to_hash_map(&v)));

fn to_hash_map(inp: &Vec<(CompleteStr, CompleteStr, CompleteStr, CompleteStr)>) -> HashMap<String, char> {
    let mut r = HashMap::new();

    for (k, _, v, _) in inp {
        r.insert(k.to_string(), v.chars().find(|_| true).unwrap());
    }

    r
}