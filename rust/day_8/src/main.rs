#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::prelude::*;
use nom::digit;
use std::str;
use std::cell::RefCell;

struct Node {
    metadata: Vec<i32>,
    children: Vec<Node>
}

impl Node {
    fn full_sum(&self) -> i32 {
        let child_sums: i32 = self.children.iter()
            .map(|c| c.full_sum())
            .sum();

        let self_sum: i32 = self.metadata.iter().sum();

        self_sum + child_sums
    }

    fn node_value(&self) -> i32 {
        if self.children.len() == 0 {
            return self.metadata.iter().sum();
        }

        self.metadata.iter().filter_map(|m| {
            let idx = m - 1;

            if idx < 0 || idx as usize >= self.children.len() {
                return None;
            }

            Some(self.children[idx as usize].node_value())
        }).sum()
    }

    fn from_values(values: &RefCell<Vec<i32>>) -> Node {
        let num_children;
        let num_metadata; 

        { //needed to kill this borrow as soon as this is done
            let mut v = values.borrow_mut();
            num_children = v.remove(0);
            num_metadata = v.remove(0);
        }

        let mut children = Vec::<Node>::new();

        for _ in 0..num_children {
            children.push(Node::from_values(values));
        }

        let mut metadata = Vec::<i32>::new();

        { //needed to kill this borrow as soon as this is done
            let mut v = values.borrow_mut();

            for _ in 0..num_metadata {
                metadata.push(v.remove(0));
            }
        }

        Node{children: children, metadata: metadata}
    }
}

fn main() {
    let mut f = File::open("input.txt").expect("Could not open file");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("could not read file");

    let (_, parsed) = split(&contents).expect("could not parse input");
    let r = RefCell::new(parsed);
    let parent = Node::from_values(&r);

    part_1(&parent);
    part_2(&parent);
}

fn part_1(node: &Node) {
    let sum = node.full_sum();
    println!("Part 1: {}", sum);
}

fn part_2(node: &Node) {
    let value = node.node_value();
    println!("Part 2: {}", value);
}

named!(split<&str, Vec<i32>>, 
    map!(
        separated_list_complete!(char!(' '), digit), 
        |v| v.iter().map(|s| s.parse().unwrap()).collect()
    ));