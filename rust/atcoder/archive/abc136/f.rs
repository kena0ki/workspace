// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct FenwickTree<T>{
    identity: T,
    n: usize,
    bit: Vec<T>,
}
impl <T:Clone+Copy+Add<Output=T>+Sub<Output=T>> FenwickTree<T>{
    pub fn new(n: usize, identity: T) -> FenwickTree<T> {
        return Self {
            identity,
            n,
            bit: vec![identity; n+1],
        };
    }
    /// Adds the value to the given index.
    pub fn add(&mut self, mut idx: usize,a: T){
        if idx >= self.n {
            panic!("Index out of bound. length:{}, but idx:{}.", self.n, idx);
        }
        idx+=1;
        loop {
            if idx > self.n {
                break;
            }
            self.bit[idx] = self.bit[idx]+a;
            let idx64 = idx as i64;
            idx+=(idx64 & -idx64) as usize;
        }
    }
    /// Returns the summary of values between l and r-1.
    pub fn sum(&self, l:usize, r:usize) -> T {
        if l>r {
            panic!("Invalid range. l:{} > r:{}", l, r);
        }
        return self.sum0(r) - self.sum0(l);
    }
    fn sum0(&self, mut idx: usize) -> T {
        //idx+=1;
        let mut ret = self.identity;
        loop {
            if idx<=0 {
                break;
            }
            ret = ret+self.bit[idx];
            let idx64 = idx as i64;
            idx-=(idx64 & -idx64) as usize;
        }
        return ret;
    }
}

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
}

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}
impl<R: ::std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, buffer: vec![] }
    }
    pub fn token<T: ::std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

#[cfg(test)]
mod abc999x {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let output = &mut Vec::new();
                let scan = &mut Scanner::new($input as &[u8]);
                solve(scan, output);
                assert_eq!($expected, std::str::from_utf8(output).unwrap());
            }
        };
    }

    test_macro!(test1, b"\
3
-1 3
2 1
3 -2
" , "\
13
");

    test_macro!(test2, b"\
4
1 4
2 1
3 3
4 2
" , "\
34
");

    test_macro!(test3, b"\
10
19 -11
-3 -12
5 3
3 -15
8 -14
-9 -20
10 -9
0 2
-7 17
6 -6
" , "\
7222
");

}

pub fn pow(val:i64, mut power: i64, modulus:i64) -> i64 {
    let mut square = val;
    let mut ret = 1;
    while 0 < power {
        if (power & 1) == 1{
            ret *= square;
            ret %= modulus;
        }
        square *= square;
        square %= modulus;
        power >>= 1;
    }
    return ret;
}
pub fn inv(val: i64, modulus:i64) -> i64 {
    return pow(val, modulus - 2, modulus);
}

// https://atcoder.jp/contests/abc136/tasks/abc136_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:i64 = 998244353;
    let n = scan.token::<usize>();
    let mut vx = Vec::with_capacity(n);
    let mut vy = Vec::with_capacity(n);
    for i in 0..n {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        vx.push((x,i));
        vy.push((y,i));
    }
    let mut setx = BTreeSet::new();
    let mut sety = BTreeSet::new();
    for i in 0..n {
        setx.insert(vx[i].0);
        sety.insert(vy[i].0);
    }
    let mut mpx = BTreeMap::new();
    let mut cnt = 0;
    for &x in &setx {
        mpx.insert(x,cnt);
        cnt+=1;
    }
    let mut mpy = BTreeMap::new();
    let mut cnt = 0;
    for &y in &sety {
        mpy.insert(y,cnt);
        cnt+=1;
    }
    let mut vxy = vec![(0,0);n];
    for i in 0..n {
        let x = vx[i].0;
        let y = vy[i].0;
        vxy[mpx[&x]] = (mpy[&y],i);
    }

    let mut ini = pow(2,n as i64,MOD) + MOD -1;
    ini %= MOD;
    let mut vans = vec![ini;n];
    vx.sort_unstable();
    for i in 0..n {
        let (_,j) = vx[i];
        vans[j] += MOD - (pow(2, i as i64, MOD) + MOD - 1);
        vans[j] %= MOD;
        vans[j] += MOD - (pow(2, (n-1-i) as i64,MOD) + MOD - 1);
        vans[j] %= MOD;
    }
    vy.sort_unstable();
    for i in 0..n {
        let (_,j) = vy[i];
        vans[j] += MOD - (pow(2, i as i64, MOD) + MOD - 1);
        vans[j] %= MOD;
        vans[j] += MOD - (pow(2, (n-1-i) as i64,MOD) + MOD - 1);
        vans[j] %= MOD;
    }
    logln!("{:?}",vans);

    let mut fw = FenwickTree::new(n,0);
    for i in 0..n {
        let (y,j) = vxy[i];
        let s = fw.sum(0,y);
        vans[j] += pow(2, s, MOD) + MOD - 1;
        vans[j] %= MOD;
        vans[j] += pow(2, i as i64 -s, MOD) + MOD -1;
        vans[j] %= MOD;
        fw.add(y,1);
    }

    let mut fw = FenwickTree::new(n,0);
    for i in (0..n).rev() {
        let (y,j) = vxy[i];
        let s = fw.sum(0,y);
        vans[j] += pow(2, s, MOD) + MOD - 1;
        vans[j] %= MOD;
        vans[j] += pow(2, (n-1-i) as i64 -s, MOD) + MOD -1;
        vans[j] %= MOD;
        fw.add(y,1);
    }
    logln!("{:?}",vans);
    let mut ans = vans.iter().sum::<i64>();
    ans %= MOD;

    writeln!(out, "{}", ans).ok();
}

