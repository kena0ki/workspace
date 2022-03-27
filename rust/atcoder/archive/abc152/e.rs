// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

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

    test_macro!(test0, b"\
3
1 1 3
" , "\
3
");

    test_macro!(test1, b"\
3
2 3 4
" , "\
13
");

    test_macro!(test2, b"\
3
1000000 999999 999998
" , "\
996989508
");

}

pub fn pow(val:usize, mut power: usize, modulus:usize) -> usize {
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
pub fn inv(val: usize, modulus:usize) -> usize {
    return pow(val, modulus - 2, modulus);
}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:usize = 1000000007;
    let n = scan.token::<usize>();
    let mut va = vec![0;n];
    let fa = Factor::new(1000005);
    let mut cnt = HashMap::<usize,usize>::new();
    for i in 0..n {
        let a = scan.token::<usize>();
        va[i] = a;
        if a == 1 { continue; }
        let mp = fa.factors(a);
        for (&k,&v) in &mp {
            let c = cnt.entry(k).or_default();
            if *c<v {
                cnt.insert(k,v);
            }
        }
    }
    let mut lcm = 1;
    for (&k,&v) in &cnt {
        for _ in 0..v {
            lcm = (lcm*k)%MOD;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans += lcm * inv(va[i], MOD);
        ans %= MOD;
        logln!("{}",ans);
    }
    writeln!(out, "{}",ans).ok();
}

