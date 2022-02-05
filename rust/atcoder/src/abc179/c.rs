// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::scanner;
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc179/tasks/abc179_c
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut ans = 0;
    for a in 1..n {
        ans+= (n-1)/a;
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
mod abc999x {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
1000000
";
        let expected = "\
13969985
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
