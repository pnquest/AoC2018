use std::collections::VecDeque;

fn main() {
    let players = 463;
    let last_score = 71787;

    let part_1 = run_game(players, last_score);
    println!("Part 1: {}", part_1);
    let part_2 = run_game(players, last_score * 100);
    println!("Part 2: {}", part_2);
}

fn run_game(players: usize, last_score: usize) -> usize {
    let mut circle = VecDeque::new();
    circle.push_back(0);

    let mut scores = vec![0;players];

    for m in 1..(last_score + 1) {
        if m % 23 == 0 {
            for _ in 0..7 {
                let mv = circle.pop_front().unwrap();
                circle.push_back(mv);
            }

            scores[m % players] += m + circle.pop_back().unwrap();
        } else {
            for _ in 0..2 {
                let mv = circle.pop_back().unwrap();
                circle.push_front(mv);
            }

            circle.push_back(m);
        }
    }

    *scores.iter().max().unwrap()
}