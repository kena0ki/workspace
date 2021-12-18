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

//       |i    0    0    0 |i    1    1    1 |i    2    2    2 |
//       |j    0    1    2 |j    0    1    2 |j    0    1    2 |
// 0 0 0 |1                |1                |1                |
// 0 0 1 |  -> 1           |  -> 2           |  -> 3           |
// 0 1 0 |       -> 1      |       -> 2      |       -> 3      |
// 0 1 1 |                 |  -> 1 -> 3      |  -> 5 -> 8      |
// 1 0 0 |            -> 1 |            -> 2 |            -> 3 |
// 1 0 1 |                 |  -> 1      -> 3 |  -> 5      -> 8 |
// 1 1 0 |                 |       -> 1 -> 3 |       -> 5 -> 8 |
// 1 1 1 |                 |       -> 1 -> 4 |       -> 9 ->17 |
const MOD:u64 = 10000007;
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![vec![0usize; n];n];
    for i in 0..n { for j in 0..n {
        let aij = scan.token::<usize>();
        a[i][j]=aij;
    }}
    let mut dp = vec![vec![ModU64::<MOD>::new(0);1<<n];n+1];
    dp[0][0]=ModU64::<MOD>::new(1);
    for i in 0..n { for msk in 0..1<<n { for j in 0..n {
        if a[i][j] == 1 && msk >> j & 1 == 1 {
            let msk_unsetj = msk ^ 1<<j; //unset j'th bit
            dp[i+1][msk] = dp[i][msk] + dp[i][msk_unsetj];
            logln!("msk {}",msk);
        } else {
            dp[i+1][msk] = dp[i][msk];
        }
    }}}
    logln!("{:?}", dp);
    writeln!(out, "{}", dp[n][(1<<n)-1]).ok();
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
mod abc999x {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
0 1 1
1 0 1
1 1 1
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
4
0 1 0 0
0 0 0 1
1 0 0 0
0 0 1 0
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
1
0
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
    fn test4() {
        let input: &[u8] = b"\
21
0 0 0 0 0 0 0 1 1 0 1 1 1 1 0 0 0 1 0 0 1
1 1 1 0 0 1 0 0 0 1 0 0 0 0 1 1 1 0 1 1 0
0 0 1 1 1 1 0 1 1 0 0 1 0 0 1 1 0 0 0 1 1
0 1 1 0 1 1 0 1 0 1 0 0 1 0 0 0 0 0 1 1 0
1 1 0 0 1 0 1 0 0 1 1 1 1 0 0 0 0 0 0 0 0
0 1 1 0 1 1 1 0 1 1 1 0 0 0 1 1 1 1 0 0 1
0 1 0 0 0 1 0 1 0 0 0 1 1 1 0 0 1 1 0 1 0
0 0 0 0 1 1 0 0 1 1 0 0 0 0 0 1 1 1 1 1 1
0 0 1 0 0 1 0 0 1 0 1 1 0 0 1 0 1 0 1 1 1
0 0 0 0 1 1 0 0 1 1 1 0 0 0 0 1 1 0 0 0 1
0 1 1 0 1 1 0 0 1 1 0 0 0 1 1 1 1 0 1 1 0
0 0 1 0 0 1 1 1 1 0 1 1 0 1 1 1 0 0 0 0 1
0 1 1 0 0 1 1 1 1 0 0 0 1 0 1 1 0 1 0 1 1
1 1 1 1 1 0 0 0 0 1 0 0 1 1 0 1 1 1 0 0 1
0 0 0 1 1 0 1 1 1 1 0 0 0 0 0 0 1 1 1 1 1
1 0 1 1 0 1 0 1 0 0 1 0 0 1 1 0 1 0 1 1 0
0 0 1 1 0 0 1 1 0 0 1 1 0 0 1 1 1 1 0 0 1
0 0 0 1 0 0 1 1 0 1 0 1 0 1 1 0 0 1 1 0 1
0 0 0 0 1 1 1 0 1 0 1 1 1 0 1 1 0 0 1 1 0
1 1 0 1 1 0 0 1 1 0 1 1 0 1 1 1 1 1 0 1 0
1 0 0 1 1 0 1 1 1 1 1 0 1 0 1 1 0 0 0 0 0
";
        let expected = "\
102515160
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
