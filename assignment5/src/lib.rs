#![allow(unused)]
use rand::prelude::*;
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct List {
    head: Link,
    tail: Link,
}

type Link = Option<Rc<RefCell<Box<dyn Node>>>>;

trait Node {
    fn data(&mut self) -> &mut u32;
    fn next(&mut self) -> &mut Link;
    fn prev(&mut self) -> &mut Link;
}

struct SmallNode {
    data: u32,
    next: Link,
    prev: Link,
}

struct BigNode {
    data: u32,
    next: Link,
    prev: Link,
}

impl Node for SmallNode {
    fn next(&mut self) -> &mut Link {
        &mut self.next
    }
    fn prev(&mut self) -> &mut Link {
        &mut self.prev
    }
    fn data(&mut self) -> &mut u32 {
        &mut self.data
    }
}

impl Node for BigNode {
    fn next(&mut self) -> &mut Link {
        &mut self.next
    }
    fn prev(&mut self) -> &mut Link {
        &mut self.prev
    }
    fn data(&mut self) -> &mut u32 {
        &mut self.data
    }
}

impl List {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    
    fn rand_typ_node(data: u32) -> Rc<RefCell<Box<dyn Node>>> {
        todo!();
    }


    pub fn push_front(&mut self, data: u32) {
        todo!();
    }

    pub fn push_back(&mut self, data: u32) {
        todo!();
    }

    pub fn pop_back(&mut self) -> Option<u32> {
        todo!();
    }

    pub fn pop_front(&mut self) -> Option<u32> {
        todo!();
    }

    pub fn into_iter_list(self) -> IntoIterList {
        IntoIterList(self)
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IntoIterList(List);

impl Iterator for IntoIterList {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.0.pop_front()
    }
}

impl DoubleEndedIterator for IntoIterList {
    fn next_back(&mut self) -> Option<u32> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_push_pop_front() {
        let mut list = List::new();
        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_push_pop_back() {
        let mut list = List::new();

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
}
