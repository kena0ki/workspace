use std::collections::{BTreeMap,BTreeSet};
use text_io::read;

const MOD: usize = 3;
#[derive(Debug)]
struct BinaryIndexedTree{
    n: usize,
    bit: Vec<usize>,
}
impl BinaryIndexedTree{
    pub fn new(n :usize) -> BinaryIndexedTree {
        return Self {
            n,
            bit: vec![0; n+1],
        };
    }
    pub fn addition(x: usize, y: usize) -> usize{
        return (x+y) % MOD;
    }
    pub fn add(self: &mut Self, mut idx: usize,a: usize){
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
    pub fn sum(self: &mut Self, mut idx: usize) -> usize{
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
pub fn modpow(mut x: usize,mut y: usize) -> usize{
    let mut ret = 1;
    while 0 < y {
        if y & 1 == 1{
            ret *= x;
            ret %= MOD;
        }
        x *= x;
        x %= MOD;
        y >>= 1;
    }
    return ret;
}
pub fn comp<T:Ord+Clone+Copy>(a: &mut Vec<T>) -> (Vec<usize>, usize) {
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
    let div:usize = modpow(2,MOD-2);
    let large_n:usize = read!();
    let mut large_a = vec![0usize;large_n];
    for i in 0..large_n {
        large_a[i]=read!();
    }
    let (arr,n) = comp(&mut large_a);
    let mut bit = BinaryIndexedTree::new(n);
    let mut ans = 0;
    for i in 0..large_n {
        ans += bit.sum(arr[i]) *modpow(2,i);
        println!("ans {:?}", bit.sum(arr[i]) *modpow(2,i));
        ans %= MOD;
        bit.add(arr[i],modpow(div,i+1));
        println!("{:?}", bit);
    }
    println!("{}", ans);
}
