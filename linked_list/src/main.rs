struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            node: None,
        }
    }
}

fn main() {
    let c = Node::new(3, None);
    // let b = Node::new(2, c);
    // let a = Node::new(1, b);
    // let head = Node::new(1, Node::new(2, Node::new(3, None)));
    // println!(head.value, head.next.value, head.next.next.value, head.next.next.next.value);
    println!(c.value);
}
