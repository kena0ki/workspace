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
mod abc165c {
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
3 4 3
1 3 3 100
1 2 2 10
2 3 2 10
" , "\
110
");

    test_macro!(test2, b"\
4 6 10
2 4 1 86568
1 4 0 90629
2 3 0 90310
3 4 1 29211
3 4 3 78537
3 4 2 8580
1 2 1 96263
1 4 2 2156
1 2 0 94325
1 4 3 94328
" , "\
357500
");

    test_macro!(test3, b"\
10 10 1
1 10 9 1
" , "\
1
");

}

// https://atcoder.jp/contests/abc165/tasks/abc165_c
// WA
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let q = scan.token::<usize>();
    let mut vq = Vec::with_capacity(q);
    for _ in 0..q {
        let a = scan.token::<usize>()-1;
        let b = scan.token::<usize>()-1;
        let c = scan.token::<usize>();
        let d = scan.token::<usize>();
        vq.push((a,b,c,d));
    }
    let mut va = vec![None;n];
    let mut memo = vec![HashMap::<String,usize>::new();q];
    let ans = f(&vq, n, m, &mut va, 0,&mut memo);
    writeln!(out, "{}", ans).ok();
    fn f(vq: &Vec<(usize,usize,usize,usize)>,
        n:usize, m:usize, va:&mut Vec<Option<usize>>, qi:usize,
        memo: &mut Vec<HashMap::<String,usize>>) -> usize {
        logln!("{}: {:?}", qi,va);
        let vastr = format!("{:?}",va);
        if qi < vq.len() && memo[qi].contains_key(&format!("{:?}",&vastr)) {
            return memo[qi][&vastr];
        }
        if qi >= vq.len(){
            return 0;
        }
        let mut res = f(vq,n,m,va,qi+1,memo);
        let (a,b,c,d) = vq[qi];
        let mut amin = 0;
        let mut amax = m;
        let mut bmin = 0;
        let mut bmax = m;
        for j in 0..n {
            if let Some(v) = va[j] {
                if j < a { amin = amin.max(v); }
                else if j > a { amax = amax.min(v); }
                if j > b { bmin = bmin.max(v); }
                else if j > b { bmax = bmax.min(v); }
            }
        }
        logln!("{}, {}, {}, {}", amin, amax, bmin, bmax);
        if va[a].is_some() && va[b].is_some() {
            if va[a].unwrap() + c == va[b].unwrap() {
                res = res.max(f(vq,n,m,va,qi+1,memo) +d);
            }
        } else if va[a].is_some() {
            let na = va[a].unwrap();
            if na + c >= bmin && na + c <= bmax {
                va[b] = Some(na + c);
                res = res.max(f(vq,n,m,va,qi+1,memo) +d);
                va[b] = None;
            }
        } else if va[b].is_some() {
            let nb = va[b].unwrap();
            if nb >= amin + c && nb <= amax + c {
                va[a] = Some(nb - c);
                res = res.max(f(vq,n,m,va,qi+1,memo) +d);
                va[a] = None;
            }
        } else {
            for j in amin..(amax+1).saturating_sub(c) {
                if j + c < bmin || j + c > bmax {
                    continue;
                }
                va[a] = Some(j);
                va[b] = Some(j+c);
                res = res.max(f(vq,n,m,va,qi+1,memo) +d);
            }
            va[a] = None;
            va[b] = None;
        }
        memo[qi].insert(vastr,res);
        return res;
    }
}
