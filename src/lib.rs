#![allow(dead_code)]
#![feature(box_syntax, box_patterns)]
use std::cmp::{Ord, PartialEq};

#[derive(Debug)]
pub struct Tree<T: Ord + PartialEq> {
    root: Option<Box<Leaf<T>>>,
}

impl<T: Ord + PartialEq> Tree<T> {
    fn new() -> Tree<T> {
        Tree {
            root: None,
        }
    }

    fn insert(&mut self, val: T) -> bool {
        if let Some(box node) = &mut self.root {
            node.insert(val)
        } else {
            self.root = Leaf::new(val);
            true
        }
    }

    fn contains(&self, val: T) -> bool {
        if let Some(box node) = &self.root {
            node.contains(val)
        } else {
            false
        }
    }

    fn max_depth(&self) -> usize {
        if let Some(box node) = &self.root {
            node.max_depth()
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub struct Leaf<T: Ord + PartialEq> {
    val: T,
    left: Option<Box<Leaf<T>>>,
    right: Option<Box<Leaf<T>>>,
}

impl<T: Ord + PartialEq> Leaf<T> {
    pub fn new(val: T) -> Option<Box<Leaf<T>>> {
        Some(Box::new(Leaf {
            val,
            left: None,
            right: None,
        }))
    }

    pub fn insert(&mut self, val: T) -> bool {
        macro_rules! ins {
            ($side:expr) => {
                match &mut $side {
                    Some(box node) => node.insert(val),
                    None => {
                        $side = Leaf::new(val);
                        true
                    }
                }
            };
        }

        //This seems more concise then splitting these into their own if branch statements no?
        match self.val {
            _ if self.val > val => ins!(self.left),
            _ if self.val < val => ins!(self.right),
            _ => false,
        }
    }

    pub fn contains(&self, val: T) -> bool {
        macro_rules! cts {
            ($side:expr) => {
                match &$side {
                    Some(box side) => side.contains(val),
                    None => false,
                }
            };
        }

        //This seems more concise then splitting these into their own if branch statements no?
        match self.val {
            _ if self.val > val => cts!(self.left),
            _ if self.val < val => cts!(self.right),
            _ => true,
        }
    }

    pub fn max_depth(&self) -> usize {
        macro_rules! dpth {
            ($side:expr) => {
                match &$side {
                    Some(box side) => 1 + side.max_depth(),
                    None => 1,
                }
            };
        }

        let left = dpth!(self.left);
        let right = dpth!(self.right);
        if left < right {
            right
        } else {
            left
        }
    }
}

use std::iter::FromIterator;

impl<T: Ord> FromIterator<T> for Tree<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut c = Tree::new();
        for i in iter {
            c.insert(i);
        }

        c
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert() {
        let mut root = Tree::new();
        assert!(root.insert(3));
        assert!(root.insert(2));
        assert!(root.insert(4));
        assert!(root.insert(1));
        assert!(root.insert(5));
        assert!(!root.insert(3));
    }

    #[test]
    fn contains() {
        let mut root = Tree::new();
        assert!(root.insert(3));
        assert!(root.insert(2));
        assert!(root.insert(4));
        assert!(root.insert(1));
        assert!(root.insert(5));
        assert!(!root.insert(3));

        assert!(root.contains(4));
        assert!(root.contains(5));
        assert!(!root.contains(6));
    }

    #[test]
    fn large_tree() {
        use rand::{self, thread_rng, seq::SliceRandom};
        let mut rng = thread_rng();
        let mut nums = (0..1_000_000).collect::<Vec<usize>>();
        nums.shuffle(&mut rng);
        let thing = nums.into_iter().collect::<Tree<usize>>();

        for i in 0..1_000_000 {
            assert!(thing.contains(i));
        }
    }

    #[test]
    fn depth() {
        let mut tree = Tree::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
        assert_eq!(4, tree.max_depth());
    }
}
