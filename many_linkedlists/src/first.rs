#![allow(dead_code)]
// Memory inefficient linked list due to tail node being junk and still allocated
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }
// [] = Stack
// () = Heap
//
// [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)

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
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn test_push_pop() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
