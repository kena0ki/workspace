use std::{time::Instant, collections::HashSet, mem::size_of_val};

use rustrithm::bitarray::BitArray;
use bitvec::prelude::*;

fn main() {
    let n = 1000;
    let darr = (1..=n).collect::<Vec<_>>();
    let start = Instant::now();
    let dp = _f_dp_bit(n, &*darr);
    let duration = start.elapsed();
    println!("Time elapsed in _f_dp_bit() is: {:?}", duration);
    println!("The memory size of dp is: {:?}", size_of_val(&dp[0].len()));
    println!("The length of dp is: {:?}", dp[0].len());

    let start = Instant::now();
    _f_dp_bitvec(n, &*darr);
    let duration = start.elapsed();
    println!("Time elapsed in _f_dp_bitvec() is: {:?}", duration);

    let start = Instant::now();
    _f_vec(n, &*darr);
    let duration = start.elapsed();
    println!("Time elapsed in _f_vec() is: {:?}", duration);

    let start = Instant::now();
    _f_dp_set(n, &*darr);
    let duration = start.elapsed();
    println!("Time elapsed in _f_dp_set() is: {:?}", duration);
}

fn _f_vec(n: usize, darr: &[usize]) {
    let mut dp = Vec::with_capacity(n+1);

    for i in 0..n {
        let mut new = 0;
        for j in 0..100_000 {
            new+=darr[i]+j;
            new/=darr[i]+j;
        }
        dp.push(new);
    }
}

fn _f_dp_set(n:usize, darr: &[usize]) -> Vec<HashSet<usize>>{
    let mut dp=Vec::with_capacity(n+1);
    let mut set = HashSet::<usize>::new();
    set.insert(0);
    dp.push(set);
    for i in 0..n {
        //let mut next_set = dp[i].clone();
        let mut next_set = HashSet::<usize>::with_capacity(dp[i].len()*2);
        for &s in &dp[i] {
            next_set.insert(s);
        }
        //next_set.reserve(next_set.len()*2);
        for s in dp[i].iter() {
            next_set.insert(s+darr[i]);
        }
        dp.push(next_set);
    }
    // println!("{:?}", dp);
    return dp;
}

const CENT: usize = bitvec::mem::elts::<usize>(1_000_000);
fn _f_dp_bitvec(n: usize, darr: &[usize]) -> Vec<bitvec::array::BitArray::<Lsb0, [usize; CENT]>>{
    let mut dp = Vec::with_capacity(n+1);
    let mut a: BitArr!(for 1_000_000)
      = bitvec::array::BitArray::<Lsb0, [usize; CENT]>::zeroed();
    *a.get_mut(0).unwrap() = true;
    dp.push(a);
    for i in 0..n {
        dp[0].rotate_left(darr[i]);
    }
    return dp;
}

fn _f_dp_bit(n: usize, darr: &[usize]) -> Vec<BitArray>{
    let mut dp = Vec::with_capacity(n+1);
    //dp.push(BitArray::from_u8slice_with_size(&[1], 3600001));
    dp.push(BitArray::from_u8slice_with_size(&[1], 1_000_000));
    for i in 0..n {
        let next = &(&dp[i] << darr[i]) | &dp[i];
        dp.push(next);
    }
    return dp;
}
