enum List {
    Cons(u32,Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self,pre_elem:u32) -> List {
        List::Cons(pre_elem,Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            List::Cons(_,ref trail) => 1 + tail.len(),
            List::Nil => 0
        }
    }
    
    fn stringfy(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            List::Nil => {
                format!("Nil!")
            },
        }
    }
}