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

//          1-indexed
// |       |i    1    1    1 |i    2    2    2 |i    3    3    3 |  |
// |       |j    1    2    3 |j    1    2    3 |j    1    2    3 |  |n!
// | 0 0 0 |1 -> 0           |0                |0                |  |
// | 0 0 1 |0 -> 1           |0                |0                |  |1
// | 0 1 0 |0      -> 1      |0                |0                |  |
// | 0 1 1 |0                |0 -> 1 -> 2      |0                |  |2
// | 1 0 0 |0           -> 1 |0                |0                |  |
// | 1 0 1 |0                |0 -> 1      -> 2 |0                |  |
// | 1 1 0 |0                |0      -> 1 -> 2 |0                |  |
// | 1 1 1 |0                |0                |0           -> 6 |  |6
//
// | 0 0 0 |  <=>  |  -   -   -|
// | 0 0 1 |       |  -   - a11|
// | 0 1 0 |       |  - a12   -|
// | 0 1 1 |       |  - a22 a21|
// | 1 0 0 |       |a13   -   -|
// | 1 0 1 |       |a23   - a21|
// | 1 1 0 |       |a23 a22   -|
// | 1 1 1 |       |a33 a32 a31|
//
const MOD:u64 = 1000000007;
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![vec![0usize; n];n];
    for i in 0..n { for j in 0..n {
        let aij = scan.token::<usize>();
        a[i][j]=aij;
    }}
    let mut dp = vec![vec![0;1<<n];n+1];
    dp[0][0]=1;
    // // not optimized
    // for i in 0..n { for msk in 0..1<<n { for j in 0..n {
    //     if a[i][j] == 1 && msk >> j & 1 == 1 {
    //         let msk_unsetj = msk ^ 1<<j; //unset j'th bit
    //         dp[i+1][msk] = (dp[i+1][msk] + dp[i][msk_unsetj]) % MOD;
    //         //logln!("msk {}",msk);
    //     }
    // }}}

    // // optimized
    // let mut dp = vec![vec![0;1<<n];n+1];
    // dp[0][0]=1;
    // for i in 0..n { for msk in 0..1<<n {
    //         if dp[i][msk] == 0 {
    //             continue;
    //         }
    //         for j in 0..n {
    //             if a[i][j] == 1 && ((msk >> j) & 1) == 0 {
    //                 dp[i+1][msk ^ 1<<j] = (dp[i+1][msk ^ 1<<j] + dp[i][msk]) % MOD;
    //             }
    //         }
    //     }
    // }

    // more optimized
    let mut dp = vec![0;1<<n];
    dp[0]=1;
    for msk in 1usize..1<<n {
        let i = msk.count_ones() - 1;
        let i = i as usize;
        for j in 0..n {
            if a[i][j] == 1 && (msk >> j & 1) == 1 {
                let msk_unsetj = msk ^ 1<<j;
                dp[msk] = (dp[msk] + dp[msk_unsetj]) % MOD;
            }
        }
    }
    // logln!("{:?}", dp);
    writeln!(out, "{}", dp[(1<<n)-1]).ok();
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
mod edpc_o {
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
