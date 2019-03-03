use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

enum Track {
    Vertical,
    Horizontal,
    CurveLTR,
    CurveRTL,
    Cross,
    Blank
}

enum Facing {
    Up,
    Down,
    Left,
    Right
}

enum NextTurn {
    Left,
    Straight,
    Right
}

struct Cart {
    faced: Facing,
    next_turn: NextTurn,
    pos_x: i64,
    pos_y: i64
}

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64
}

impl Cart {
    fn new(f: Facing, x: i64, y: i64) -> Cart{
        Cart {faced: f, next_turn: NextTurn::Left, pos_x: x, pos_y: y}
    }
}

fn main() {
    let mut tracks: Vec<Vec<Track>> = Vec::new();
    let mut carts:HashMap<Point, Cart> = HashMap::new();
    parse_track(&mut tracks, &mut carts);
}

fn parse_track(tracks:&mut Vec<Vec<Track>>, carts: &mut HashMap<Point, Cart>) {
    let f = File::open("./input.txt").expect("could not open file");
    let mut x = 0;
    let mut y = 0;

    for line in BufReader::new(f).lines() {
        x = 0;
        tracks.push(Vec::new());
        let inner = tracks.last_mut().unwrap();
        for c in line.unwrap().chars() {
            let t = match c {
                '-' => Track::Horizontal,
                '|' => Track::Vertical,
                '/' => Track::CurveLTR,
                '\\' => Track::CurveRTL,
                '+' => Track::Cross,
                ' ' => Track::Blank,
                '>' => {
                    let c = Cart::new(Facing::Right, x, y);
                    let p = Point{x:x, y:y};
                    carts.insert(p, c);
                    Track::Horizontal
                },
                '<' => {
                    let c = Cart::new(Facing::Left, x, y);
                    let p = Point{x:x, y:y};
                    carts.insert(p, c);
                    Track::Horizontal
                },
                'v' => {
                    let c = Cart::new(Facing::Down, x, y);
                    let p = Point{x:x, y:y};
                    carts.insert(p, c);
                    Track::Vertical
                },
                '^' => {
                    let c = Cart::new(Facing::Up, x, y);
                    let p = Point{x:x, y:y};
                    carts.insert(p, c);
                    Track::Vertical
                },
                _ => panic!("could not parse this character")
            };

            inner.push(t);
            x += 1;
        }
        y+= 1;
    }
}
