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
2 2
" , "\
5
");

    test_macro!(test2, b"\
3 3
" , "\
79
");

    test_macro!(test3, b"\
50 10000
" , "\
77436607
");

}

const MOD:usize = 998244353;

pub fn fast_gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn pow(x:usize,p:usize) -> usize {
    if p == 0 {
        return 1;
    }
    let mut r = pow(x,p>>1);
    r*=r;
    r%=MOD;
    if p&1 == 1 {
        r*=x;
    }
    r%=MOD;
    return r;
}

fn inv(x:usize) -> usize {
    return pow(x,MOD-2);
}


// https://atcoder.jp/contests/abc226/tasks/abc226_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let zk = scan.token::<usize>();
    let mut fct = vec![0;n+1];
    let mut ifct = vec![0;n+1];
    fct[0]=1;
    for i in 0..n {
        fct[i+1] = fct[i]*(i+1)%MOD;
    }
    ifct[n] = inv(fct[n])%MOD;
    for i in (0..n).rev() {
        ifct[i] = ifct[i+1]*(i+1)%MOD;
    }
    let comb = |n,k| {
        return fct[n]*ifct[n-k]%MOD*ifct[k]%MOD;
    };
    let mut dp = vec![HashMap::new();n+1];
    *dp[0].entry(1).or_default() = 1;
    for i in 0..n { for j in 1..n-i+1 {
        let mp = dp[i].clone();
        for (&k,&pv) in &mp {
            let g = fast_gcd(k,j);
            let lcm = k/g*j;
            let nv = pv*comb(n-i-1,j-1)%MOD*fct[j-1];
            let val = dp[i+j].entry(lcm).or_default();
            *val = (*val+nv%MOD)%MOD;
        }
        //logln!("{},{:?}",i,dp);
    } }
    let mut ans = 0;
    for (&k,&v) in &dp[n] {
        ans += pow(k,zk)%MOD*v%MOD;
        ans %= MOD;
    }
    writeln!(out, "{}", ans).ok();
}

