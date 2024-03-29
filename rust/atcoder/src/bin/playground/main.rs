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

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let a = scan.token::<usize>();
    writeln!(out, "{} 3", a).ok();
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
1 2
";
        let expected = "\
1 3
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test_pg1() {
        let s = "1x2";
        let s = s.split("x").collect::<Vec<_>>();
        let l = s[0].parse::<i32>().unwrap();
        let r = s[1].parse::<i32>().unwrap();
        println!("{},{}", l, r);
        unsafe {
            let sb = b"abcdefg";
            let mut ss = Vec::from(&sb[1..4]);
            ss.reverse();
            let s = &[&sb[..1],&ss, &sb[4..]].concat();
            let s = std::str::from_utf8_unchecked(s);
            println!("{}", s);
            assert!(false);
        }
    }
}
