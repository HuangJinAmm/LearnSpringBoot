#![feature(const_generics)]

use core::marker::PhantomData;
use core::ops::Add;

trait LengthType<const F: i64> {}

#[derive(Debug, Clone, Copy)]
struct Length<const F: f64>(f64, PhantomData<dyn LengthType<F>>);

type Meter = Length<1.0>;
type Mm = Length<1000.0>;
type Inch = Length<{ 1000.0 / 25.4 }>;

impl<const F1: f64> Length<F1> {
    fn new(val: f64) -> Self {
        Length(val, PhantomData)
    }

    fn value(self) -> f64 {
        self.0
    }

    fn factor() -> f64 {
        F1
    }

    fn _from<const F2: f64>(src: Length<F2>) -> Self {
        Self::new(src.0 * F1 / F2)
    }

    fn _into<const F2: f64>(self) -> Length<F2> {
        Length::new(self.0 * F2 / F1)
    }
}

impl<const F: f64> Add for Length<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.0 + other.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_gc() {
        let m1 = Meter::new(1.0);
        let inch3 = Inch::new(3.0);
        let mm40 = Mm::new(40.0);

        let a = m1 + inch3._into();
        println!("m1 + inch3 = {} m", a.value());

        let b = mm40 + inch3._into();
        println!("mm40 + inch3 = {} mm", b.value());

        let c = mm40 + m1._into();
        println!("mm40 + m1 = {} mm", c.value());
    }
}
