use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    usize,
};
const INIT: u16 = 0x1FF;

#[derive(Clone)]
pub struct Board {
    inner: [u16; 81],
    to_modifys: Vec<(usize, usize)>,
}
fn apply_filter(n: &mut u16, mask: u16) -> bool {
    if n.count_ones() != 1 {
        let temp = (*n) & mask;
        *n = temp;
        return temp.count_ones() == 1;
    }
    false
}

impl Board {
    fn new() -> Self {
        Self {
            inner: [INIT; 81],
            to_modifys: vec![],
        }
    }
    pub fn solve(&self) -> Self {
        let mut init = self.clone();
        for i in 0..9 {
            for j in 0..9 {
                let temp = init[i][j];
                if temp.count_ones() == 1 {
                    init.to_modifys.push((i, j));
                }
            }
        }
        let mut stack = vec![init];
        let mut res = Board::new();
        while let Some(mut actual) = stack.pop() {
            actual.filter_board();
            res = actual;
            if res.is_solve() {
                break;
            }
            let (x, y) = self.get_lowest_undefine();
            let mut val = self[x][y];
            println!("val[{x}][{y}]: {val}");
            let mut c = 0;
            while val > 0 {
                let temp = val & 1;
                if temp != 0 {
                    let mut attempt = self.clone();
                    attempt[x][y] = 1 << c;
                    attempt.to_modifys.push((x, y));
                    println!("{x},{y}: {}", 1 << c);
                    //println!("{}", &attempt);
                    //stack.push(attempt);
                }

                c += 1;
                val >>= 1;
            }
        }
        return res;
    }
    fn get_lowest_undefine(&self) -> (usize, usize) {
        let mut i = 0;
        let mut min = 9;
        let mut min_val = 0x1FF;
        while min.count_ones() != 2 && i < 81 {
            let count = min.count_zeros();
            if count == 1 {
                i += 1;
                continue;
            }

            i += 1;
        }
        return min;
    }
    fn filter_board(&mut self) {
        loop {
            while let Some((i, j)) = self.to_modifys.pop() {
                self.filter_column(j, i);
                self.filter_row(i, j);
                self.filter_block(i, j);
            }
            self.filter_block_possibility();
            self.filter_column_possibility();
            self.filter_row_possibility();
            if self.to_modifys.len() == 0 {
                break;
            }
        }
    }

    fn filter_column(&mut self, column_pos: usize, filter_pos: usize) {
        let mask = !self[filter_pos][column_pos];

        for i in 0..filter_pos {
            if apply_filter(&mut self[i][column_pos], mask) {
                self.to_modifys.push((i, column_pos))
            }
        }
        for i in (filter_pos + 1)..9 {
            if apply_filter(&mut self[i][column_pos], mask) {
                self.to_modifys.push((i, column_pos))
            }
        }
    }
    fn filter_row(&mut self, row_pos: usize, filter_pos: usize) {
        let mask = !self[row_pos][filter_pos];

        for i in 0..filter_pos {
            if apply_filter(&mut self[row_pos][i], mask) {
                self.to_modifys.push((row_pos, i))
            }
        }
        for i in (filter_pos + 1)..9 {
            if apply_filter(&mut self[row_pos][i], mask) {
                self.to_modifys.push((row_pos, i))
            }
        }
    }
    fn filter_block(&mut self, filter_posx: usize, filter_posy: usize) {
        let idx = filter_posx / 3;
        let idy = filter_posy / 3;

        let mask: u16 = !self[filter_posx][filter_posy];
        for i in 0..3 {
            let row_idx = idx * 3 + i;
            for j in 0..3 {
                let column_idy = idy * 3 + j;
                if row_idx != filter_posx
                    && column_idy != filter_posy
                    && apply_filter(&mut self[row_idx][column_idy], mask)
                {
                    self.to_modifys.push((row_idx, column_idy))
                }
            }
        }
    }
    fn filter_block_possibility(&mut self) {
        for blk_x in 0..3 {
            for blk_y in 0..3 {
                let mut global_mask = 0;
                let mut global_duplicate = 0;
                for i in 0..3 {
                    let row = blk_x * 3 + i;
                    for j in 0..3 {
                        let n = self[row][blk_y * 3 + j];
                        let intersection = global_mask & n;
                        global_duplicate |= intersection;
                        global_mask |= n;
                    }
                }
                let unique_mask = global_mask ^ global_duplicate;
                for i in 0..3 {
                    let row = blk_x * 3 + i;
                    for j in 0..3 {
                        let col = blk_y * 3 + j;
                        let temp = self[row][col] & unique_mask;
                        if temp != 0 && self[row][col] != temp {
                            self[row][col] = temp;
                            self.to_modifys.push((row, col));
                        }
                    }
                }
            }
        }
    }
    fn filter_row_possibility(&mut self) {
        for i in 0..9 {
            let mut global_mask = 0;
            let mut global_duplicate = 0;
            for j in 0..9 {
                let n = self[i][j];
                let intersection = global_mask & n;
                global_duplicate |= intersection;
                global_mask |= n;
            }
            let unique_mask = global_mask ^ global_duplicate;
            for j in 0..9 {
                let temp = self[i][j] & unique_mask;
                if temp != 0 && self[i][j] != temp {
                    self[i][j] = temp;
                    self.to_modifys.push((i, j));
                }
            }
        }
    }
    fn filter_column_possibility(&mut self) {
        for j in 0..9 {
            let mut global_mask = 0;
            let mut global_duplicate = 0;
            for i in 0..9 {
                let n = self[i][j];
                let intersection = global_mask & n;
                global_duplicate |= intersection;
                global_mask |= n;
            }
            let unique_mask = global_mask ^ global_duplicate;
            for i in 0..9 {
                let temp = self[i][j] & unique_mask;
                if temp != 0 && self[i][j] != temp {
                    self[i][j] = temp;
                    self.to_modifys.push((i, j));
                }
            }
        }
    }

