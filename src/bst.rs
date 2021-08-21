//! Binary search tree implementation

use std::cmp::Ordering;

pub struct Node<T> {
    pub key: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

#[derive(Default)]
pub struct BinarySearchTree<T> {
    pub root: Option<Node<T>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn insert(&mut self, key: T) -> bool {
        match self.root {
            Some(ref mut root) => {
                let mut cursor: &mut Node<T> = root;
                loop {
                    match key.cmp(&cursor.key) {
                        Ordering::Equal => return false,
                        Ordering::Less => match cursor.left {
                            Some(ref mut left) => cursor = left,
                            None => {
                                cursor.left = Some(Box::new(Node::new(key)));
                                return true;
                            }
                        },
                        Ordering::Greater => match cursor.right {
                            Some(ref mut right) => cursor = right,
                            None => {
                                cursor.right = Some(Box::new(Node::new(key)));
                                return true;
                            }
                        },
                    }
                }
            }
            None => {
                self.root = Some(Node::new(key));
                true
            }
        }
    }
}

impl<T> Node<T> {
    fn new(key: T) -> Self {
        Self {
            key,
            left: None,
            right: None,
        }
    }
}
