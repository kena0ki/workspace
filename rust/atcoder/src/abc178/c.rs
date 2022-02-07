// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, math::{modulo::ModU64, combin::Factorial}};
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc178/tasks/abc178_c
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    const MOD:u64 = 1000000007;
    const ZERO:ModU64<MOD> = ModU64::<MOD>::new(0);
    let ten = ZERO+10;
    let eight = ZERO+8;
    let fc = Factorial::new(n);
    let mut sub = ZERO;
    for i in 1..n {
        sub += eight.pow(i as u64)*fc.kcombin(n-i);
    }
    let n = n as u64;
    let ans = ten.pow(n) - sub*2 - eight.pow(n) - 2;
    writeln!(out, "{}",ans).ok();
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
mod abc178c {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2
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
1
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
    fn test3() {
        let input: &[u8] = b"\
869121
";
        let expected = "\
2511445
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
