// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, math::num::Rational};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc225/tasks/abc225_f
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut pairs = Vec::<(Rational, Rational)>::with_capacity(n);
    for _ in 0..n {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        let h = Rational::new(y,x-1);
        let l = Rational::new(y-1,x);
        pairs.push((l,h));
    }
    logln!("{:?}", pairs);
    pairs.sort_unstable();
    let mut bar = Rational::from(0);
    let mut ans = 0;
    for &(l,h) in &pairs {
        if l >= bar {
            ans+=1;
            bar = h;
        }
    }
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
mod abc225e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
1 1
2 1
1 2
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
10
414598724 87552841
252911401 309688555
623249116 421714323
605059493 227199170
410455266 373748111
861647548 916369023
527772558 682124751
356101507 249887028
292258775 110762985
850583108 796044319
";
        let expected = "\
10
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
