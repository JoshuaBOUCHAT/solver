use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    usize,
};
const INIT: u16 = 0x1FF;

#[derive(Clone)]
pub struct Board {
    inner: [u16; 81],
}
fn apply_filter(n: &mut u16, mask: u16) -> bool {
    if n.count_ones() != 1 {
        let temp = *n ^ mask;
        *n = temp;
        return temp.count_ones() == 1;
    }
    false
}

impl Board {
    fn new() -> Self {
        Self { inner: [INIT; 81] }
    }
    pub fn solve(&mut self) {
        let mut to_modifys = vec![];
        for i in 0..9 {
            for j in 0..9 {
                let temp = self[i][j];
                if temp.count_ones() == 1 {
                    to_modifys.push((i, j));
                    println!("Pushing: {}:{}", i, j);
                }
            }
        }
        println!(
            "The number of filter to test: {}\n here are the inital grid:\n{:?}",
            to_modifys.len(),
            &self
        );
        while let Some((i, j)) = to_modifys.pop() {
            println!("Poping: {}:{}", i, j);
            self.filter_column(j, i, &mut to_modifys);
            println!("Column:\n{:?}", &self);
            self.filter_row(i, j, &mut to_modifys);
            println!("Rows:\n{:?}", &self);
            self.filter_block(i, j, &mut to_modifys);
            println!("Block:\n{:?}", &self);
        }
    }
    fn filter_column(
        &mut self,
        column_pos: usize,
        filter_pos: usize,
        to_modifys: &mut Vec<(usize, usize)>,
    ) {
        let mask = self[filter_pos][column_pos];

        for i in 0..filter_pos {
            if apply_filter(&mut self[i][column_pos], mask) {
                to_modifys.push((i, column_pos))
            }
        }
        for i in (filter_pos + 1)..9 {
            if apply_filter(&mut self[i][column_pos], mask) {
                to_modifys.push((i, column_pos))
            }
        }
    }
    fn filter_row(
        &mut self,
        row_pos: usize,
        filter_pos: usize,
        to_modifys: &mut Vec<(usize, usize)>,
    ) {
        let mask = self[row_pos][filter_pos];

        for i in 0..filter_pos {
            if apply_filter(&mut self[row_pos][i], mask) {
                to_modifys.push((row_pos, i))
            }
        }
        for i in (filter_pos + 1)..9 {
            if apply_filter(&mut self[row_pos][i], mask) {
                to_modifys.push((row_pos, i))
            }
        }
    }
    fn filter_block(
        &mut self,
        filter_posx: usize,
        filter_posy: usize,
        to_modifys: &mut Vec<(usize, usize)>,
    ) {
        let idx = filter_posx / 3;
        let idy = filter_posy / 3;

        let mask: u16 = self[filter_posx][filter_posy];
        for i in 0..3 {
            let row_idx = idx * 3 + i;
            for j in 0..3 {
                let column_idy = idy * 3 + j;
                if row_idx != filter_posx
                    && column_idy != filter_posy
                    && apply_filter(&mut self[row_idx][column_idy], mask)
                {
                    to_modifys.push((row_idx, column_idy))
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
        '?'
    } else {
        ((16 - repr.leading_zeros()) as u8 + '0' as u8) as char
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            let row = &self[i];
            for j in (0..9).step_by(3) {
                write!(f, "{} ", repr_number_to_char(row[j]))?;
                write!(f, "{} ", repr_number_to_char(row[j + 1]))?;
                write!(f, "{}", repr_number_to_char(row[j + 2]))?;
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
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            let row = &self[i];
            for j in (0..9).step_by(3) {
                write!(f, "{:02} ", row[j])?;
                write!(f, "{:02} ", row[j + 1])?;
                write!(f, "{:02}", row[j + 2])?;
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