    pub fn transform_back(self, vec: &mut Vec<Vec<char>>) {
        for i in 0..9 {
            for j in 0..9 {
                vec[i][j] = (((16 - (self[i][j].leading_zeros())) as u8) + '0' as u8) as char
            }
        }
    }
    fn is_solve(&self) -> bool {
        self.inner.iter().all(|i| i.count_ones() == 1)
    }
}
impl From<&Vec<Vec<char>>> for Board {
    fn from(value: &Vec<Vec<char>>) -> Self {
        let mut ret = Self::new();
        for i in 0..9 {
            for j in 0..9 {
                let c = value[i][j];
                match c {
                    '1'..='9' => {
                        ret[i][j] = 1u16 << (c as u8 - '1' as u8);
                    }
                    '.' => {}
                    _ => panic!("wrong input"),
                }
            }
        }
        ret
    }
}

impl Index<usize> for Board {
    type Output = [u16];
    fn index(&self, index: usize) -> &Self::Output {
        let idx = index * 9;
        &self.inner[idx..(idx + 9)]
    }
}
impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let idx = index * 9;
        &mut self.inner[idx..(idx + 9)]
    }
}

fn repr_number_to_char(repr: u16) -> char {
    if repr.count_ones() != 1 {
        ' '
    } else {
        ((16 - repr.leading_zeros()) as u8 + '0' as u8) as char
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            let row = &self[i];
            for j in (0..9).step_by(3) {
                write!(f, " {} ", repr_number_to_char(row[j]))?;
                write!(f, " {} ", repr_number_to_char(row[j + 1]))?;
                write!(f, " {} ", repr_number_to_char(row[j + 2]))?;
                if j == 0 || j == 3 {
                    write!(f, "|")?;
                }
            }
            writeln!(f, "")?;
            if i == 2 || i == 5 {
                write!(f, "-----------------------------")?;
            }
            writeln!(f, "")?;
        }
        writeln!(f)
    }
}
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            let row = &self[i];
            for j in (0..9).step_by(3) {
                write!(f, "{:03} ", row[j])?;
                write!(f, "{:03} ", row[j + 1])?;
                write!(f, "{:03}", row[j + 2])?;
                if j != 8 {
                    write!(f, "|")?;
                }
            }
            if i != 8 {
                write!(f, "\n-----------------\n")?;
            }
        }
        writeln!(f)
    }
}
