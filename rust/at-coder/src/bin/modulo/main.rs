
use text_io::read;
use modulo::ModUsizeFactory;

fn main() {
    let m:usize= read!();
    let n:usize= read!();
    let fa = ModUsizeFactory::new(m);
    let mu = fa.create(n);
    println!("add_by: {:?}", mu.add_by(n));
    println!("sub_by: {:?}", mu.sub_by(n));
    println!("mul_by: {:?}", mu.mul_by(n));
    println!("div_by: {:?}", mu.div_by(n));
}

pub mod modulo {
    use core::fmt;
    use std::ops::{Add,Sub,Mul,Div};

    type NumberType = usize;
    pub struct ModUsizeFactory (NumberType);
    impl ModUsizeFactory {
        pub fn new(modulus:usize) -> Self{
            return Self(modulus);
        }
        pub fn create(self: &Self, val: NumberType) -> ModUsize{
            return ModUsize {
                modulus: self.0,
                val: val%self.0,
            };
        }
    }

    #[derive(Debug,Clone,Copy)]
    pub struct ModUsize{
        modulus: NumberType,
        pub val: NumberType,
    }
    impl ModUsize {
        fn sibling(self: &Self, val:usize) -> Self {
            return Self {
                modulus: self.modulus,
                val: val%self.modulus,
            };
        }
        pub fn set_val(self: &mut Self, val: usize) {
            self.val = val %self.modulus;
        }
        pub fn add_by(self: Self, rhs: NumberType) -> Self{
            return self + self.sibling(rhs);
        }
        pub fn sub_by(self: Self, rhs: NumberType) -> Self{
            return self - self.sibling(rhs);
        }
        pub fn mul_by(self: Self, rhs: NumberType) -> Self{
            return self * self.sibling(rhs);
        }
        pub fn div_by(self: Self, rhs: NumberType) -> Self{
            return self / self.sibling(rhs);
        }
        pub fn pow(self: Self, mut power: NumberType) -> Self{
            let mut square = self.val;
            let mut ret = 1;
            while 0 < power {
                if (power & 1) == 1{
                    ret *= square;
                    ret %= self.modulus;
                }
                square *= square;
                square %= self.modulus;
                power >>= 1;
            }
            return Self {
                val:ret,
                modulus: self.modulus,
            };
        }
        pub fn inv(self: Self) -> Self {
            return self.pow(self.modulus - 2);
        }
    }
    impl Add for ModUsize {
        type Output = Self;
        fn add(mut self: Self, rhs: Self) -> Self {
            self.val += rhs.val;
            if self.val >= self.modulus {
                self.val -= self.modulus;
            }
            return self;
        }
    }
    impl Sub for ModUsize {
        type Output = Self;
        fn sub(mut self: Self, rhs: Self) -> Self {
            if self.val < rhs.val {
                self.val += self.modulus - rhs.val;
            } else {
                self.val -= rhs.val;
            }
            return self;
        }
    }
    impl Mul for ModUsize {
        type Output = Self;
        fn mul(mut self: Self, rhs: Self) -> Self {
            self.val = (self.val * rhs.val) % self.modulus;
            return self;
        }
    }
    impl Div for ModUsize {
        type Output = Self;
        fn div(mut self: Self, rhs: Self) -> Self {
            let mut power = self.modulus - 2;
            let mut square = rhs.val;
            while 0 < power {
                if (power & 1) == 1{
                    self.val *= square;
                    self.val %= self.modulus;
                }
                square *= square;
                square %= self.modulus;
                power >>= 1;
            }
            return self;
        }
    }
    impl fmt::Display for ModUsize {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            return write!(f, "{}",self.val);
        }
    }
}

