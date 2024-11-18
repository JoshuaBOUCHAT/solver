#![allow(dead_code)]

use crate::node::Node;
use std::cmp::Ordering::*;
use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    usize,
};

pub struct SearchTree<T, const SIZE: usize>
where
    T: Ord,
{
    data: [Node<T>; SIZE],
    index_to_insert: usize,
}
impl<T: Ord, const SIZE: usize> Index<usize> for SearchTree<T, SIZE> {
    type Output = Node<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T: Ord, const SIZE: usize> IndexMut<usize> for SearchTree<T, SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Ord, const SIZE: usize> SearchTree<T, SIZE> {
    pub fn new() -> Self {
        Self {
            data: unsafe {
                MaybeUninit::array_assume_init(core::array::from_fn(|_| MaybeUninit::zeroed()))
            },
            index_to_insert: 0,
        }
    }
    /// Try to insert an element inside the tree if the tree already contain the value
    /// insert do nothing and return false, if the value is does not existe it add it and return false.
    /// This function assume that you will not push element where there is no more space available and
    /// crash if size isn't enough
    pub fn insert(&mut self, value: T) -> bool {
        if self.index_to_insert == 0 {
            self.insert_value(value);
            return true;
        }
        let mut actual_node = 0;
        loop {
            match value.cmp(self[actual_node].value()) {
                Equal => return false,

                Greater => {
                    if let Some(idx) = self[actual_node].right() {
                        actual_node = idx;
                        continue;
                    }
                    let idx = self.insert_value(value);
                    self[actual_node].set_right(idx);
                    return true;
                }
                Less => {
                    if let Some(idx) = self[actual_node].left() {
                        actual_node = idx;
                        continue;
                    }
                    let idx = self.insert_value(value);
                    self[actual_node].set_left(idx);
                    return true;
                }
            }
        }
    }
    fn insert_value(&mut self, value: T) -> usize {
        if self.index_to_insert > SIZE {
            panic!("Try to insert when no more space is available")
        }
        let idx = self.index_to_insert;
        self[idx] = Node::new(value);
        self.index_to_insert += 1;
        idx
    }
}
