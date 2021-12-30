// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, graph::disjoint_set::DisjointSets};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc232/tasks/abc232_d
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut dsu = DisjointSets::new(n);
    let mut deg = vec![0;n];
    for _ in 0..m {
        let v = scan.token::<usize>()-1;
        let u = scan.token::<usize>()-1;
        if ! dsu.merge(u,v) {
            writeln!(out, "No").ok();
            return;
        }
        deg[u]+=1;
        deg[v]+=1;
    }
    for d in deg {
        if d >= 3 {
            writeln!(out, "No").ok();
            return;
        }
    }
    writeln!(out, "Yes").ok();
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
4 2
1 3
2 3
";
        let expected = "\
Yes
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
4 3
1 4
2 4
3 4
";
        let expected = "\
No
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
