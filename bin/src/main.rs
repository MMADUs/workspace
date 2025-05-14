use core::add;

#[derive(Debug)]
struct Node {
    key: usize,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(key: usize) -> Self {
        Self {
            key,
            next: None,
        }
    }

    fn push(&mut self, key: usize) {
        match self.next {
            Some(ref mut next_n) => next_n.push(key),
            None => self.next = Some(Box::new(Node::new(key))),
        }
    }
}

fn main() {
    let mut node = Node::new(1);
    node.push(2);
    println!("{}", node.next.unwrap().key);
    println!("Cargo Workspace!");
    println!("2 + 2 is: {}", add(2, 2));
}
