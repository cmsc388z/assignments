use std::fmt;

pub struct LinkedList<T> {
    head: Option<Node<T>>,
    len: i32
}

pub struct Node<T> {
    pub next: Option<Box<Node<T>>>,
    pub data: T
}

