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

// https://atcoder.jp/contests/abc214/tasks/abc214_f
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let s = scan.token::<String>();
    let s = s.as_bytes();
    let n = s.len();
    let mut a = vec![0;26];
    let mut dp = vec![vec![0;2];n+1];
    logln!("dp len:{}", dp.len());
    dp[0][0] = 1;
    for i in 1..n+1 {
        dp[i][0] = dp[i-1][1]+dp[i-1][0];
        let ai = (s[i-1] - b'a') as usize;
        dp[i][1] = dp[i-1][0] - a[ai];
        a[ai] = dp[i][1];
    }
    writeln!(out, "{}", dp[n][0]+dp[n][1] -1).ok();
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
mod abc214f {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
abc
";
        let expected = "\
4
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
aa
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
acba
";
        let expected = "\
6
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test4() {
        let input: &[u8] = b"\
chokudai
";
        let expected = "\
54
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
