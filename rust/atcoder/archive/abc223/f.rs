// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, range_query::{StaticArq, ArqSpec}};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

struct ArqImpl;
const BIG:i64 = i64::MAX>>20;
impl ArqSpec for ArqImpl {
    type S=(i64,i64);
    type F=(i64,i64);
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        logln!("{:?}, {:?}", a, b);
        return (a.0+b.0,a.1.min(a.0+b.1));
    }
    fn identity() -> Self::S {
        return (0,BIG);
    }
    fn apply(f: &Self::F, _a: &Self::S, _size: i64) -> Self::S {
        return *f;
    }
    fn compose(f: &Self::F, _g: &Self::F) -> Self::F {
        return *f;
    }
}

// https://atcoder.jp/contests/abc223/tasks/abc223_f
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let q = scan.token::<usize>();
    let s = scan.token::<String>();
    let s = s.as_bytes();
    let mut sa = StaticArq::<ArqImpl>::new(&vec![(0,BIG);n][..]);
    for i in 0..n {
        if s[i] == b'(' {
            sa.update(i,i,&(1,1));
        } else {
            sa.update(i,i,&(-1,-1));
        }
    }
    for _ in 0..q {
        let o = scan.token::<usize>();
        let l = scan.token::<usize>() -1;
        let r = scan.token::<usize>() -1;
        if o == 1 {
            let pre_l = sa.query(l,l);
            let pre_r = sa.query(r,r);
            sa.update(l,l,&pre_r);
            sa.update(r,r,&pre_l);
        } else if o == 2 {
            logln!("l,r {},{}", l,r);
            logln!("{:?}", sa.show());
            let x = sa.query(l,r);
            if x.0 == 0 && x.1 == 0 {
                writeln!(out, "Yes").ok();
            } else {
                writeln!(out, "No").ok();
            }
        }
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
mod abc999x {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
5 3
(())(
2 1 4
2 1 2
2 4 5
";
        let expected = "\
Yes
No
No
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
8 8
(()(()))
2 2 7
2 2 8
1 2 5
2 3 4
1 3 4
1 3 5
1 1 4
1 6 8
";
        let expected = "\
Yes
No
No
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
5 3
(())(
2 1 4
1 1 4
2 1 4
";
        let expected = "\
Yes
No
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
