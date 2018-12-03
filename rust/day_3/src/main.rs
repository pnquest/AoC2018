extern crate common;
extern crate stopwatch;
extern crate regex;
use stopwatch::Stopwatch;
use regex::Regex;
use common::geometry::Rectangle;

#[derive(PartialEq, PartialOrd)]
struct Entry {
    id: usize,
    area: Rectangle
}

fn main() {
    let mut sw = Stopwatch::start_new();
    let input:Vec<String> = common::file_to_vector("input.txt").unwrap();
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let rects= input.into_iter().map(|s| convert_string(s, &re)).collect();
    part_1(&rects);
    part_2(&rects);
    sw.stop();
    println!("Runtime: {}", sw.elapsed_ms());
}

fn convert_string(s:String, re:&Regex) -> Entry {
     for cap in re.captures_iter(&s) {
            let id = (&cap[1]).parse().unwrap();
            let left = (&cap[2]).parse().unwrap();
            let top = (&cap[3]).parse().unwrap();
            let width = (&cap[4]).parse().unwrap();
            let height = (&cap[5]).parse().unwrap();

            return Entry {id: id, area: Rectangle::new(left, top, width, height)}
        }

        Entry{id: 0, area: Rectangle::new(0,0,0,0)}
}

fn part_1(entries:&Vec<Entry>){
    let mut overlap_area = 0;
    let mut overlaps:Vec<Rectangle> = vec![];

    for i in 0..entries.len() {
        let r = &entries[i].area;
        for j in i..entries.len() {
            let r2 = &entries[j].area;

            if r  != r2 {
                let overlap = r.intersect(&r2);

                match overlap {
                    Some(o) => {
                        overlap_area += find_subtractable_area(&o, &overlaps);
                        overlaps.push(o);
                    },
                    None => {}
                };
            }
        }
    }

    println!("Overlap area is: {}", overlap_area);
}

fn find_subtractable_area(over: &Rectangle, overlaps:&Vec<Rectangle>) -> usize {
    let mut area = over.get_area();
    let mut sub_overlaps: Vec<Rectangle> = vec![];

    for r in overlaps {
        match r.intersect(over) {
            Some(o) => {
                area -= find_subtractable_area(&o, &sub_overlaps);
                sub_overlaps.push(o);
            },
            None => {}
        };
    }

    area
}

fn part_2(entries:&Vec<Entry>){
    let no_overlap = entries.into_iter()
        .find(|&e| {
            let itr2 = entries.clone();

            itr2.into_iter().all(|e2| {
                if *e2 == *e {
                    return true;
                }

                match e.area.intersect(&e2.area) {
                    None => true,
                    Some(_) => false
                }
            })
        }).unwrap();

    println!("The id with no overlap is {}", no_overlap.id);
}
