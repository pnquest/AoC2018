pub struct SummedAreaTable {
    pub table: Vec<Vec<usize>>
}

impl SummedAreaTable {
    pub fn from_vec(v: Vec<Vec<usize>>) -> SummedAreaTable {
        let outer_size = v.len();
        let inner_size = v[0].len();
        let mut t = vec![vec![0; inner_size]; outer_size];

        for x in 0..outer_size {
            for y in 0..inner_size {
                let mut val = v[x][y];

                if x > 0 {
                    val += t[x - 1][y];
                }

                if y > 0 {
                    val += t[x][y - 1];
                }

                if x > 0 && y > 0 {
                    val -= t[x - 1][y - 1];
                }

                t[x][y] = val;
            }
        }

        SummedAreaTable{table: t}
    }

    pub fn compute_sum(&self, x: isize, y: isize, w: isize, h: isize) -> usize {
        let (dx, dy) = (x + w - 1, y + h - 1);
        let (ax, ay) = (dx - w, dy - h);
        let (bx, by) = (dx, dy - h);
        let (cx, cy) = (dx - w, dy);

        let d = self.table[dx as usize][dy as usize];
        let a = if ax >= 0 && ay >= 0 {
            self.table[ax as usize][ay as usize]
        } else {
            0
        };
        let b = if bx >= 0 && by >= 0 {
            self.table[bx as usize][by as usize]
        } else {
            0
        };
        let c = if cx >= 0 && cy >= 0 {
            self.table[cx as usize][cy as usize]
        } else {
            0
        };

        d + a - b - c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_work() {
        let v = vec![
            vec![31, 12, 13, 24, 30, 1],
            vec![2, 26, 17, 23, 8, 35],
            vec![4, 9, 21, 15, 28, 34],
            vec![33, 10, 22, 16, 27, 3],
            vec![5, 29, 20, 14, 11, 32],
            vec![36, 25, 18, 19, 7, 6]
        ];

        let sat = SummedAreaTable::from_vec(v);

        let sum = sat.compute_sum(2, 3, 3, 2);

        assert_eq!(sum, 111);
    }

}