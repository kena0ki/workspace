// template

use std::{io::{BufRead, BufWriter, Write}, collections::HashMap};
use rustrithm::{scanner, graph::disjoint_set::DisjointSets, math::modulo::ModU64};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc226/tasks/abc226_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut ds = DisjointSets::new(n);
    let mut deg = vec![0;n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        ds.merge(u,v);
        deg[u] += 1;
    }
    let mut numc = HashMap::<usize,usize>::with_capacity(n);
    let mut nume = HashMap::<usize,usize>::with_capacity(n);
    for i in 0..n {
        let rep = ds.find(i);
        *numc.entry(rep).or_default() += 1;
        *nume.entry(rep).or_default() += deg[i];
    }
    for i in numc.keys() {
        if numc[i] != nume[i] {
            writeln!(out, "{}", 0).ok();
            return;
        }
    }
    const MOD:u64=998244353;
    let ans = ModU64::<MOD>::new(2).pow(numc.len() as u64);
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
mod abc226f {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3 3
1 2
1 3
2 3
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
2 1
1 2
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
7 7
1 2
2 3
3 4
4 2
5 6
6 7
7 5
";
        let expected = "\
4
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
