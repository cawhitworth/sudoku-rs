use std::collections::HashSet;
use std::vec::Vec;
use std::usize;

pub struct MultiSet<T> {
    set: Vec<HashSet<T>>,
}

impl<T: Eq + std::hash::Hash> MultiSet<T> {
    pub fn new(count: usize) -> MultiSet<T> {
        let mut set = Vec::new();
        for _ in 0..count {
            let h = HashSet::new();
            set.push(h);
        }

        MultiSet {
            set: set,
        }
    }

    pub fn insert(&mut self, index: usize, t: T ) -> bool {
        self.set[index].insert(t)
    }

    pub fn set(&mut self, index: usize) -> &mut HashSet<T> {
        &mut self.set[index]
    }
}
