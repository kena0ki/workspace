// template

use std::{io::{BufRead, BufWriter, Write}, collections::HashSet};
use rustrithm::{scanner, math::combin::Permutations};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc232/tasks/abc232_c
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut x = vec![vec![false;n];n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        x[u][v] = true;
        x[v][u] = true;
    }
    let mut y = vec![vec![false;n];n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        y[u][v] = true;
        y[v][u] = true;
    }
    let perm = Permutations::new((0..n).collect::<Vec<usize>>());
    let mut ny = y.clone();
    for p in perm {
        if x == ny {
            writeln!(out, "Yes").ok();
            return;
        }
        for i in 0..p.len() {
            for j in 0..p.len() {
                ny[i][j] = y[p[i]][p[j]];
            }
        }
    }
    writeln!(out, "No").ok();
}

// https://atcoder.jp/contests/abc232/tasks/abc232_c
fn _solve_ugly(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut x = HashSet::<(usize,usize)>::with_capacity(m);
    let mut y = HashSet::<(usize,usize)>::with_capacity(m);
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        x.insert((u,v));
    }
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        y.insert((u,v));
    }
    let perm = Permutations::new((0..n).collect::<Vec<usize>>());
    for p in perm {
        let mut ny = HashSet::with_capacity(m);
        for &(u,v) in &y {
            let mut nu = u;
            let mut nv = v;
            for i in 0..p.len() {
                if nu == u && u == i {
                    nu = p[i];
                }
                if nv == v && v == i {
                    nv = p[i];
                }
            }
            ny.insert((nu,nv));
        }
        logln!("{:?}", p);
        logln!("{:?}", ny);
        let mut yes = true;
        for &(xu,xv) in &x {
            let mut found = false;
            for &(yu,yv) in &ny {
                if xu == yu && xv == yv
                    || xv == yu && xu == yv {
                    found = true;
                }
            }
            if !found {
                yes = false;
                break;
            }
        }
        if yes {
            writeln!(out, "Yes").ok();
            return;
        }
    }
    logln!("{:?}", x);
    logln!("{:?}", y);
    writeln!(out, "No").ok();
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
mod abc232c {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4 4
1 2
1 3
1 4
3 4
1 3
1 4
2 3
3 4
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
5 6
1 2
1 3
1 4
3 4
3 5
4 5
1 2
1 3
1 4
1 5
3 5
4 5
";
        let expected = "\
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
8 0
";
        let expected = "\
Yes
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
