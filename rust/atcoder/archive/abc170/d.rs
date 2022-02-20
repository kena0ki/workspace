// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

use std::collections::BTreeMap;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc170/tasks/abc170_d
fn _solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut va = vec![0;n];
    let big = 1001001;
    let mut cnt = vec![0;big];
    for i in 0..n {
        let a = scan.token::<usize>();
        va[i] = a;
        if cnt[a] >= 1 {
            cnt[a] = 2;
            continue;
        }
        let mut j = a;
        while j < big {
            cnt[j] += 1;
            j+=a;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if cnt[va[i]] == 1 {
            ans += 1;
        }
    }
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc170/tasks/abc170_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut va = vec![0;n];
    let mut map = HashMap::<usize,usize>::new();
    for i in 0..n {
        va[i] = scan.token::<usize>();
        *map.entry(va[i]).or_default() += 1;
    }
    logln!("{:?}", map);
    if let Some(&v) = map.get(&1) {
        if v >= 2 {
            writeln!(out, "0").ok();
        } else {
            writeln!(out, "1").ok();
        }
        return;
    }
    let big = 1001001;
    let f = Factor::new(big);
    let mut ans = 0;
    for i in 0..n {
        let a = va[i];
        if map[&a] >= 2 { continue; }
        let mut flg = true;
        let mut set = HashSet::new();
        set.insert(1);
        for (k,&v) in &f.factors(a) {
            for _ in 0..v {
                for s in set.clone() {
                    let x = k*s;
                    set.insert(x);
                    if x != a && map.contains_key(&x) {
                        logln!("{}", x);
                        flg = false;
                    }
                }
            }
        }
        logln!("{}: {:?}", i, set);
        if flg { ans += 1; }
    }
    writeln!(out, "{}", ans).ok();
}

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
mod abc170d {
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
5
24 11 8 3 1
" , "\
1
");

    test_macro!(test1, b"\
5
24 11 8 3 16
" , "\
3
");

    test_macro!(test2, b"\
4
5 5 5 5
" , "\
0
");

    test_macro!(test3, b"\
10
33 18 45 28 8 19 89 86 2 4
" , "\
5
");


}
