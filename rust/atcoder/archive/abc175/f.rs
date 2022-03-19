// template

use std::{io::{BufRead, BufWriter, Write}, cmp::Reverse};
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
ba 3
abc 4
cbaa 5
" , "\
7
");

    test_macro!(test2, b"\
2
abcab 5
cba 3
" , "\
11
");

    test_macro!(test3, b"\
4
ab 5
cba 3
a 12
ab 10
" , "\
8
");

    test_macro!(test4, b"\
2
abc 1
ab 2
" , "\
-1
");

}

// https://atcoder.jp/contests/abc175/tasks/abc175_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vs1 = Vec::with_capacity(n);
    let mut vs2 = Vec::with_capacity(n);
    let mut vc = Vec::with_capacity(n);
    for _ in 0..n {
        let s1 = scan.token_bytes();
        let mut s2 = s1.clone();
        s2.reverse();
        vs1.push(s1);
        vs2.push(s2);
        let c = scan.token::<usize>();
        vc.push(c);
    }
    let mut que = BinaryHeap::with_capacity(n);
    let mut dist = HashMap::<(usize,Vec<u8>),usize>::new();
    let inf = 1<<60;
    let mut ans = inf;
    for i in 0..n {
        let s = vs1[i].clone();
        let key = (0,s.clone());
        if let Some(&d) = dist.get(&key) {
            if d <= vc[i] { continue; }
        }
        dist.insert(key.clone(),vc[i]);
        que.push((Reverse(vc[i]),key));
        let mut rs = s.clone();
        rs.reverse();
        if s == rs {
            ans = ans.min(vc[i]);
        }
    }
    while let Some((Reverse(d),(o,s))) = que.pop() {
        let pd = dist[&(o,s.clone())];
        if pd < d {
            continue;
        }
        let vs = if o == 0 { &vs2 } else { &vs1 };
        for i in 0..n {
            let mut s1 = s.clone();
            let mut s2 = vs[i].clone();
            let mut no = o;
            logln!("s2,s1:{:?},{:?}",s2,s1);
            if s2.len() > s1.len() {
                no = o^1;
                std::mem::swap(&mut s2, &mut s1);
            }
            let len = s2.len();
            if s2[..len] == s1[..len] {
                let ns = s1[len..].to_vec();
                logln!("{:?}",ns);
                if let Some(&nd) = dist.get(&(no,ns.clone())) {
                    if nd <= pd + vc[i] {
                        continue;
                    }
                }
                let nd = pd + vc[i];
                dist.insert((no,ns.clone()),nd);
                que.push((Reverse(nd),(no,ns.clone())));
                let mut rns = ns.clone();
                rns.reverse();
                if ns == rns {
                    ans = ans.min(nd);
                }
            }
        }
    }
    logln!("{:?}",dist);
    if ans < inf {
        writeln!(out, "{}", ans).ok();
    } else {
        writeln!(out, "-1").ok();
    }
}


