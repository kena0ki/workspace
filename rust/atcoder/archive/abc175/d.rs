// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;
use std::io;
use std::str;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc175/tasks/abc175_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut p = Vec::with_capacity(n);
    for _ in 0..n {
        let pi = scan.token::<usize>();
        p.push(pi-1);
    }
    let mut c = Vec::with_capacity(n);
    for _ in 0..n {
        let ci = scan.token::<i64>();
        c.push(ci);
    }
    let mut cycles = vec![(Vec::<usize>::with_capacity(n),0i64);n];
    let mut vis = vec![false; n];
    for i in 0..n {
        if vis[i] == true { continue; }
        let mut x = i;
        let mut cycle = Vec::with_capacity(n);
        let mut tot = 0i64;
        while vis[x] == false {
            vis[x] = true;
            cycle.push(x);
            x = p[x];
            tot += c[x];
        }
        cycles.push((cycle,tot));
    }
    // for i in 0..n {
    //     if vis[i] == false {
    //         let res = dfs(i,&p,&mut vis);
    //         let mut tot = 0;
    //         for &i in &res {
    //             tot += c[i];
    //         }
    //         cycles.push((res, tot));
    //     }
    // }
    // fn dfs(v: usize, p:&Vec<usize>, vis: &mut Vec<bool>) -> Vec<usize> {
    //     if vis[v] == true {
    //         return vec![];
    //     }
    //     vis[v] = true;
    //     let mut res = dfs(p[v], p, vis);
    //     res.push(v);
    //     return res;
    // }
    let mut ans = c.iter().max().copied().unwrap();
    for (cy, cytot) in cycles {
        let len = cy.len();
        for i in 0..len {
            let mut tot = 0i64;
            let end = len.min(k);
            for j in 0..end {
                tot += c[cy[(j+i)%len]];
                ans = ans.max(tot);
            }
            if k < len { continue; }
            let q = k/len-1;
            let r = k%len;
            let mut tot = q as i64* cytot;
            for j in 0..(len+r) {
                tot += c[cy[(j+i)%len]];
                ans = ans.max(tot);
            }
        }
    }
    writeln!(out, "{}",ans).ok();

}

// https://atcoder.jp/contests/abc175/tasks/abc175_d
// RE
fn _solve_re(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut p = Vec::with_capacity(n);
    for _ in 0..n {
        let pi = scan.token::<usize>();
        p.push(pi-1);
    }
    let mut c = Vec::with_capacity(n);
    for _ in 0..n {
        let ci = scan.token::<i64>();
        c.push(ci);
    }
    let mut dp = vec![vec![0;n];k+1];
    let mut ans = c.iter().max().copied().unwrap();
    for i in 0..k {
        for j in 0..n {
            let nj = p[j];
            let next = dp[i][j] + c[nj];
            dp[i+1][nj] = next;
            ans = ans.max(next);
        }
    }
    writeln!(out, "{}", ans).ok();
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
impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, buffer: vec![] }
    }
    pub fn token<T: str::FromStr>(&mut self) -> T {
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
mod abc175d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
5 2
2 4 5 1 3
3 4 -10 -8 8
";
        let expected = "\
8
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
2 3
2 1
10 -7
";
        let expected = "\
13
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
3 3
3 1 2
-1000 -2000 -3000
";
        let expected = "\
-1000
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test4() {
        let input: &[u8] = b"\
10 58
9 1 6 7 8 4 3 2 10 5
695279662 988782657 -119067776 382975538 -151885171 -177220596 -169777795 37619092 389386780 980092719
";
        let expected = "\
29507023469
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
