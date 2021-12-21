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

// https://atcoder.jp/contests/abc222/tasks/abc222_d
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0usize;n];
    for i in 0..n {
        let ai = scan.token::<usize>();
        a[i]=ai;
    }
    let mut b = vec![0usize;n];
    for i in 0..n {
        let bi = scan.token::<usize>();
        b[i]=bi;
    }
    let m=b[n-1];
    const MOD:u64 = 998244353;
    let mut dp = vec![vec![ModU64::<MOD>::new(0); m+1];n+1];
    dp[0][a[0]]=ModU64::<MOD>::new(1);
    for i in 1..=n {
        if i >=2 {
            let mut sum=ModU64::<MOD>::new(0);
            for j in a[i-2]..a[i-1] {
                sum+=dp[i-1][j];
            }
            dp[i][a[i-1]-1] = sum;
        }
        for j in a[i-1]..=b[i-1] {
                dp[i][j] = dp[i][j-1] + dp[i-1][j];
        }
    }
    logln!("{:?}", dp);
    let mut ans=ModU64::<MOD>::new(0);
    for j in a[n-1]..=m {
        ans+=dp[n][j];
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
mod abc222d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2
1 1
2 3
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
3
2 2 2
2 2 2
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
10
1 2 3 4 5 6 7 8 9 10
1 4 9 16 25 36 49 64 81 100
";
        let expected = "\
978222082
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
