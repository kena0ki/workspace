// template

use std::io::{BufRead, BufWriter, Write};

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

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc207/tasks/abc207_f
// WIP
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut adj = vec![Vec::<usize>::with_capacity(n);n];
    for _ in 0..n-1 {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push(v);
        adj[v].push(u);
    }

    const MOD: usize = 1000000007;
    let res = f(&adj, 0, usize::max_value());
    fn f(adj: &Vec<Vec<usize>>, u: usize, p:usize) -> Vec<Vec<usize>> {
        if adj[u].len() <= 1 && p != usize::max_value() {
            return vec![vec![0,1],vec![1,0]];
        }
        let mut res = vec![vec![0,1]];
        for &v in &adj[u] {
            if v == p { continue; }
            let res2 = f(adj,v,u);
            let len = res.len()+res2.len()-1;
            let mut nxt = vec![vec![0;2];len];
            for i1 in 0..res.len() { for i2 in 0..res2.len() {
                for j1 in 0..3 { for j2 in 0..3 {
                    if j1 == 1 || j2 == 1 {
                        nxt[i1+i2][1] += res[i1][j1] * res2[i2][j2];
                        nxt[i1+i2][1] %= MOD;
                    } else {
                        nxt[i1+i2][0] += res[i1][j1] * res2[i2][j2];
                        nxt[i1+i2][0] %= MOD;
                    }
                }}
                logln!("{:?}", nxt);
            }}
            let mut nxt2 = vec![vec![0;2];len];
            for i in 0..len { for j in 0..2 { for nj in 0..2 {
                let ni = if j == 0 && nj == 0 {
                    i
                } else if j == 1 || nj == 1 {
                    i+1
                } else { // nj == 1 && j == 0
                    i+2
                };
                if nxt[i][j] > 0 {
                    nxt2[ni][nj] = nxt[i][j];
                }
            }}}
            res = nxt2;
        }
        return res;
    }
    for i in 0..n+1 {
        let ans = if res.len() > i { res[i][0] + res[i][1] } else { 0 };
        writeln!(out, "{}", ans).ok();
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

#[cfg(test)]
mod abc208e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
1 3
1 2
";
        let expected = "\
1
0
2
5
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
5
1 3
4 5
1 5
2 3
";
        let expected = "\
1
0
2
5
7
17
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
10
6 10
1 8
2 7
5 6
3 8
3 4
7 10
4 9
2 8
";
        let expected = "\
1
0
3
8
15
32
68
110
196
266
325
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
