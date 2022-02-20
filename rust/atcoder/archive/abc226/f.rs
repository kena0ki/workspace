// template

use std::{io::{BufRead, BufWriter, Write}, collections::HashMap, convert::TryInto};
use rustrithm::{scanner, math::{modulo::ModU64, num::fast_gcd}};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc226/tasks/abc226_f
//
// ciclic permutation: nCk * (k-1)
//
//  i | j
//  0 | 1~4
// ------------------
//    | o                3P0   <= n-1Ck-1 * (k-1) = (n-i-1)P(j-1)
//    | o o              3P1
//    | o o o            3P2
//    | o o o o          3P3
//
//  i | j
//  1 | 1~3
// ------------------
//  o |
//  o | o               2P0
//  o | o o             2P1
//  o | o o o           2P2
//
//  i   | j
//  2   | 1~2
// ------------------
//  o   |
//  o o |
//  o o | o             1P0
//  o o | o o           1P1
//
//  i     | j
//  3     | 1
// ------------------
//  o     |
//  o o   |
//  o o o |
//  o o o | o           0P0
//
// 1 2 3 4  lcm(1, 1, 1, 1)
//
// 1 2 4 3  lcm(1, 1, 2)
// 1 3 2 4
// 2 1 3 4
// 1 4 3 2
// 3 2 1 4
// 4 2 3 1
//
// 2 1 4 3  lcm(2, 2)
// 4 3 2 1
// 3 4 1 2
//
// 1 3 4 2  lcm(1, 3)
// 1 4 2 3
// 3 2 4 1
// 4 2 1 3
// 2 4 3 1
// 4 1 3 2
// 2 3 1 4
// 3 1 2 4
//
// 2 3 4 1  lcm(0, 4)
// 3 1 4 2
// 4 1 2 3
// 2 4 1 3
// 3 4 2 1
// 4 3 1 2
//
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:u64 = 998244353;
    let zero = ModU64::new(0);
    let n = scan.token::<usize>();
    let k = scan.token::<u32>();

    let fct = Factorial::new(n);
    let mut dp = vec![HashMap::<u64,ModU64<MOD>>::with_capacity(10_000); n+1];
    dp[0].insert(1,zero+1);
    for i in 0..n {
        let (dpl,dpr) = dp.split_at_mut(i+1);
        for (&l,&x) in &dpl[i] {
            for j in 1..=n-i {
                let lcm = lcm(l,j);
                let val = dpr[/*i+1+*/j-1].entry(lcm).or_default();
                *val += x*fct.kperm(n-i-1,j-1);
            }
        }
    }
    let mut ans = zero;
    for (&l,&x) in &dp[n] {
        ans+=zero.sibling(l).pow(k as u64) * x;
    }
    writeln!(out, "{}", ans).ok();
}

pub fn lcm <A:TryInto<i64>, B:TryInto<i64>> (a:A,b:B) -> u64 {
    let a = a.try_into().ok().expect("Unable to cast a to i64");
    let b = b.try_into().ok().expect("Unable to cast b to i64");
    if a == 0 && b==0 {
        return 0;
    }
    return (a*b/fast_gcd(a, b)) as u64;
}


pub struct Factorial<const M:u64> {
    fact: Vec<ModU64<M>>,
    ifact: Vec<ModU64<M>>,
}
impl <const M:u64> Factorial<M>{
    pub fn new(n:usize) -> Self{
        let zero = ModU64::<M>::new(0);
        let mut fact = Vec::<ModU64<M>>::with_capacity(n+1);
        fact.push(zero+1);
        for i in 1..=n {
            fact.push(fact[i-1] * (i) as u64);
        }
        let mut ifact = vec![zero+1;n+1];
        ifact[n] = fact[n].inv();
        for i in (3..=n).rev() {
            ifact[i-1] = ifact[i] * i as u64;
        }
        return Self { fact, ifact };
    }

    pub fn kperm <T: TryInto<usize>>(&self, n:T,k:T) -> ModU64<M> {
        let n = n.try_into().ok().expect("Unable to cast n to usize");
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        if n < k {
            return ModU64::<M>::new(0);
        }
        return self.fact[n]*self.ifact[n-k];
    }

    pub fn kcombi <T: TryInto<usize>>(&self, n:T,k:T) -> ModU64<M> {
        let n = n.try_into().ok().expect("Unable to cast n to usize");
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        if n < k {
            return ModU64::<M>::new(0);
        }
        return self.fact[n]*self.ifact[k]*self.ifact[n-k];
    }

    pub fn fact(&self) -> &Vec<ModU64<M>> { &self.fact }
    pub fn ifact(&self) -> &Vec<ModU64<M>> { &self.ifact }
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
}

#[cfg(test)]
mod abc226f {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 2
";
        let expected = "\
5
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
3 3
";
        let expected = "\
79
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
50 10000
";
        let expected = "\
77436607
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
