extern crate common;

use common::summed_area::SummedAreaTable;

fn main() {
    let mut power_grid = vec![vec![0;300];300];

    for x in 0..300 {
        for y in 0..300 {
            power_grid[x][y] = calc_power_level(x, y);
        }
    }

    part_1(&power_grid);
    part_2(&power_grid);
}

fn part_2(grid: &Vec<Vec<isize>>) {
    let sat = SummedAreaTable::from_vec(grid);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_s = 0;
    let mut max_val = -100;

    for s in 1..301 {
        for x in 0..300 - (s - 1) {
            for y in 0..300 - (s - 1) {
                let v = sat.compute_sum(x, y, s, s);

                if v > max_val {
                    max_x = x + 1;
                    max_y = y + 1;
                    max_s = s;
                    max_val = v;
                }
            }
        }
    }

    println!("Part 1: ({},{},{}) = {}", max_x, max_y, max_s, max_val);
}

fn part_1(grid: &Vec<Vec<isize>>) {
    let sat = SummedAreaTable::from_vec(grid);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_val = -100;

    for x in 0..300 - 2 {
        for y in 0..300 - 2 {
            let v = sat.compute_sum(x, y, 3, 3);

            if v > max_val {
                max_x = x + 1;
                max_y = y + 1;
                max_val = v;
            }
        }
    }

    println!("Part 1: ({},{}) = {}", max_x, max_y, max_val);
}

fn calc_power_level(x: usize, y: usize) -> isize {
    let rack_id: isize = x as isize + 1 + 10;
    let serial_number: isize = 8772;
    let mut pow = rack_id * (y as isize + 1);
    pow += serial_number;
    pow *= rack_id;

    if pow < 0 {
        -5
    } else {
        pow / 100 % 10 - 5
    }
}