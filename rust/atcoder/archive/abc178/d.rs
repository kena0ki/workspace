// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, math::modulo::ModU64};
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc178/tasks/abc178_d
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    const MOD:u64 = 1000000007;
    const ZERO:ModU64<MOD> = ModU64::<MOD>::new(0);
    let mut dp =vec![vec![ZERO;n+1];n+1];
    dp[0][0]=ZERO+1;
    let mut ans = ZERO;
    for i in 0..n {
        for j in 0..n+1-3 {
            dp[i+1][j+3] = dp[i][j];
        }
        for j in 0..n {
            dp[i+1][j+1] = dp[i+1][j+1] + dp[i+1][j];
        }
        ans += dp[i+1][n];
    }
    writeln!(out,"{}", ans).ok();
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
mod abc178d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
7
";
        let expected = "\
3
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
2
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
1729
";
        let expected = "\
294867501
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
