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
4 1
2 3 4 1
" , "\
3
");

    test_macro!(test2, b"\
6 2
8 6 9 1 2 1
" , "\
7
");

    test_macro!(test3, b"\
10 0
1 1000000000 1 1000000000 1 1000000000 1 1000000000 1 1000000000
" , "\
4999999996
");

}

// https://atcoder.jp/contests/abc145/tasks/abc145_f
fn _solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut vh = vec![0;n];
    for i in 0..n {
        let h = scan.token::<usize>();
        vh[i] = h;
    }
    let mut set = BTreeSet::new();
    set.insert(0);
    for i in 0..n {
        set.insert(vh[i]);
    }
    let mut mp = BTreeMap::new();
    let mut mp2 = BTreeMap::new();
    let mut cnt = 0usize;
    for &h in &set {
        mp.insert(cnt,h);
        mp2.insert(h,cnt);
        cnt+=1;
    }
    let mh = mp.len();
    let inf = 1usize<<60;
    //let inf = 1usize<<6;
    let mut dp = vec![vec![vec![inf;mh];k+1];n+1];
    dp[0][0][0] = 0;
    for i in 0..n { for j in 0..k+1 { for l in 0..mh {
        if dp[i][j][l] == inf { continue; }
        let h = vh[i];
        let nl = mp2[&h];
        dp[i+1][j][nl] = dp[i+1][j][nl].min(dp[i][j][l] + h.saturating_sub(mp[&l]));
        if j==k { continue; }
        dp[i+1][j+1][l] = dp[i+1][j+1][l].min(dp[i][j][l]);
        logln!("{:?}",dp[i+1]);
    }}}
    let mut ans = inf;
    for j in 0..k+1 {
        ans = dp[n][j].iter().min().copied().unwrap();
    }
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc145/tasks/abc145_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let m = n-k;
    let mut vh = vec![0;n+1];
    for i in 0..n {
        let h = scan.token::<usize>();
        vh[i+1] = h;
    }
    let inf = 1usize<<60;
    //let inf = 1usize<<6;
    let mut dp = vec![vec![inf;m+1];n+1];
    dp[0][0] = 0;
    for i in 0..n { for j in 0..m+1 {
        if j==m { continue; }
        let mut val = inf;
        for l in 0..i+1 {
            if dp[l][j] == inf { continue; }
            logln!("{},{},{:?}",l,j,dp[l][j]);
            let hi = vh[i+1];
            let hl = vh[l];
            val = val.min(dp[l][j] + hi.saturating_sub(hl));
        }
        dp[i+1][j+1] = dp[i+1][j+1].min(val);
        logln!("{:?}",dp[i+1]);
    }}
    let mut ans = inf;
    for i in 0..n+1 {
        ans = dp[i][m].min(ans);
    }
    writeln!(out, "{}", ans).ok();
}

