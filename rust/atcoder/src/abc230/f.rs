// template

use std::{io::{BufRead, BufWriter, Write}, collections::HashMap};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc230/tasks/abc230_f
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = Vec::with_capacity(n+1);
    a.push(0);
    for i in 1..=n {
        let ai = scan.token::<i64>();
        a.push(a[i-1]+ai);
    }
    let a = &a[1..n];
    let n = n-1;

    let mut dp = vec![0;n+1];
    let mut p = HashMap::<i64,usize>::with_capacity(n);
    dp[0]=1;
    for i in 0..n{
        let sub = if p.contains_key(&a[i]) { p[&a[i]] } else { 0 };
        dp[i+1] = 2*dp[i] - sub;
        p.insert(a[i],dp[i]);
    }
    logln!("{:?}", dp);
    writeln!(out, "{}", dp[n]).ok();

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
mod abc230f {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
1 -1 1
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
10
377914575 -275478149 0 -444175904 719654053 -254224494 -123690081 377914575 -254224494 -21253655
";
        let expected = "\
321
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
