use geometry::Point;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum Space {
    Start(Point),
    Open(Point),
    Blocked(Point),
    Goal(Point)
}

impl Space {
    fn extract(&self) -> Point {
        *match self {
            Space::Start(v) => v,
            Space::Open(v) => v,
            Space::Blocked(v) => v,
            Space::Goal(v) => v
        }
    }
}

pub type Map = Vec<Vec<Space>>;

#[derive(Clone)]
struct Node {
    coordinate: Space,
    from_start: usize,
    to_goal: usize,
    parent: Option<Rc<Node>>
}

impl Node {
    fn score(&self) -> usize {
        self.from_start + self.to_goal
    }

    fn new(coord: Space, from_start: usize, to_goal: usize, parent: Option<&Rc<Node>>) -> Node {
        let prnt = match parent {
            Some(nd) => Some(Rc::clone(nd)),
            None => None
        };
        Node{coordinate: coord, from_start:from_start, to_goal:to_goal, parent: prnt}
    }
}

enum SearchResult {
    Candidate(Node),
    Found(Node),
    None
}

fn unwrap_nodes(node: &Rc<Node>) -> Vec<Point> {
    let mut ret = Vec::new();

    ret.push(node.coordinate.extract());
    if let Some(ref n) = node.parent {
        ret.append(&mut unwrap_nodes(n));
    }
    ret
}

pub fn solve_map(map: &Map, start: Point, goal: Point) -> Option<Vec<Point>> {
    let start_space = map[start.x][start.y];
    
    let start = Node::new(start_space, 0, start.manhattan_distance(&goal), None);

    let max_x = map.len() as isize;
    let max_y = map[0].len() as isize;
    let moves_vec = vec![-1,1,0];

    let mut closed: Vec<Rc<Node>> = Vec::new();
    let mut open = vec![Rc::new(start)];

    loop {
        let cloned = open.clone();
        let smallest = cloned.iter()
            .enumerate()
            .min_by_key(|(_, x)| x.score());

        if let Some((idx, val)) = smallest {
            open.remove(idx);
            let pt = val.coordinate.extract();
            for x in moves_vec.clone() {
                for y in -1..=1 {
                    if (x == 0 || y == 0) 
                        && !(x == 0  && y == 0)
                        && x + pt.x as isize >= 0 
                        && y + pt.y as isize >= 0 
                        && x + (pt.x as isize) < max_x
                        && y + (pt.y as isize) < max_y{
                        
                        let sp = map[(x + pt.x as isize) as usize][(y + pt.y as isize) as usize];

                        let nd = match sp {
                            Space::Open(p) => SearchResult::Candidate(Node::new(sp, val.from_start + 1, p.manhattan_distance(&goal), Some(&val))),
                            Space::Goal(_) => SearchResult::Found(Node::new(sp, val.from_start + 1, 0, Some(&val))),
                            _ => SearchResult::None
                        };

                        match nd {
                            SearchResult::Candidate(n) => {
                                let np = n.coordinate.extract();
                                let cln = open.clone();
                                let op_find = cln.iter().find(|nx| {
                                    let ndp = nx.coordinate.extract();
                                    ndp.x == np.x && ndp.y == np.y && nx.score() < n.score()
                                });
                                let cls_find = closed.iter().find(|nx| {
                                    let ndp = nx.coordinate.extract();
                                    ndp.x == np.x && ndp.y == np.y && nx.score() < n.score()
                                });
                                if op_find.is_none() && cls_find.is_none() {
                                    open.push(Rc::new(n));
                                }
                            },
                            SearchResult::Found(n) => {
                                return Some(unwrap_nodes(&Rc::new(n)));
                            },
                            SearchResult::None => {}
                        }
                    }
                }
            }

            closed.push(Rc::clone(val));
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        let input = vec![Space::Start(Point{x:0, y:0}), Space::Open(Point{x:0,y:1}), Space::Goal(Point{x:0, y:2})];
        let ip2:Map = vec![input];

        let result = solve_map(&ip2, Point{x:0, y:0}, Point{x:0, y:2}).unwrap();

        for p in result {
            println!("{},{}",p.x, p.y );
        }
    }

    #[test]
    fn blocked_case() {
        let input = vec![Space::Start(Point{x:0, y:0}), Space::Open(Point{x:0,y:1}), Space::Blocked(Point{x:0,y:2}), Space::Goal(Point{x:0, y:3})];
        let input2 = vec![Space::Open(Point{x:1, y:0}), Space::Open(Point{x:1,y:1}), Space::Open(Point{x:1, y:2}), Space::Open(Point{x:1, y:3})];
        let ip2:Map = vec![input, input2];

        let result = solve_map(&ip2, Point{x:0, y:0}, Point{x:0, y:3}).unwrap();

        for p in result {
            println!("{},{}",p.x, p.y );
        }
    }

    #[test]
    fn full_block_case() {
        let input = vec![Space::Start(Point{x:0, y:0}), Space::Open(Point{x:0,y:1}), Space::Blocked(Point{x:0,y:2}), Space::Goal(Point{x:0, y:3})];
        let input2 = vec![Space::Open(Point{x:1, y:0}), Space::Open(Point{x:1,y:1}), Space::Blocked(Point{x:1, y:2}), Space::Open(Point{x:1, y:3})];
        let ip2:Map = vec![input, input2];

        let result = solve_map(&ip2, Point{x:0, y:0}, Point{x:0, y:3});

        assert!(result.is_none());
    }
}