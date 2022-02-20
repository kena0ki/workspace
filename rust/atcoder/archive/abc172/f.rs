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

// https://atcoder.jp/contests/abc172/tasks/abc172_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut av = vec![0;n];
    for i in 0..n {
        av[i] = scan.token::<usize>();
    }
    let mut x = if n >2 { av[2] } else { 0 };
    for i in 3..n {
        x ^= av[i];
    }
    let a = av[0];
    let b = av[1];
    let f = |mut num| {
        let mut cnt = 0;
        while num > 0 {
            num /= 2;
            cnt += 1;
        }
        return cnt;
    };
    let len = f(a).max(f(b)).max(f(x));
    const INF:usize = 1001001001001;
    let mut memo = vec![vec![HashMap::<usize,usize>::new();2];len+1];
    logln!("{:b}", a);
    logln!("{:b}", b);
    logln!("{:b}", x);
    let ans = fnc(0, 0, a, b, x, &mut memo);
    if ans == INF || ans == a {
        writeln!(out, "-1").ok();
        return;
    }
    writeln!(out, "{}", ans).ok();
    fn fnc(i:usize, j:usize, a:usize, b:usize, x:usize, memo: &mut Vec<Vec<HashMap<usize,usize>>>) -> usize {
        if i >= memo.len() {
            return 0;
        }
        if memo[i][j].contains_key(&a) {
            return memo[i][j][&a];
        }
        logln!("i: {}", i);
        let ai = a >> i & 1;
        let bi = b >> i & 1;
        let xi = x >> i & 1;
        let mut res = INF;
        if ai ^ bi ^ j != xi {
            *memo[i][j].entry(a).or_default() = res;
            return res;
        }
        if ai == 0 && a >= 1<<i {
            let na = a-(1<<i);
            res = res.min(fnc(i+1,(bi|j)&1,na,b,x,memo) + (1<<i));
        }
        if bi & j == 1 {
            res = res.min(fnc(i+1,1,a,b,x,memo));
        } else if ai == 1 && bi|j == 1 {
            res = res.min(fnc(i+1,1,a,b,x,memo) + (1<<i));
            res = res.min(fnc(i+1,0,a,b,x,memo));
        } else {
            res = res.min(fnc(i+1,0,a,b,x,memo));
        }
        *memo[i][j].entry(a).or_default() = res;
        logln!("dp_{} {:?}", i, memo[i]);
        return res;
    }
}

// https://atcoder.jp/contests/abc172/tasks/abc172_f
fn _solve_dp(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut av = vec![0;n];
    for i in 0..n {
        av[i] = scan.token::<usize>();
    }
    let mut x = if n >2 { av[2] } else { 0 };
    for i in 3..n {
        x ^= av[i];
    }
    let a = av[0];
    let b = av[1];
    let s = a+b;
    logln!("{:b}", a);
    logln!("{:b}", b);
    logln!("{:b}", x);
    logln!("{:b}", s);
    let f = |mut num| {
        let mut cnt = 0;
        while num > 0 {
            num /= 2;
            cnt += 1;
        }
        return cnt;
    };
    let len = f(x).max(f(s));
    const INF:i64 = 1001001001001;
    let mut dp = vec![vec![vec![-INF;2];2];len+1];
    dp[0][0][0] = 0;
    for i in 0..len {
        let xi = x >> i & 1;
        let si = s >> i & 1;
        let ai = a>>i&1;
        for aj in 0..2 { for bj in 0..2 { for j in 0..2 { for k in 0..2 {
            if aj ^ bj != xi || aj ^ bj ^ j != si{
                continue;
            }
            //logln!("{},{},{},{},{},{},{}", i,ai,xi,aj,bj,j,k);
            let nj = if aj+bj+j >= 2 { 1 } else { 0 };
            let add = if aj == 0 { 0 } else { 1<<i };
            if ai > aj || k==0 && ai == aj{
                dp[i+1][nj][0] = dp[i+1][nj][0].max(dp[i][j][k] + add);
            } else {
                dp[i+1][nj][1] = dp[i+1][nj][1].max(dp[i][j][k] + add);
            }
        }}}}
        //logln!("{:?}", dp[i+1]);
    }
    let a2 = dp[len][0][0];
    if a2 <= 0 {
        writeln!(out, "-1").ok();
        return;
    }
    let ans = a - a2 as usize;
    logln!("ans: {:b}",ans);
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
mod abc172f {
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

    test_macro!(test0, b"\
2
100 12
" , "\
44
");

    test_macro!(test1, b"\
2
5 3
" , "\
1
");
    test_macro!(test2, b"\
2
3 5
" , "\
-1
");
    test_macro!(test3, b"\
3
1 1 2
" , "\
-1
");
    test_macro!(test4, b"\
8
10 9 8 7 6 5 4 3
" , "\
3
");
    test_macro!(test5, b"\
3
4294967297 8589934593 12884901890
" , "\
1
");

}
