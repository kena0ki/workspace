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
8 8
1 2 a
2 3 b
1 3 c
3 4 b
4 5 a
5 6 c
6 7 b
7 8 a
" , "\
10
");

    test_macro!(test2, b"\
4 5
1 1 a
1 2 a
2 3 a
3 4 b
4 4 a
" , "\
5
");

    test_macro!(test3, b"\
3 4
1 1 a
1 2 a
2 3 b
3 3 b
" , "\
-1
");

}

// https://atcoder.jp/contests/abc197/tasks/abc197_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut adj = vec![Vec::with_capacity(n);n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        let c = scan.token_bytes()[0] as char;
        adj[u].push((v,c));
        adj[v].push((u,c));
    }
    let mut que = VecDeque::with_capacity(n);
    let mut dist = HashMap::<(usize,usize),usize>::with_capacity(n*n);
    let inf = 1<<60;
    let mut ans = inf;
    que.push_back((0,n-1));
    dist.insert((0,n-1),0);
    while let Some((u1,u2)) = que.pop_front() {
        for &(v1,c1) in &adj[u1] {
            for &(v2,c2) in &adj[u2] {
                if dist.contains_key(&(v1,v2)) { continue; }
                if c1 != c2 { continue; }
                let d = dist[&(u1,u2)]+1;
                if v1==v2 {
                    ans = ans.min(2*d);
                    break;
                } else if v1==u2 && v2==u1 {
                    ans = ans.min(2*d-1);
                    break;
                }
                *dist.entry((v1,v2)).or_default() = d;
                que.push_back((v1,v2));
            }
        }
    }
    logln!("{:?}",ans);
    logln!("{:?}",dist);

    if ans == inf {
        writeln!(out, "{}", -1).ok();
    } else {
        writeln!(out, "{}", ans).ok();
    }
}

