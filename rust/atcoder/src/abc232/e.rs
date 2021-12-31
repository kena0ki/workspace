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

// https://atcoder.jp/contests/abc232/tasks/abc232_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let k = scan.token::<usize>();
    let x1 = scan.token::<usize>();
    let y1 = scan.token::<usize>();
    let x2 = scan.token::<usize>();
    let y2 = scan.token::<usize>();
    const MOD:u64 = 998244353;
    let zero = ModU64::<MOD>::new(0);
    let mut dp = vec![vec![vec![zero;2];2];k+1];
    dp[0][(x1==x2) as usize][(y1==y2) as usize] = zero+1;
    for ki in 1..k+1 {
        for i in 0..2 {
            dp[ki][i][0] = dp[ki][i][0] + dp[ki-1][i][0]*(h-2) as u64;
            dp[ki][i][0] = dp[ki][i][0] + dp[ki-1][i][1]*(h-1) as u64;
            dp[ki][i][1] = dp[ki][i][1] + dp[ki-1][i][0]*1;
            //dp[ki][i][1] += dp[ki-1][i][1]*0;
        }
        for j in 0..2 {
            dp[ki][0][j] = dp[ki][0][j] + dp[ki-1][0][j]*(w-2) as u64;
            dp[ki][0][j] = dp[ki][0][j] + dp[ki-1][1][j]*(w-1) as u64;
            dp[ki][1][j] = dp[ki][1][j] + dp[ki-1][0][j]*1;
            //dp[ki][1][j] += dp[ki-1][1][j]*0;
        }
    }
    writeln!(out, "{}", dp[k][1][1]).ok();
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
mod abc232e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 2 2
1 2 2 1
";
        let expected = "\
2
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
1000000000 1000000000 1000000
1000000000 1000000000 1000000000 1000000000
";
        let expected = "\
24922282
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
3 3 3
1 3 3 3
";
        let expected = "\
9
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
