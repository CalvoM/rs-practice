#![allow(dead_code)]
// Memory inefficient linked list due to tail node being junk and still allocated
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }

// Even though Empty is not called
// Does not have null pointer optimization like above approach
// pub enum List {
//     Empty,
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }

// no extra junk at the end, due to npo
// elements uniformly allocated
// /////////////////////////////////////////
// struct Node {
//     elem: i32,
//     next: List,
// }
//
// pub enum List {
//     Empty,
//     More(Box<Node>),
// }
// /////////////////////////////////////////

// List -> Link -> Node
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty), // ugly solution of replacement
        });
        self.head = Link::More(new_node)
    }
    pub fn pop(&mut self) -> Option<i32> {
        match &self.head {
            Link::Empty => {}
            Link::More(node) => {}
        };
        unimplemented!()
    }
}
