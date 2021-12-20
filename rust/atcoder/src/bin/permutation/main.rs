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

//
//  P(n,k) = P(n-1,k) + k*P(n-1,k-1)
//
//   k |  |  |  |  |
//     |  |  |  |  |
//   4 | 0| 0| 0|24|
//   3 | 0| 0| 6|24|
//   2 | 0| 2| 6|12|
//   1 | 1| 2| 3| 4|
//     +------------
//      1  2  3  4  n
//
//  n! = n*(n-1)!
//
//   k |  |  |  |  |
//     |  |  |  |  |
//   4 | 0| 0| 0| 6|
//   3 | 0| 0| 2| 6|
//   2 | 0| 1| 2| 6|
//   1 | 1| 1| 2| 6|
//     +------------
//      1  2  3  4  n : sum(n) = n!
//
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut dp = vec![vec![0;n+1];n+1];
    dp[0][0]=1;
    for i in 1..=n {
        let mut cum = vec![0;i+1];
        for j in 1..=i {
            cum[j] = cum[j-1]+dp[i-1][j-1];
        }
        logln!("{:?}", cum);
        for j in 1..=i {
            dp[i][j] = cum[i];
        }
    }
    logln!("{:?}", dp);
    let mut ans = 0;
    for j in 0..=n {
        ans +=dp[n][j];
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
mod permutation {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4
";
        let expected = "\
24
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
