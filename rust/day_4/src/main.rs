//omg, this is brutal... but it does run in 1s
//could be optimized by storing times as a hashmap of minutes with counts... yeah that would have been better

extern crate common;
extern crate regex;
extern crate stopwatch;
extern crate chrono;

use regex::Regex;
use stopwatch::Stopwatch;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use chrono::{DateTime, prelude::*};


struct Guard {
    id: u16,
    times: RefCell<Vec<u32>>
}

enum Event {
    BeginShift(u16, DateTime<Utc>),
    FallsAsleep(DateTime<Utc>),
    WakesUp(DateTime<Utc>)
}

fn parse_datetime(st:&str) -> DateTime<Utc> {
    let yr:i32 = st[0..4].parse().unwrap();
    let mnth:u32 = st[5..7].parse().unwrap();
    let day:u32 = st[8..10].parse().unwrap();
    let hour:u32 = st[11..13].parse().unwrap();
    let minute:u32 = st[14..16].parse().unwrap();
    Utc.ymd(yr, mnth, day).and_hms(hour, minute, 0)
}

fn main() {
    let guard_regex = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] Guard #(\d+) begins shift").unwrap();
    let other_regex = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (wakes up|falls asleep)").unwrap();

    let mut sw = Stopwatch::start_new();
    let lines:Vec<String> = common::file_to_vector("input.txt").unwrap();
    let mut events:Vec<Event> = lines.into_iter()
        .map(|l| {
            if l.ends_with("begins shift") {
                for c in guard_regex.captures_iter(&l) {
                    let dt:DateTime<Utc> = parse_datetime(&c[1]);
                    let id = (&c[2]).parse().unwrap();
                    return Event::BeginShift(id, dt);
                }
            }
            if l.ends_with("wakes up") {
                for c in other_regex.captures_iter(&l) {
                    let dt:DateTime<Utc> = parse_datetime(&c[1]);
                    return Event::WakesUp(dt);
                }
            }
            else {
                for c in other_regex.captures_iter(&l) {
                    let dt:DateTime<Utc> = parse_datetime(&c[1]);
                    return Event::FallsAsleep(dt);
                }
            }
            panic!("something bad happened");
        }).collect();

    events.sort_by(|e1, e2| {
        let dt1 = match e1 {
            Event::BeginShift(_, d) => d,
            Event::FallsAsleep(d) => d,
            Event::WakesUp(d) => d
        };

        let dt2 = match e2 {
            Event::BeginShift(_, d) => d,
            Event::FallsAsleep(d) => d,
            Event::WakesUp(d) => d
        };

        dt1.cmp(&dt2)
    });
    let mut guards:HashMap<u16, Rc<RefCell<Guard>>> = HashMap::new();
    let mut cur_guard:Option<Rc<RefCell<Guard>>> = None;
    let mut last_minute:u32 = 0;
    let mut last_event:Option<Event> = None;
    for evt in events {
        match evt {
            Event::BeginShift(id, dt) => {
                if let Some(cg) = cur_guard {
                    if let Some(Event::FallsAsleep(t)) = last_event {
                        for i in last_minute..t.time().minute() {
                            let g = cg.borrow_mut();
                            g.times.borrow_mut().push(i);
                        }
                    }
                }
                let g = guards.entry(id).or_insert(Rc::new(RefCell::new(Guard {
                    id:id, 
                    times:RefCell::new(vec![])
                })));
                cur_guard = Some(Rc::clone(&g));
                last_minute = dt.minute();
            },
            Event::FallsAsleep(dt) => {
                last_minute = dt.minute();
            },
            Event::WakesUp(dt) => {
                if let Some(cg) = &cur_guard {
                    for i in last_minute..dt.minute() {
                        let g = cg.borrow_mut();
                        g.times.borrow_mut().push(i);
                    }
                }
            }
        }

        last_event = Some(evt);
    }
    part_1(&guards);
    part_2(&guards);
    sw.stop();

    println!("Runtime: {}", sw.elapsed_ms());
}

fn part_1(guards: &HashMap<u16, Rc<RefCell<Guard>>>) {
    let max = guards.values().max_by_key(|g| {
        let grd = g.borrow();
        let tms = grd.times.borrow();
        tms.len()
    }).unwrap();
    let mut time_map: HashMap<u32, u32> = HashMap::new();
    let maxg = max.borrow();
    let tms = maxg.times.borrow();
    for v in tms.iter() {
        *time_map.entry(*v).or_insert(0) += 1
    }

    let (max_time,_) = time_map
        .into_iter()
        .max_by_key(|(_,v)| *v)
        .unwrap();
    println!("part 1: {}", max.borrow().id as u32 * max_time);
}

fn part_2(guards: &HashMap<u16, Rc<RefCell<Guard>>>) {
    let (g, m, _) = guards.values()
    .filter(|g| {
        let gg = g.borrow();
        let tms = gg.times.borrow();
        tms.len() > 0
    })
    .map(|g| {
        let mut mp:HashMap<u32, u32> = HashMap::new();
        let gg = g.borrow();
        let tms = gg.times.borrow();
        for v in tms.iter() {
            *mp.entry(*v).or_insert(0) += 1
        }

        let (mk, mv) = mp.iter().max_by_key(|(_,v)| *v).unwrap();

        let res = (gg.id as u32, *mk, *mv);
        res
    })
    .max_by_key(|(_,_,v)| *v)
    .unwrap();

    println!("part 2: {}", g  * m);
}
