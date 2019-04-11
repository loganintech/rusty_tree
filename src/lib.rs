#![allow(dead_code)]
#![feature(box_syntax, box_patterns)]
use std::cmp::{Ord, PartialEq};

#[derive(Debug)]
pub struct Tree<T: Ord + PartialEq> {
    root: Option<Box<Leaf<T>>>,
}

impl<T: Ord + PartialEq> Tree<T> {
    fn new(val: T) -> Tree<T> {
        Tree {
            root: Leaf::new(val),
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

    fn contains(&mut self, val: T) -> bool {
        if let Some(box node) = &mut self.root {
            node.contains(val)
        } else {
            false
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert() {
        let mut root = Tree::new(3);
        assert!(root.insert(2));
        assert!(root.insert(4));
        assert!(root.insert(1));
        assert!(root.insert(5));
        assert!(!root.insert(3));

        assert!(root.contains(4));
        assert!(root.contains(5));
        assert!(!root.contains(6));
        dbg!(root);
    }
}
