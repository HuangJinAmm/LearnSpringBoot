use std::rc::Rc;

fn main(){
    let a = Node::new(3, Rc::new(None));
    let b = Node::new(4, Rc::new(Some(&a)));
    let c = Node::new(5, Rc::new(Some(&a)));

    println!("b is {:#?}",b);
    println!("a is {:#?}",c);
}

#[derive(Debug)]
struct  Node {
    value: u32,
    next: Rc<Option<Node>>
}

impl Node {
    fn new(value: u32, next: Rc<Option<Node>>) -> Self { Self { value, next } }
}
