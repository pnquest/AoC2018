extern crate common;
extern crate stopwatch;


use std::char;
use std::cell::RefCell;

struct Step {
    step_id: char,
    is_completed: bool,
    is_started: bool,
    depends_on: RefCell<Vec<usize>>
}

impl Step {
    fn new(id: char) -> Step {
        Step {step_id: id, is_completed: false, is_started: false, depends_on: RefCell::new(vec![])}
    }

    fn is_ready(&self, nodes: &Vec<RefCell<Step>>) -> bool {
            if self.is_completed || self.is_started {
                return false;
            }

            let deps = self.depends_on.borrow();

            deps.iter().all(|d| nodes[*d].borrow().is_completed)
    }
}

#[derive(Copy, Clone)]
struct Worker {
    step_index: Option<usize>,
    time_remaining: Option<usize>
}

impl Worker {
    fn new() -> Worker {
        Worker{step_index: None, time_remaining: None}
    }

    fn tick_time(&mut self) -> Option<usize> {
        match self.time_remaining {
            Some(t) => {
                let new_t = t - 1;
                if new_t == 0 {
                    let old_index = self.step_index;
                    self.step_index = None;
                    self.time_remaining = None;
                    return Some(old_index.unwrap());
                }
                self.time_remaining = Some(t - 1)
            },
            None => panic!("I can't do that, dave")
        };

        None
    }

    fn assign_work(&mut self, step_index: usize, time: usize) {
        self.step_index = Some(step_index);
        self.time_remaining = Some(time);
    }
}

fn main() {
    let lines:Vec<String> = common::file_to_vector("input.txt").unwrap();

    part_1(&lines);
    part_2(&lines);
}

fn part_2(lines: &Vec<String>) {
    let workers = vec![RefCell::new(Worker::new());5];

    let mut nodes = Vec::new();
    let mut completed:Vec<char> = vec![];

    fill_nodes(&lines, &mut nodes);

    let mut time_elapsed = 0;

    while nodes.iter().any(|s| !s.borrow().is_completed) {
        let num_free_workers = workers
            .iter()
            .filter(|w| w.borrow().step_index.is_none())
            .count();

        for _i in 0..num_free_workers {
            let op = nodes
                .iter()
                .enumerate()
                .find(|(_,st)| st.borrow().is_ready(&nodes));

            match op {
                Some((idx, nd)) => find_and_assign_worker(idx, nd, &workers),
                None => break
            }
        }

        time_elapsed += 1;


        for w in &workers {
            let mut wr = w.borrow_mut();
            if !wr.step_index.is_none() {
                let r = wr.tick_time();

                if let Some(id) = r {
                    let mut done_node = nodes[id].borrow_mut();
                    done_node.is_completed = true;
                    completed.push(done_node.step_id);
                }
            }
        }        
    }
    let res:String = completed.iter().collect();
    println!("Part 2: {} ({})", time_elapsed, res);
}

fn find_and_assign_worker(idx: usize, nd: &RefCell<Step>, workers: &Vec<RefCell<Worker>>) {
    let opw = workers.iter()
        .find(|w| w.borrow().step_index.is_none());

    if let Some(wrk) = opw {
        let mut node = nd.borrow_mut();
        node.is_started = true;
        let mut wr = wrk.borrow_mut();
        wr.assign_work(idx, 60 + idx + 1);
    }
}

fn part_1(lines: &Vec<String>) {
    let mut nodes = vec![];
    fill_nodes(lines, &mut nodes);

    let mut completed:Vec<char> = vec![];

    while nodes.iter().any(|s| !s.borrow().is_completed) {
        let next = nodes.iter().find(|st| st.borrow().is_ready(&nodes)).unwrap();
        
        let mut nxt = next.borrow_mut();
        completed.push(nxt.step_id);
        nxt.is_completed = true;
    }

    let result:String = completed.iter().collect();
    println!("part 1: {}", result);
}

fn fill_nodes(lines: &Vec<String>, nodes: &mut Vec<RefCell<Step>>) {
    for cd in 65..91 {
        let c = cd as u8 as char;
        let st = Step::new(c);
        nodes.push(RefCell::new(st));
    } 

    let offset = 'A' as u8;

    for l in lines {
        let parent = l.chars().nth(5).unwrap();
        let child = l.chars().nth(36).unwrap();

        let parent_node = (parent as u8 - offset) as usize;
        let child_node = (child as u8 - offset) as usize;

        let mut nd = nodes[child_node].borrow_mut();
        let mut pt = nd.depends_on.borrow_mut();
        pt.push(parent_node);
    }
}
