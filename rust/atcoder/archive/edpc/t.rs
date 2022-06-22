// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

const MOD:i64 = 1000000007;

fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let s = scan.token::<String>();
    let s = s.as_bytes();
    let mut dp = vec![vec![0;n+1];n+1];
    for j in 0..n+1 {
        dp[1][j]=1;
    }
    for i in 1..n {
        logln!("{:?}", dp);
        let mut cum:Vec<i64> = vec![0;n+1];
        for j in 0..n-i+1 {
            cum[j+1] = (cum[j] + dp[i][j])%MOD;
        }
        if s[i-1] == b'<' {
            for j in 0..n-i { dp[i+1][j] = (cum[n-i+1] - cum[j+1] + MOD)%MOD; }
        } else {
            for j in 0..n-i { dp[i+1][j] = cum[j+1]; }
        }
    }
    logln!("{:?}", dp);
    writeln!(out, "{}", dp[n][0]).ok();
}

// https://atcoder.jp/contests/dp/submissions/3941494
fn _solve_another(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let s = scan.token::<String>();
    let s = s.as_bytes();
    let mut dp = vec![0;1];
    dp[0]=1;
    let mut pre = '<' as u8;
    for i in 0..n-1 {
        logln!("{:?}", dp);
        let mut p = vec![0;dp.len()+1];
        std::mem::swap(&mut dp, &mut p);
        if s[i] != pre {
            p.reverse();
        }
        logln!("p: {:?}", p);
        pre = s[i];
        let mut x=0;
        for j in 0..p.len() {
            x = (x+p[j]) % MOD;
            dp[j+1] = x;
        }
    }
    logln!("{:?}", dp);
    let mut ans = 0;
    for j in 0..n {
        ans += dp[j];
    }
    writeln!(out, "{}", ans ).ok();
}

fn _solve_ugly(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let s = scan.token::<String>();
    let s = s.as_bytes();
    let mut dp = vec![vec![0;n];n];
    dp[0][0]=1;
    let mut pre = b'<';
    for i in 1..n {
        let mut cum = vec![0;i+1];
        for j in 1..=i {
            if s[i-1] == pre {
                cum[j] = (cum[j-1]+dp[i-1][j-1]) % MOD;
            } else {
                cum[j] = (cum[j-1]+dp[i-1][i-j]) % MOD;
            }
        }
        pre = s[i-1];
        logln!("{:?}", cum);
        for j in 1..=i {
            dp[i][j] = cum[j];
        }
    }
    logln!("{:?}", dp);
    let mut ans = 0;
    for j in 0..n {
        ans = (ans + dp[n-1][j]) % MOD;
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

#[cfg(test)]
mod edpc_t {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4
<><
";
        let expected = "\
5
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
5
<<<<
";
        let expected = "\
1
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
20
>>>><>>><>><>>><<>>
";
        let expected = "\
217136290
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
