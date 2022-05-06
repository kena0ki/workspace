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
3 1
1 2 3
" , "\
665496245
");

    test_macro!(test2, b"\
2 2
1 2
" , "\
499122182
");

    test_macro!(test3, b"\
10 1000000000
998244350 998244351 998244352 998244353 998244354 998244355 998244356 998244357 998244358 998244359
" , "\
138512322
");

}

const MOD:usize = 998244353;

fn pow(x:usize,p:usize) -> usize{
    if p == 0 { return 1; }
    let mut r = pow(x,p>>1);
    r = r*r%MOD;
    if p&1 == 1 { r=x*r%MOD; }
    return r;
}

fn inv(x:usize) -> usize {
    return pow(x,MOD-2);
}

// https://atcoder.jp/contests/abc231/tasks/abc231_g
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut va = vec![0;n];
    for  i in 0..n {
        let a = scan.token::<usize>();
        va[i] = a;
    }
    let mut fct = vec![0;n+1];
    let mut ifct = vec![0;n+1];
    fct[0] = 1;
    for i in 0..n {
        fct[i+1] = fct[i]*(i+1)%MOD;
    }
    ifct[n] = inv(fct[n]);
    for i in (0..n).rev() {
        ifct[i] = ifct[i+1]*(i+1)%MOD;
    }
    let mut fct2 = vec![0;n+1];
    let mut ifct2 = vec![0;n+1];
    let l = k.saturating_sub(n);
    fct2[0]=l.max(1);
    for i in 0..n {
        fct2[i+1] = fct2[i]*(l+i+1)%MOD;
    }
    ifct2[n]=inv(fct2[n]);
    for i in (0..n).rev() {
        ifct2[i] = ifct2[i+1]*(l+i+1)%MOD;
    }
    logln!("{:?},{:?}",fct,fct2);
    logln!("{:?},{:?}",ifct,ifct2);
    let comb = |x,y| {
        return fct2[x-l]*ifct2[x-l-y]%MOD*ifct[y]%MOD;
    };

    let mut dp = vec![0;n+1];
    dp[0] = 1;
    for i in 0..n {
        let pre = dp.clone();
        for j in 0..n {
            dp[j+1] += pre[j]*va[i];
            dp[j+1] %= MOD;
        }
    }
    logln!("{:?}",dp);
    let mut ans = 0;
    for i in 0..n.min(k)+1 {
        let now = dp[n-i]*comb(k,i)%MOD*pow(n,k-i)%MOD*fct[i]%MOD;
        logln!("{},{},{},{}",k,i,comb(k,i),now);
        ans = (ans+now)%MOD;
    }
    logln!("{}",ans);
    let d = inv(pow(n,k));
    ans = (ans * d) %MOD;
    writeln!(out, "{}", ans).ok();
}

