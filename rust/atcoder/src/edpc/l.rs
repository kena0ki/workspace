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

// The order that elements are visited
// 4, -, -
// 5, 2, -
// 6, 3, 1
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let ai = scan.token::<i64>();
        a.push(ai);
    }
    let mut dp = vec![vec![None;n];n];
    let ans = f(0,n-1,&mut dp, &a);
    fn f(l:usize, r:usize, dp: &mut Vec<Vec<Option<i64>>>, a: &Vec<i64>) -> i64 {
        if dp[l][r].is_none() {
            if l==r {
                dp[l][r] = Some(a[l]);
            } else {
                let left = a[l] - f(l+1, r, dp, a);
                let right = a[r] - f(l, r-1, dp, a);
                dp[l][r]=Some(left.max(right));
            }
        }
        return dp[l][r].unwrap();
    }
    writeln!(out, "{}", ans).ok();
}

#[cfg(test)]
mod edpc_l {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4
10 80 90 30
";
        let expected = "\
10
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
10 100 10
";
        let expected = "\
-80
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
10
";
        let expected = "\
10
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test4() {
        let input: &[u8] = b"\
10
1000000000 1 1000000000 1 1000000000 1 1000000000 1 1000000000 1
";
        let expected = "\
4999999995
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test5() {
        let input: &[u8] = b"\
6
4 2 9 7 1 5
";
        let expected = "\
2
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}