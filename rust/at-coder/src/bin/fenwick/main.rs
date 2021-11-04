use core::fmt;
use std::collections::{BTreeMap,BTreeSet};
use text_io::read;
use std::ops::{Add,Sub,Mul,Div};

const MOD: usize = 3;
type BitType=i64;
#[derive(Debug)]
struct BinaryIndexedTree{
    n: usize,
    bit: Vec<BitType>,
}
impl BinaryIndexedTree{
    pub fn new(n :usize) -> BinaryIndexedTree {
        return Self {
            n,
            bit: vec![0; n+1],
        };
    }
    pub fn addition(x: BitType, y: BitType) -> BitType{
        return (x+y) % MOD as BitType;
    }
    pub fn add(self: &mut Self, mut idx: usize,a: BitType){
        idx+=1;
        loop {
            if idx > self.n {
                break;
            }
            self.bit[idx] = Self::addition(self.bit[idx],a);
            let idx64 = idx as i64;
            idx+=(idx64 & -idx64) as usize;
        }
    }
    pub fn sum(self: &mut Self, mut idx: usize) -> BitType{
        idx+=1;
        let mut ret = 0;
        loop {
            if idx<=0 {
                break;
            }
            println!("idx: {}", idx);
            ret += Self::addition(ret,self.bit[idx]);
            let idx64 = idx as i64;
            idx-=(idx64 & -idx64) as usize;
        }
        return ret;
    }
}
type Modulus = usize;
struct MUIntFactory (Modulus);

impl MUIntFactory {
    fn new(self: &Self) -> MUInt{
        return self.new_val(0);
    }
    fn new_val(self: &Self, val: usize) -> MUInt{
        return MUInt {
            modulus: self.0,
            val,
        };
    }
}

#[derive(Debug,Clone,Copy)]
struct MUInt{
    modulus: usize,
    val: usize,
}
impl MUInt {
    pub fn pow(self: &Self, mut power: usize) -> Self{
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
    pub fn inv(self: &Self) -> Self {
        return self.pow(self.modulus - 2);
    }
}
impl Add for MUInt {
    type Output = Self;
    fn add(mut self: Self, rhs: Self) -> Self {
        self.val += rhs.val;
        if self.val >= self.modulus {
            self.val -= self.modulus;
        }
        return self;
    }
}
impl Sub for MUInt {
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
impl Mul for MUInt {
    type Output = Self;
    fn mul(mut self: Self, rhs: Self) -> Self {
        self.val = (self.val * rhs.val) % self.modulus;
        return self;
    }
}
impl Div for MUInt {
    type Output = Self;
    fn div(self: Self, rhs: Self) -> Self {
        return self * rhs.inv();
    }
}
impl fmt::Display for MUInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}",self.val);
    }
}


pub fn modpow(mut x: usize,mut y: usize) -> usize{
    let mut ret = 1;
    while 0 < y {
        if y & 1 == 1{
            ret *= x;
            ret %= MOD as usize;
        }
        x *= x;
        x %= MOD as usize;
        y >>= 1;
    }
    return ret;
}
pub fn modinv(x: usize) -> usize {
    return modpow(x, MOD as usize - 2);
}
pub fn compress<T:Ord+Clone+Copy>(a: &mut Vec<T>) -> (Vec<usize>, usize) {
    let mut set = BTreeSet::<T>::new();
    for i in 0..a.len() {
        set.insert(a[i]);
    }
    let mut size = 0;
    let mut mem = BTreeMap::<T,usize>::new();
    for key in set {
        mem.insert(key, size);
        size+=1;
    }
    let mut ret = vec![0; a.len()];
    for i in 0..a.len() {
        ret[i] = *mem.get(&a[i]).unwrap();
    }
    return (ret, size);
}
fn main(){
    let n:usize = read!();
    let mut a = vec![0usize;n];
    for i in 0..n {
        a[i]=read!();
    }
    let (arr,m) = compress(&mut a);
    let mut bit = BinaryIndexedTree::new(m);
    let factory = MUIntFactory(MOD);
    let mut ans = factory.new();
    let m2 = factory.new_val(2);
    let m2_inv = m2.inv();
    for i in 0..n {
        let sum = factory.new_val(bit.sum(arr[i]) as usize) * m2.pow(i);
        ans = ans + sum;
        bit.add(arr[i], m2_inv.pow(i+1).val as i64);
        println!("{:?}", bit);
    }
    println!("{}", ans);
}
