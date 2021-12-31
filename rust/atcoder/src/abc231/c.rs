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

// https://atcoder.jp/contests/abc232/tasks/abc232_c
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let q = scan.token::<usize>();
    let mut x = Vec::with_capacity(n);
    for i in 0..n {
        let xi = scan.token::<i64>();
        x.push((xi,i+1));
    }
    x.sort_unstable();
    for _ in 0..q {
        let qi = scan.token::<i64>();
        let ans = x.len() - x.binary_search(&(qi,0)).unwrap_err();
        writeln!(out, "{}", ans).ok();
    }
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
mod abc231c {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3 1
100 160 130
120
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
5 5
1 2 3 4 5
6
5
4
3
2
";
        let expected = "\
0
1
2
3
4
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
5 5
804289384 846930887 681692778 714636916 957747794
424238336
719885387
649760493
596516650
189641422
";
        let expected = "\
5
3
5
5
5
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
