pub enum Node {
    Next(u32, Box<Node>),
    Nil,
}

pub struct LinkedList {
    pub head: Node,
}

impl LinkedList {
    // pub fn add(&mut self, node: Node) {
    //     match self.head {
    //         Node::Nil => {},
    //         Node::Next(value,next )=>{},
    //     }
    // }
}
