trait Trait {
    fn f(&self);
}

impl<'a> dyn Trait + 'a {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for bool {
    fn f(&self) {
        print!("2");
    }
}

#[cfg(test)]
mod tests{

    use super::*;
    #[test]
    fn test_q10() {
        Trait::f(&true);
        Trait::f(&true as &dyn Trait);
        <_ as Trait>::f(&true);
        <_ as Trait>::f(&true as &dyn Trait);
        <bool as Trait>::f(&true);
    }
}