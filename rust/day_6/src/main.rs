extern crate common;
extern crate stopwatch;
#[macro_use]
extern crate itertools;

use stopwatch::Stopwatch;
use std::collections::HashMap;

fn main() {
    let mut sw = Stopwatch::start_new();
    let lines:Vec<String> = common::file_to_vector("input.txt")
        .expect("There was an error");

    let points: Vec<(usize, i32, i32)> = lines.iter().enumerate().map(|(i,l)| {
        let comma = l.find(',').unwrap();
        let x = l[0..comma].parse().unwrap();
        let y = l[comma+2..].parse().unwrap();
        (i,x,y)
    }).collect();

    let (min_x, max_x, min_y, max_y) = points.iter().fold((999999,0,999999,0), |(min_x, max_x, min_y, max_y), (_,x,y)| {
        
        let mut mx = min_x;
        let mut max = max_x;
        let mut my = min_y;
        let mut may = max_y;

        if *x < min_x {
            mx = *x;
        }

        if *x > max_x {
            max = *x;
        }

        if *y < min_y {
            my = *y;
        }

        if *y > max_y {
            may = *y;
        }

        (mx, max, my, may)
    });

    part_1(min_x, max_x, min_y, max_y, &points);
    part_2(min_x, max_x, min_y, max_y, &points);
    sw.stop();
    println!("Elapsed: {}", sw.elapsed_ms());
}

fn part_2(min_x: i32, max_x: i32, min_y: i32, max_y: i32, points: &Vec<(usize, i32, i32)>) {
    let result = iproduct!(min_x..max_x + 1, min_y..max_y + 1)
        .filter(|(x,y)| {
            let distance_sum:i32 = points.iter()
                .map(|(_,px,py)| {
                    (*px - x).abs() + (*py - y).abs()
                })
                .sum();

            distance_sum < 10000
        })
        .count();

    println!("Part 2: {}", result);
}

fn part_1(min_x: i32, max_x: i32, min_y: i32, max_y: i32, points: &Vec<(usize, i32, i32)>) {
    let edge_points: Vec<usize> = points.iter().filter(|(_,x,y)| {
        *x == min_x || *x == max_x || *y == min_y || *y == max_y 
    }).map(|(i,_,_)| *i).collect();

    let result: Vec<usize> = iproduct!(min_x..max_x + 1, min_y..max_y + 1)
        .filter_map(|(x, y)| {
            let mut min_val = 999999;
            let mut min_id = 0;
            let mut tie = false;

            for (i, px, py) in points {
                let d = (*px - x).abs() + (*py - y).abs();

                if d < min_val {
                    min_val = d;
                    min_id = *i;
                    tie = false;
                }
                else if d == min_val {
                    tie = true;
                }
            }

            if let Some(_) = edge_points.iter().find(|&i| *i == min_id){
                return None;
            }

            if !tie {
                return Some(min_id);
            }

            None
        }).collect();

    let mut set:HashMap<usize, usize> = HashMap::new();

    for r in result {
        *set.entry(r).or_insert(0) += 1;
    }

    let mut max_id = 0;
    let mut max_count = 0;

    for (i,c) in set {
        if c > max_count {
            max_count = c;
            max_id = i;
        }
    }

    println!("part 1: id:{} count: {}", max_id, max_count);
}
