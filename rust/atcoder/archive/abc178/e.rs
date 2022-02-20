// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, range_query::{StaticArq, specs::AssignMin}};
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc178/tasks/abc178_d
// WIP
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = BTreeMap::<i64,Vec<i64>>::new();
    let mut b = BTreeMap::<i64,Vec<i64>>::new();
    for _ in 0..n {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        let d = a.entry(x).or_default();
        d.push(y);
        let d = b.entry(x).or_default();
        d.push(y);
    }
    let seg = StaticArq::<AssignMin>::new();
    let mut ans = 0;
    for m in [a,b] {
        let mut mn = 1001001001;
        let mut mx = 0;
        for (x,v) in &m {
            for &y in v {
                mx = mx.max(x+y);
            }
            for &y in v {
                mn = mn.min(y+x);
                ans = ans.max((mx-mn).abs());
            }
        }
    }
    writeln!(out,"{}",ans).ok();
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
mod abc178d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4
10 1
2 4
3 2
1 10
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
2
1 1
1 1
";
        let expected = "\
0
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

}
