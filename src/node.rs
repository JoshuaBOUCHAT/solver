#[derive(Debug, Clone)]
pub struct Node<T> {
    value: T,
    left: usize,
    right: usize,
}
impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: usize::MAX,
            right: usize::MAX,
        }
    }
    pub fn left(&self) -> Option<usize> {
        if self.left == usize::MAX {
            return None;
        }
        return Some(self.left);
    }
    pub fn right(&self) -> Option<usize> {
        if self.right == usize::MAX {
            return None;
        }
        Some(self.right)
    }
    pub fn set_right(&mut self, right: usize) {
        self.right = right;
    }
    pub fn set_left(&mut self, left: usize) {
        self.left = left;
    }
    pub fn value(&self) -> &T {
        &self.value
    }
}
