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

// https://atcoder.jp/contests/abc224/tasks/abc224_g
//
// E = (n-k/n) * E + sum((i/n)*a: {i=1~k}) + b
//   = (n-k/n) * E + (k(k-1)/2n)*a + b
// k/n * E = k(k-1)/2n * a + b
// E = (k-1)/2 * a + n/k * b
//   = k/2 * a + n/k * b - a/2
//
// AM–GM inequality
// https://en.wikipedia.org/wiki/Inequality_of_arithmetic_and_geometric_means
//
// k/2 * a = n/k * b
// k^2 = 2 * n * b / a
// k = sqrt(2nb/a)
//
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<u64>();
    let s = scan.token::<u64>();
    let t = scan.token::<u64>();
    let a = scan.token::<u64>();
    let b = scan.token::<u64>();
    let f = |k:u64| ((k * a) as f64 /2. + (n*b) as f64 /k as f64 - a as f64/2.);
    let k = f64::sqrt((2*n*b) as f64/a as f64) as u64;
    let mut ans = f64::MAX;
    ans = ans.min(f(1));
    ans = ans.min(f(t));
    for i in k-1..=k+1 {
        if i < 1 || t < i {
            continue;
        }
        ans = ans.min(f(t.min(i)));
    }
    if s <= t {
        ans = ans.min(((t-s) * a) as f64);
    }
    writeln!(out, "{:.9}", ans).ok();
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
mod abc224g {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
5 2 4 10 4
";
        let expected = "\
15.0000000000000000
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(
            format!("{:.9}",expected),
            format!("{:.9}",std::str::from_utf8(output).unwrap())
        );
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
10 6 6 1 2
";
        let expected = "\
0.0000000000000000
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(
            format!("{:0.9}",expected),
            format!("{:0.9}",std::str::from_utf8(output).unwrap())
        );
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
1000000000 1000000000 1 1000000000 1000000000
";
        let expected = "\
1000000000000000000.0000000000000000
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(
            format!("{:.09}",expected),
            format!("{:.09}",std::str::from_utf8(output).unwrap())
        );
    }
}
