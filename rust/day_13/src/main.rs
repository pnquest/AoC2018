extern crate common;
use common::geometry::Point;

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
    pos_x: usize,
    pos_y: usize
}

impl Cart {
    fn new(f: Facing, x: usize, y: usize) -> Cart{
        Cart {faced: f, next_turn: NextTurn::Left, pos_x: x, pos_y: y}
    }

    fn trun_right(&mut self) {
        match self.faced {
            Facing::Up => self.faced = Facing::Right,
            Facing::Right => self.faced = Facing::Down,
            Facing::Down => self.faced = Facing::Left,
            Facing::Left => self.faced = Facing::Up
        }
    }

    fn turn_left(&mut self) {
        match self.faced {
            Facing::Up => self.faced = Facing::Left,
            Facing::Right => self.faced = Facing::Up,
            Facing::Down => self.faced = Facing::Right,
            Facing::Left => self.faced = Facing::Down
        }
    }

    fn move_cart(&mut self) {
        match self.faced {
            Facing::Up => self.pos_y -= 1,
            Facing::Down => self.pos_y += 1,
            Facing::Left => self.pos_x -= 1,
            Facing::Right => self.pos_x += 1
        };
    }

    fn turn_cart(&mut self, trk: &Track) {
        match trk {
            Track::CurveLTR => {
                match self.faced {
                    Facing::Up => self.trun_right(),
                    Facing::Down => self.trun_right(),
                    _ => self.turn_left()
                };
            },
            Track::CurveRTL => {
                match self.faced {
                    Facing::Up => self.turn_left(),
                    Facing::Down => self.turn_left(),
                    _ => self.trun_right()                
                };
            },
            Track::Cross => {
                match self.next_turn {
                    NextTurn::Left => {
                        self.turn_left();
                        self.next_turn = NextTurn::Straight;
                    },
                    NextTurn::Straight => self.next_turn = NextTurn::Right,
                    NextTurn::Right => {
                        self.trun_right();
                        self.next_turn = NextTurn::Left
                    }
                }
            },
            Track::Blank => panic!("off the rails!"),
            _ => {}
        };
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let mut tracks: Vec<Vec<Track>> = Vec::new();
    let mut carts:HashMap<Point, Cart> = HashMap::new();
    parse_track(&mut tracks, &mut carts);

    let mut last_site = None;

    while last_site.is_none() {
        last_site = move_carts_2(&tracks, &mut carts);
    }

    let ls = last_site.unwrap();

    println!("Part 2: {},{}", ls.x, ls.y);
}

fn part_1() {
    let mut tracks: Vec<Vec<Track>> = Vec::new();
    let mut carts:HashMap<Point, Cart> = HashMap::new();
    parse_track(&mut tracks, &mut carts);

    let mut crash_site = None;

    while crash_site.is_none() {
        //print_track(&tracks, &carts);
        crash_site = move_carts(&tracks, &mut carts);
    }

    let cr = crash_site.unwrap();

    println!("part 1: {}, {}", cr.x, cr.y);
}

fn print_track(tracks: &Vec<Vec<Track>>, carts: &HashMap<Point, Cart>) {
    for y in 0..tracks.len() {
        for x in 0..tracks[y].len() {
            let pt = Point{x:x, y:y};
            if carts.contains_key(&pt) {
                let ct = &carts[&pt];
                match ct.faced {
                    Facing::Down => print!("v"),
                    Facing::Up => print!("^"),
                    Facing::Left => print!("<"),
                    Facing::Right => print!(">")
                }
            }
            else {
                let tk = &tracks[y][x];
                match tk {
                    Track::Blank => print!(" "),
                    Track::Cross => print!("+"),
                    Track::CurveLTR => print!("/"),
                    Track::CurveRTL => print!("\\"),
                    Track::Horizontal => print!("-"),
                    Track::Vertical => print!("|")
                }
            }
        }
        println!();
    }
}

fn move_carts_2(tracks: &Vec<Vec<Track>>, carts: &mut HashMap<Point, Cart>) -> Option<Point> {
    let mut k: Vec<Point> = carts.keys().map(|k| *k).collect();
    k.sort();

    for p in k {
        if let Some(ct) = carts.remove(&p) {
            move_cart_2(tracks, carts, ct);    
        }
    }

    if carts.keys().count() == 1 {
        let pt = *carts.keys().find(|_| true).unwrap();
        return Some(pt);
    }

    None
}

fn move_cart_2(tracks: &Vec<Vec<Track>>, carts: &mut HashMap<Point, Cart>, ct: Cart) {
    let mut cart = ct;
    cart.move_cart();

    let trk = &tracks[cart.pos_y][cart.pos_x];
    cart.turn_cart(&trk);

    let pt = Point{x:cart.pos_x, y: cart.pos_y};

    if carts.contains_key(&pt) {
        carts.remove(&pt);
    }
    else {
        carts.insert(pt, cart);
    }
}

fn move_carts(tracks: &Vec<Vec<Track>>, carts: &mut HashMap<Point, Cart>) -> Option<Point> {
    let mut k: Vec<Point> = carts.keys().map(|k| *k).collect();
    k.sort();

    for p in k {
        let ct = carts.remove(&p).unwrap();
        match move_cart(tracks, carts, ct) {
            Some(pt) => return Some(pt),
            None => {}
        };
    }

    None
}

fn move_cart(tracks: &Vec<Vec<Track>>, carts: &mut HashMap<Point, Cart>, ct: Cart) -> Option<Point> {
    let mut cart = ct;
    cart.move_cart();

    let trk = &tracks[cart.pos_y][cart.pos_x];
    cart.turn_cart(&trk);

    let pt = Point{x:cart.pos_x, y: cart.pos_y};

    if carts.contains_key(&pt) {
        return Some(pt);
    }
    carts.insert(pt, cart);

    None
}

fn parse_track(tracks:&mut Vec<Vec<Track>>, carts: &mut HashMap<Point, Cart>) {
    let f = File::open("./input.txt").expect("could not open file");
    let mut x;
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
