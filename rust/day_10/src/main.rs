
#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::prelude::*;
use nom::types::CompleteStr;

#[derive(Copy, Clone)]
struct Star {
    pos_x: i64,
    pos_y: i64,
    vel_x: i64,
    vel_y: i64
}

impl Star {
    fn move_star(&self) -> Star {
        Star {
            pos_x:self.pos_x + self.vel_x, 
            pos_y:self.pos_y + self.vel_y, 
            vel_x:self.vel_x, 
            vel_y:self.vel_y
        }
    }
}

fn main() {
    let mut f = File::open("input.txt").expect("Could not open file");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("could not read file");

    let mut stars = parse(&contents);
    let (w, h) = calc_size(&stars);
    let mut prev_w = w;
    let mut prev_h = h;
    let mut prev = stars.clone();
    let mut seconds = 0;

    loop {
        stars = step_values(&stars);
        let (tw, th) = calc_size(&stars);
        if tw > prev_w && th > prev_h {
            print_stars(&prev);
            break;
        }
        seconds += 1;
        prev = stars.clone();
        prev_w = tw;
        prev_h = th;
    }
    println!("Part 2: {}", seconds);
}

fn print_stars(stars: &Vec<Star>) {
    let (miw, maw, mih, mah) = calc_min_max(stars);

    for y in (mih - 1)..(mah + 2) {
        for x in (miw - 1)..(maw + 2) {
            let found = match stars.iter().find(|s| s.pos_x == x && s.pos_y == y) {
                Some(_) => true,
                None => false
            };

            if found {
                print!("*");
            } else {
                print!(".");
            }
        }

        println!("");
    }
}

fn step_values(stars: &Vec<Star>) -> Vec<Star> {
    stars.iter().map(|s| s.move_star()).collect()
}

fn calc_min_max(stars: &Vec<Star>) -> (i64, i64, i64, i64) {
    stars.iter().fold((999999999999, -999999999999, 999999999999, -999999999999), |(amiw, amaw, amih, amah), s| {
        let tmiw = if s.pos_x < amiw {
            s.pos_x
        }else {
            amiw
        };

        let tmaw = if s.pos_x > amaw {
            s.pos_x
        }else {
            amaw
        };

        let tmih = if s.pos_y < amih {
            s.pos_y
        } else {
            amih
        };

        let tmah = if s.pos_y > amah {
            s.pos_y
        } else {
            amah
        };

        (tmiw, tmaw, tmih, tmah)
    })
}

fn calc_size(stars: &Vec<Star>) -> (i64, i64) {
     let (miw, maw, mih, mah) = calc_min_max(stars);
    (maw - miw, mah - mih)
}

fn parse(input: &str) -> Vec<Star> {
    let (_, stars) = parse_stars(CompleteStr(input))
        .unwrap();
    stars
}


named!(first_number<CompleteStr,i64>, map!(
    delimited!(
        tag!("position=<"), 
        take_until!(","), 
        tag!(", ")), 
    |v| v.trim().parse().unwrap()));

named!(second_number<CompleteStr, i64>, map!(take_until!(">"), |v| v.trim().parse().unwrap()));

named!(third_number<CompleteStr, i64>, map!(
    delimited!(
        tag!("> velocity=<"), 
        take_until!(","), 
        tag!(", ")), 
    |v| v.trim().parse().unwrap()));

named!(fourth_number<CompleteStr, i64>, map!(terminated!(take_until_and_consume!(">"), opt!(tag!("\n"))), |v| v.trim().parse().unwrap()));

named!(parse_star<CompleteStr, Star>, 
    map!(
        permutation!(first_number, second_number, third_number, fourth_number), 
        |(px, py, vx, vy)| Star{pos_x: px, pos_y: py, vel_x: vx, vel_y: vy}));

named!(parse_stars<CompleteStr, Vec<Star>>, many1!(parse_star));