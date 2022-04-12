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
4 6
1 4
2 3
1 3
1 2
3 4
2 4
" , "\
1.5000000000
");

    test_macro!(test2, b"\
3 2
1 2
2 3
" , "\
2.0000000000
");

    test_macro!(test3, b"\
10 33
3 7
5 10
8 9
1 10
4 6
2 5
1 7
6 10
1 4
1 3
8 10
1 5
2 6
6 9
5 6
5 8
3 6
4 8
2 7
2 9
6 7
1 2
5 9
6 8
9 10
3 9
7 8
4 5
2 10
5 7
3 5
4 7
4 9
" , "\
3.0133333333
");

}

// https://atcoder.jp/contests/abc144/tasks/abc144_f
fn _solve_nm(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut adj = vec![Vec::new();n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push(v);
    }
    let inf = 1usize<<60;
    let inf = inf as f64;
    let mut ans = inf;
    for i in 0..n-1 {
        let mut exp = vec![0f64;n];
        for j in (0..n-1).rev() {
            if adj[j].len() == 1 {
                let v = adj[j][0];
                exp[j] = exp[v]+1f64;
                continue;
            }
            let mut max = 0f64;
            let mut now = 0f64;
            for &v in &adj[j] {
                now += exp[v];
                max = exp[v].max(max);
            }
            let mut deg = adj[j].len() as f64;
            if i == j {
                now = now - max;
                deg = deg-1f64;
            }
            //logln!("{},{}",now,deg);
            exp[j] = now/deg + 1f64;
        }
        logln!("{:?}",exp);
        ans = ans.min(exp[0]);
    }
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc144/tasks/abc144_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut adj = vec![Vec::new();n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push(v);
    }
    let mut xp = vec![0f64;n];
    for i in (0..n-1).rev() {
        let mut now = 0f64;
        for &v in &adj[i] {
            now += xp[v];
        }
        let deg = adj[i].len() as f64;
        xp[i] = now/deg + 1f64;
    }
    logln!("xp:{:?}",xp);
    let mut pct = vec![0f64;n];
    pct[0]=1f64;
    for i in 0..n-1 {
        let deg = adj[i].len() as f64;
        for &v in &adj[i] {
            pct[v] += pct[i]/deg;
        }
    }
    logln!("pct:{:?}",pct);
    let mut ans = xp[0];
    for i in 0..n-1 {
        if adj[i].len() <= 1 { continue; }
        let mut max = 0f64;
        let mut now = 0f64;
        for &v in &adj[i] {
            max = max.max(xp[v]);
            now += xp[v];
        }
        let deg = adj[i].len() as f64;
        let aug = (now - max)/(deg-1f64) + 1f64;
        //logln!("{},{}", xp[i],aug);
        //logln!("{},{}", pct[i], xp[i]-aug);
        ans = ans.min(xp[0] - pct[i]*(xp[i]-aug));
    }
    writeln!(out, "{}", ans).ok();
}

