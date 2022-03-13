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
4 3
1 4
2 4
3 4
3
1 2 3
" , "\
5
");

    test_macro!(test2, b"\
4 3
1 4
2 4
1 2
3
1 2 3
" , "\
-1
");

    test_macro!(test3, b"\
10 10
3 9
3 8
8 10
2 10
5 8
6 8
5 7
6 7
1 6
2 4
4
1 2 7 9
" , "\
11
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut va = vec![Vec::with_capacity(n);n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        va[u].push(v);
        va[v].push(u);
    }
    let k = scan.token::<usize>();
    let mut mc = HashMap::with_capacity(k);
    for i in 0..k {
        let c = scan.token::<usize>()-1;
        mc.insert(c,i);
    }
    let inf = 1usize<<60;
    //let inf = 100usize;
    let mut dc = vec![vec![inf;k];k];
    for (&c,&i) in &mc {
        let mut que = VecDeque::<usize>::with_capacity(n);
        let mut dist = vec![inf;n];
        que.push_back(c);
        dist[c] = 0;
        while let Some(u) = que.pop_front() {
            for &v in &va[u] {
                if dist[v] < inf { continue; }
                let dv = dist[u]+1;
                dist[v]=dist[v].min(dv);
                if mc.contains_key(&v) {
                    dc[i][mc[&v]] = dv;
                }
                que.push_back(v);
            }
        }
    }
    logln!("{:?}",dc);
    let k2 = 1<<k;
    let mut dp = vec![vec![inf;k];k2];
    for i in 0..k {
        dp[1<<i][i] = 1;
    }
    for i in 0..k2 { for j in 0..k {
        if i>>j&1 == 0 { continue; }
        if dp[i][j] == inf { continue; }
        for l in 0..k {
            if i>>l&1 == 1 { continue; }
            dp[i|1<<l][l] = dp[i|1<<l][l].min(dp[i][j]+dc[j][l]);
        }
    }
    logln!("{:?}",dp);
    }
    let mut ans = inf;
    for i in 0..k {
        ans = dp[k2-1][i].min(ans);
    }
    if ans == inf {
        writeln!(out, "-1").ok();
    } else {
        writeln!(out, "{}", ans).ok();
    }

}

