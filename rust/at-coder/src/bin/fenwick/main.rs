use core::fmt;
use std::collections::{BTreeMap,BTreeSet};
use text_io::read;
use std::ops::{Add,Sub,Mul,Div};

// const MOD: usize = 53;
const MOD: usize = 998244353;

#[derive(Debug)]
struct BinaryIndexedTree<T>{
    ini_val: T,
    n: usize,
    bit: Vec<T>,
}
impl <T:Clone+Copy+Add<Output=T>> BinaryIndexedTree<T>{
    pub fn new(n: usize, ini_val: T) -> BinaryIndexedTree<T> {
        return Self {
            ini_val,
            n,
            bit: vec![ini_val; n+1],
        };
    }
    pub fn addition(x: T, y: T) -> T{
        return x+y;
    }
    pub fn add(self: &mut Self, mut idx: usize,a: T){
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
    pub fn sum(self: &mut Self, mut idx: usize) -> T {
        idx+=1;
        let mut ret = self.ini_val;
        loop {
            if idx<=0 {
                break;
            }
            ret = Self::addition(ret,self.bit[idx]);
            let idx64 = idx as i64;
            idx-=(idx64 & -idx64) as usize;
        }
        return ret;
    }
}

type Modulus = usize;
struct ModUsizeFactory (Modulus);
impl ModUsizeFactory {
    fn new(self: &Self) -> ModUsize{
        return self.new_val(0);
    }
    fn new_val(self: &Self, val: usize) -> ModUsize{
        return ModUsize {
            modulus: self.0,
            val,
        };
    }
}
#[derive(Debug,Clone,Copy)]
struct ModUsize{
    modulus: usize,
    val: usize,
}
impl ModUsize {
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
    fn div(self: Self, rhs: Self) -> Self {
        return self * rhs.inv();
    }
}
impl fmt::Display for ModUsize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}",self.val);
    }
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
    let f = ModUsizeFactory(MOD);
    let mut bit = BinaryIndexedTree::new(m, f.new());
    let mut ans = f.new();
    let m2 = f.new_val(2);
    let m2_inv = m2.inv();
    for i in 0..n {
        let sum = bit.sum(arr[i]) * m2.pow(i);
        ans = ans + sum;
        bit.add(arr[i], m2_inv.pow(i+1));
    }
    println!("{}", ans);
}
