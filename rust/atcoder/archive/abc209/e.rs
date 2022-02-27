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
abcd
bcda
ada
" , "\
Aoki
Takahashi
Draw
");

    test_macro!(test2, b"\
1
ABC
" , "\
Draw
");

    test_macro!(test3, b"\
5
eaaaabaa
eaaaacaa
daaaaaaa
eaaaadaa
daaaafaa
" , "\
Takahashi
Takahashi
Takahashi
Aoki
Takahashi
");

}

// https://atcoder.jp/contests/abc209/tasks/abc209_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vvb = Vec::with_capacity(n);
    let mut set = HashSet::with_capacity(n);
    for _ in 0..n {
        let vb = scan.token_bytes();
        set.insert(vb[..3].to_owned());
        set.insert(vb[vb.len()-3..vb.len()].to_owned());
        vvb.push(vb);
    }
    let mut mp = HashMap::with_capacity(n);
    let mut m = 0;
    for s in &set {
        mp.insert(s.to_owned(),m);
        m += 1;
    }
    let mut adj = vec![Vec::new(); m];
    let mut degs = vec![0usize;m];
    let mut dup = HashSet::with_capacity(n);
    for i in 0..n {
        let vb = &vvb[i];
        let v = mp[&vb[..3]];
        let u = mp[&vb[vb.len()-3..vb.len()]];
        if dup.contains(&(u,v)) { continue; }
        dup.insert((u,v));
        adj[u].push(v);
        degs[v] += 1;
    }
    logln!("{:?}",mp);
    let mut que = VecDeque::with_capacity(n);
    let mut memo = vec![2;m];
    for i in 0..m {
        if degs[i] == 0 {
            memo[i] = 0;
            que.push_back(i);
        }
    }
    while let Some(u) = que.pop_front() {
        for &v in &adj[u] {
            if memo[v] <= 1 { continue; }
            degs[v] -= 1;
            if memo[u] == 0 {
                memo[v] = 1;
                que.push_back(v);
            } else if memo[u] == 1 {
                if degs[v] == 0 {
                    memo[v] = 0;
                    que.push_back(v);
                }
            }
            logln!("{:?}", memo);
        }
    }
    for i in 0..n {
        let vb = &vvb[i];
        let b = &vvb[i][vb.len()-3..vb.len()];
        if memo[mp[b]] == 0 {
            writeln!(out, "Takahashi").ok();
        } else if memo[mp[b]] == 1 {
            writeln!(out, "Aoki").ok();
        } else {
            writeln!(out, "Draw").ok();
        }
    }
}

