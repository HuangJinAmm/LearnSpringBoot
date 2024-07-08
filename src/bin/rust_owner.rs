#[derive(Debug)]
struct A {
    a: String,
    b: String,
    c: String,
}

fn main() {
    let x = &A {
        a: "a".into(),
        b: "2".into(),
        c: "3".into(),
    };
    let a = x.a.clone();
    let b = x.b.clone();
    let c = x.c.clone();
    println!("{},{},{},{:?}", a, b, c, x);
}
