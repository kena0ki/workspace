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

fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut dp = vec![vec![0f64; n+1]; n+1];
    dp[0][0]=1f64;
    for i in 1..=n {
        let p = scan.token::<f64>();
        for j in 0..=n {
            if j>0 {
                dp[i][j] = dp[i-1][j-1]*p + dp[i-1][j]*(1f64 - p);
            } else {
                dp[i][j] = dp[i-1][j]*(1f64 - p);
            }
        }
    }
    println!("{:?}", dp);
    let mut ans=0f64;
    let s = n/2 + 1;
    for j in s..=n {
        ans+=dp[n][j];
    }
    writeln!(out, "{}", ans).ok();
}

#[cfg(test)]
mod edpc_i {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
0.30 0.60 0.80
";
        let expected = "\
0.612
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
1
0.50
";
        let expected = "\
0.5
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
5
0.42 0.01 0.42 0.99 0.42
";
        let expected = "\
0.3821815872
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
