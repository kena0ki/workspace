// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc172/tasks/abc172_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    const BIG:usize = 10000000;
    let p = Factor::new(BIG);
    let mut ans = 1;
    for i in 2..(BIG).min(n)+1 {
        let mut x =1;
        let f = p.factors(i);
        for (&_,&v) in &f {
            x *= v+1;
        }
        logln!("{:?}", f);
        ans += i * x;
    }
    writeln!(out, "{}", ans).ok();
    return;
    //if n <= BIG {
    //    writeln!(out, "{}", ans).ok();
    //    return;
    //}
    //for i in (BIG+1)..n+1 {
    //    let mut w = i;
    //    let mut x = 1;
    //    for &j in &p.primes {
    //        let mut cnt = 0;
    //        while w % j > 0 {
    //            w /= j;
    //            cnt+=1;
    //        }
    //        x *= cnt+1;
    //    }
    //    if w > 1 {
    //        x *= 2;
    //    }
    //    ans += i * x;
    //}
    //writeln!(out, "{}", ans).ok();
}

use std::collections::BTreeMap;

pub struct Factor {
    sieve: Vec<usize>,
    pub primes: Vec<usize>,
}

impl Factor {
    pub fn new(n:usize) -> Self{
        let mut sieve = vec![0;n+1];  // i=0,1 elements are not used.
        let mut primes = Vec::new();
        for i in 2..n {
            if sieve[i] > 0 {
                continue;
            }
            primes.push(i);
            sieve[i] = i;
            let mut j = i*i;
            while j <= n {
                if sieve[j] == 0 {
                    sieve[j] = i;
                }
                j+=i;
            }
        }
        return Self {sieve, primes};
    }
    pub fn factors(&self, mut x: usize) -> BTreeMap<usize,usize> {
        if x < 2 {
            panic!("x should be greater than 1.");
        }
        let mut facts = BTreeMap::new();
        while x > 1 {
            *facts.entry(self.sieve[x]).or_default() +=1;
            x = x / self.sieve[x];
        }
        return facts;
    }
    pub fn factors_flat(&self, mut x:usize) -> Vec<usize> {
        if x < 2 {
            panic!("x should be greater than 1.");
        }
        let mut facts = Vec::new();
        while x > 1 {
            facts.push(self.sieve[x]);
            x = x / self.sieve[x];
        }
        return facts;
    }
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
mod abc172d {
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
4
" , "\
23
");
    test_macro!(test2, b"\
100
" , "\
26879
");
    test_macro!(test3, b"\
10000000
" , "\
838627288460105
");

}
