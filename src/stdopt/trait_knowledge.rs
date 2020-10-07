#![allow(dead_code)]
trait Animal {
    fn name(&self)->&'static str;

    fn noise(&self)->&'static str;

    fn talk(&self) {
        println!("{} says {}",self.name(),self.noise()); 
    }
}

impl Animal for Cat {

    fn name(&self)->&'static str{
        self.name
    }

    fn noise(&self)->&'static str{
        "miaomiao"
    }
}

impl Animal for StuffedAnimal {

    fn name(&self)->&'static str{
        self.name
    }

    fn noise(&self)->&'static str{
        "zzzzz"
    }
}
#[derive(Debug)]
struct Cat {
    name: &'static str,
    age: i32
}

struct StuffedAnimal {
    name: &'static str,
}

fn make_cat(name:&'static str) -> impl Animal {
        Cat {
            name: name,
            age: 5i32
        }
    }

#[cfg(test)]
mod tests{

    use super::*;
    #[test]
    fn test_trait() {
        let c = make_cat("aaa");
        c.talk();
    }
}