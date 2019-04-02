fn main() {
    let r_count = 554401;

    part_1(r_count);
    part_2(r_count);
}

fn part_1(r_count: usize) {
    let mut recipies = vec![3,7];
    let mut elves: [usize;2] = [0,1];
    
    while recipies.len() < r_count + 10 { 
        push_recipies(&mut recipies, &mut elves);
    }

    print!("part 1: ");
    for i in r_count..r_count + 10 {
        print!("{}", recipies[i]);
    }

    println!("");
}

fn part_2(r_count: usize) {
    let mut recipies = vec![3,7];
    let mut elves: [usize;2] = [0,1];
    let r_values = r_count
        .to_string()
        .bytes()
        .map(|b| b - b'0')
        .map(|v| v as usize)
        .collect::<Vec<usize>>();

    loop {
        push_recipies(&mut recipies, &mut elves);
        for o in 0..=1 {
            if recipies.len() - o >= r_values.len() {
                if recipies[recipies.len() - r_values.len() - o..recipies.len() - o] == r_values[..] {
                    println!("part 2: {}", recipies.len() - r_values.len() - o);
                    return;
                }
            }
        }
    }
}

fn push_recipies(recipies: &mut Vec<usize>, elves: &mut [usize]) {
    let sum: usize = elves.iter().map(|e| recipies[*e]).sum();

        let v1 = sum / 10;
        if v1 > 0 {
            recipies.push(v1);
        }
        recipies.push(sum % 10);

        elves[0] = (elves[0] + recipies[elves[0]] + 1) % recipies.len();
        elves[1] = (elves[1] + recipies[elves[1]] + 1) % recipies.len();
}
