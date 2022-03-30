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
3
1 2
2 3
" , "\
125000001
");

    test_macro!(test2, b"\
4
1 2
2 3
3 4
" , "\
375000003
");

    test_macro!(test3, b"\
4
1 2
1 3
1 4
" , "\
250000002
");

    test_macro!(test4, b"\
7
4 7
3 1
2 6
5 2
7 1
2 7
" , "\
570312505
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

// https://atcoder.jp/contests/abc149/tasks/abc149_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:usize = 1000000007;
    let n = scan.token::<usize>();
    let mut adj = vec![Vec::new();n];
    for _ in 0..n-1 {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut two = vec![0;n+1];
    two[0] = 1;
    for i in 0..n {
        two[i+1] = two[i]*2%MOD;
    }
    let mut twos = vec![0;n+1];
    for i in 0..n {
        twos[i+1] = two[i]+twos[i]%MOD;
    }
    logln!("{:?}", twos);
    let res = dfs(&adj,0,usize::max_value(),0,n,&twos);
    let ans = res.0*inv(two[n], MOD)%MOD;
    writeln!(out, "{}",ans).ok();
    fn dfs(adj:&Vec<Vec<usize>>, u:usize, p:usize, d: usize, n:usize, twos: &Vec<usize>) -> (usize,usize) {
        let inp = d;
        let mut out = d;
        let mut sum = 0;
        for &v in &adj[u] {
            if p == v { continue; }
            let cres = dfs(adj,v,u,out+1,n,twos);
            let suball = cres.1-inp;
            let sub = cres.1-out;
            let par = n-suball-1;
            out = cres.1;
            sum = (sum+cres.0)%MOD;
            if sub >= 1 && par >= 1 {
                logln!("{},{}",sub,par);
                let subc = twos[sub]%MOD;
                let parc = twos[par]%MOD;
                sum += (subc*parc)%MOD;
                sum %=MOD;
            }
        }
        logln!("{}",sum);
        return (sum,out);
    }
}

