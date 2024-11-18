#![feature(maybe_uninit_array_assume_init)]
mod bst;
mod node;

#[cfg(test)]
mod tests {
    use crate::bst::SearchTree;

    #[test]
    fn it_works() {
        let mut tree: SearchTree<i32, 10> = SearchTree::new();
        assert_eq!(tree.insert(100), true);
        assert_eq!(tree.insert(10), true);
        assert_eq!(tree.insert(100), false);
    }
}
