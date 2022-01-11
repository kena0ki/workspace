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

// https://atcoder.jp/contests/abc228/tasks/abc228_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<u64>();
    let k = scan.token::<u64>();
    let m = scan.token::<u64>();
    const MOD:u64 = 998244353;
    const MOD2:u64 = 998244352;
    if m % MOD == 0 {
        writeln!(out, "0").ok();
    }
    let k = ModU64::<MOD2>::new(k);
    let kn = k.pow(n);
    let m = ModU64::<MOD>::new(m);
    let ans = m.pow(kn.val());
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
mod abc228e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 2 2
";
        let expected = "\
16
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
3 14 15926535
";
        let expected = "\
109718301
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
