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

//
// dp[0][0] = 1
// dp[1][j] = 0
// dp[i][j] = sum (dp[i-1][k]) where k is from max(0,j-a[i]) to j
//                                   i is from 1 to N
// N=3, M=4, a[i] = (0,3,4,2)
//   a[i]  3   4   2
//   i 0   1   2   3
// j  -----------------
// 0 | 1 | 1 | 1 | 1 |
// 1 | 0 | 1 | 2 | 3 |
// 2 | 0 | 1 | 3 | 6 |
// 3 | 0 | 1 | 4 | 9 |
// 4 | 0 | 0 | 4 |11 |
//    -----------------
//
// N=3, M=4, a[i] = (0,3,2,4)
//   a[i]  3   2   4
//   i 0   1   2   3
// j  -----------------
// 0 | 1 | 1 | 1 | 1 |
// 1 | 0 | 1 | 2 | 3 |
// 2 | 0 | 1 | 3 | 6 |
// 3 | 0 | 1 | 3 | 9 |
// 4 | 0 | 0 | 2 |11 |
//    -----------------
//
// https://emtubasa.hateblo.jp/entry/2018/08/29/161456
// https://kyopro-friends.hatenablog.com/entry/2019/01/12/231035
const MOD:u64 = 1000000007;
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut dp = vec![vec![ModU64::<MOD>::new(0);m+1];n+1];
    dp[0][0]=ModU64::<MOD>::new(1);
    for i in 1..=n {
        let ai = scan.token::<usize>();
        let mut rsum = vec![ModU64::<MOD>::new(0); m+1+1];
        for j in 1..=m+1 {
            rsum[j] = rsum[j-1] + dp[i-1][j-1];
        }
        for j in 0..=m {
            let max = j.checked_sub(ai).unwrap_or(0);
            dp[i][j]=rsum[j+1]-rsum[max];
        }
    }
    writeln!(out, "{}", dp[n][m]).ok();
}

fn _solve_not_optimized(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut dp = vec![vec![ModU64::<MOD>::new(0);m+1];n+1];
    dp[0][0]=ModU64::<MOD>::new(1);
    for i in 1..=n {
        let ai = scan.token::<usize>();
        for j in 0..=m {
            let max = j.checked_sub(ai).unwrap_or(0);
            let mut sum=ModU64::<MOD>::new(0);
            for k in max..=j {
                sum += dp[i-1][k];
            }
            dp[i][j]=sum;
        }
    }
    writeln!(out, "{}", dp[n][m]).ok();
}
#[cfg(test)]
mod edpc_m {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3 4
1 2 3
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
1 10
9
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
2 0
0 0
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
    fn test4() {
        let input: &[u8] = b"\
4 100000
100000 100000 100000 100000
";
        let expected = "\
665683269
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test5() {
        let input: &[u8] = b"\
3 4
3 2 4
";
        let expected = "\
11
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
