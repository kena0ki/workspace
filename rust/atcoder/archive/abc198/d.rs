// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

pub struct Permutations<T> {
    items: Vec<T>,
    swaps: Vec<usize>,
    i: usize,
}

impl <T:Clone> Permutations<T> {
    pub fn new(items: Vec<T>) -> Permutations<T> {
        let swaps = vec![0; items.len()];
        Permutations { items, swaps, i: 0 }
    }
}

impl <T:Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = 1;
            return Some(self.items.clone());
        }
        loop {
            if self.i >= self.items.len() {
                return None;
            }
            if self.swaps[self.i] < self.i {
                break;
            }
            self.swaps[self.i] = 0;
            self.i += 1;
        }
        self.items.swap(self.i, (self.i & 1) * self.swaps[self.i]);
        self.swaps[self.i] += 1;
        self.i = 1;
        return Some(self.items.clone());
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
a
b
c
" , "\
1
2
3
");

    test_macro!(test2, b"\
x
x
y
" , "\
1
1
2
");

    test_macro!(test3, b"\
p
q
p
" , "\
UNSOLVABLE
");

    test_macro!(test4, b"\
abcd
efgh
ijkl
" , "\
UNSOLVABLE
");

    test_macro!(test5, b"\
send
more
money
" , "\
9567
1085
10652
");

}

// https://atcoder.jp/contests/abc198/tasks/abc198_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let va = scan.token_bytes();
    let vb = scan.token_bytes();
    let vc = scan.token_bytes();
    let mut keys = Vec::with_capacity(10);
    for i in 0..va.len() { keys.push(va[i]); }
    for i in 0..vb.len() { keys.push(vb[i]); }
    for i in 0..vc.len() { keys.push(vc[i]); }
    keys.sort_unstable();
    keys.dedup();
    if keys.len() > 10 {
        writeln!(out, "UNSOLVABLE").ok();
        return;
    }
    logln!("{:?}",keys);
    let mut seed = vec![0;10];
    for i in 0..10 {
        seed[i]=i;
    }
    let perm = Permutations::new(seed);
    let mut _cnt = 0;
    for p in perm {
        let mut mp = HashMap::with_capacity(keys.len());
        for i in 0..keys.len() {
            mp.insert(keys[i],p[i]);
        }
        logln!("{:?},{:?}",mp,p);
        let s1 = to_int(&mp,&va);
        let s2 = to_int(&mp,&vb);
        let s3 = to_int(&mp,&vc);
        logln!("{},{},{}",s1,s2,s3);
        //_cnt+=1;
        //if _cnt > 10 {
        //    break;
        //}
        if s1 == 0 || s2 == 0 || s3 == 0 {
            continue;
        }
        if s1 + s2 == s3 {
            writeln!(out, "{}",s1).ok();
            writeln!(out, "{}",s2).ok();
            writeln!(out, "{}",s3).ok();
            return;
        }
    }
    writeln!(out, "UNSOLVABLE").ok();

    fn to_int(mp: &HashMap<u8,usize>, vx: &Vec<u8>) -> usize{
        let mut res = 0;
        if mp[&vx[0]] == 0 {
            return 0;
        }
        for x in vx {
            res += mp[x];
            res *= 10;
        }
        res /= 10;
        return res;
    }
}

