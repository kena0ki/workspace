// template

use std::{io::{BufRead, BufWriter, Write}, collections::HashSet};
use rustrithm::{scanner, math::num::fast_gcd};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut p = Vec::<(i64, i64)>::with_capacity(n);
    for _ in 0..n {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        p.push((x,y));
    }
    let mut a = HashSet::<(i64,i64)>::with_capacity(n*n);
    for i in 1..n { for j in 0..i {
        let (xd, yd) = (p[i].0 - p[j].0, p[i].1-p[j].1);
        let g = fast_gcd(xd,yd);
        a.insert((xd/g,yd/g));
        let (xd, yd) = (p[j].0 - p[i].0, p[j].1-p[i].1);
        let g = fast_gcd(xd,yd);
        a.insert((xd/g,yd/g));
    }}
    writeln!(out, "{}", a.len()).ok();
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
mod abc226d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
1 2
3 6
7 4
";
        let expected = "\
5
";
        println!("{}",(1.5 + 0.5) as i32);
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
3
1 2
2 2
4 2
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
    fn test3() {
        let input: &[u8] = b"\
4
0 0
0 1000000000
1000000000 0
1000000000 1000000000
";
        let expected = "\
8
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
