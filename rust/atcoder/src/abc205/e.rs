// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, math::modulo::ModU64};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc205/tasks/abc205_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD: u64 = 1000000007;
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let k = scan.token::<usize>();
    const ZERO:ModU64::<MOD> = ModU64::new(0);
    let mut dp = vec![vec![vec![ZERO;m+1];n+1];n+m+1];
    dp[0][0][0] = ZERO + 1;
    for i in 1..n+m+1 {
        for ni in 0..n+1 { for mi in 0..m+1 {
            if ni+1 < n+1 && ni+1<= k + mi {
                logln!("{},{}", i,ni);
                dp[i][ni+1][mi] = dp[i][ni+1][mi]+dp[i-1][ni][mi];
            }
            if mi+1 < m+1 {
                dp[i][ni][mi+1] = dp[i][ni][mi+1]+dp[i-1][ni][mi];
            }
        }}
    }
    logln!("{:?}", dp);
    let mut ans = ZERO;
    for ni in 0..n+1 {
        for mi in 0..m+1 {
            ans += dp[n+m][ni][mi];
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

#[cfg(test)]
mod abc208e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 3 1
";
        let expected = "\
9
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
1 0 0
";
        let expected = "\
0
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
1000000 1000000 1000000
";
        let expected = "\
192151600
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

}
