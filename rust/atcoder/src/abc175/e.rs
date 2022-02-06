// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::scanner;
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc175/tasks/abc175_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let r = scan.token::<usize>();
    let c = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut v = vec![vec![0;c+1];r+1];
    for _ in 0..k {
        let ri = scan.token::<usize>();
        let ci = scan.token::<usize>();
        let vi = scan.token::<usize>();
        v[ri][ci] = vi;
    }
    let mut dp = vec![vec![vec![0;4];c+1];r+1];
    for ri in 1..r+1 {
        for ci in 1..c+1 {
            let mx = dp[ri-1][ci].iter().max().copied().unwrap();
            dp[ri][ci][0] = dp[ri][ci-1][0].max(mx);
            dp[ri][ci][1] = dp[ri][ci-1][1].max(dp[ri][ci][0]+v[ri][ci]);
            dp[ri][ci][2] = dp[ri][ci-1][2].max(dp[ri][ci-1][1]+v[ri][ci]);
            dp[ri][ci][3] = dp[ri][ci-1][3].max(dp[ri][ci-1][2]+v[ri][ci]);
        }
    }
    let mut ans = 0;
    for i in 0..4 {
        ans = ans.max(dp[r][c][i]);
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
mod abc175d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 2 3
1 1 3
2 1 4
1 2 5
";
        let expected = "\
8
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
2 5 5
1 1 3
2 4 20
1 2 1
1 3 4
1 4 2
";
        let expected = "\
29
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
4 5 10
2 5 12
1 5 12
2 3 15
1 2 20
1 1 28
2 4 26
3 2 27
4 5 21
3 5 10
1 3 10
";
        let expected = "\
142
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
