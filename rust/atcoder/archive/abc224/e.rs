// template

use std::{io::{BufRead, BufWriter, Write}, collections::{BinaryHeap, BTreeMap}};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc224/tasks/abc224_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let n = scan.token::<usize>();
    let mut a = BTreeMap::<i64,Vec<(usize,usize)>>::new();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        let r = scan.token::<usize>()-1;
        let c = scan.token::<usize>()-1;
        let ai = scan.token::<i64>()-1;
        a.entry(-ai).or_default().push((r,c));
        v.push((r,c));
    }
    let mut rmax = vec![0;h];
    let mut cmax = vec![0;w];
    let mut maxtbl = vec![vec![0;w];h];
    for ai in &a {
        logln!("{:?}", ai);
    }
    for p in a.values() {
        logln!("{:?}", rmax);
        logln!("{:?}", cmax);
        for &(r,c) in p {
            let now = rmax[r].max(cmax[c]);
            maxtbl[r][c] = now;
        }
        for &(r,c) in p {
            let now = maxtbl[r][c];
            rmax[r] = rmax[r].max(now+1);
            cmax[c] = cmax[c].max(now+1);
        }
    }
    logln!("{:?}", maxtbl);
    for rc in v {
        writeln!(out, "{}", maxtbl[rc.0][rc.1]).ok();
    }
}
fn _solve_ugly(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let n = scan.token::<usize>();
    let mut a = BinaryHeap::<(usize,usize,usize)>::with_capacity(n);
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        let r = scan.token::<usize>()-1;
        let c = scan.token::<usize>()-1;
        let ai = scan.token::<usize>()-1;
        a.push((ai, r, c));
        v.push((r,c));
    }
    let mut maxtbl:Vec<Vec<i64>> = vec![vec![0;w];h];
    let mut rowmax:Vec<(usize,i64)> = vec![(usize::MAX, -1);h];
    let mut colmax:Vec<(usize,i64)> = vec![(usize::MAX, -1);w];
    while let Some(item) = a.pop() {
        let ai = item.0;
        let r = item.1;
        let c = item.2;
        let rm = &mut rowmax[r];
        let cm = &mut colmax[c];
        if rm.0 <= ai && cm.0 <= ai {
            logln!("skipped");
            maxtbl[r][c] = rm.1.max(cm.1);
            continue;
        }
        let mut val = -1;
        if rm.0 > ai {
            val = rm.1.max(val);
        }
        if cm.0 > ai {
            val = cm.1.max(val);
        }
        val += 1;
        *rm = (ai, rm.1.max(val));
        *cm = (ai, cm.1.max(val));
        maxtbl[r][c] = val;
        logln!("{:?}", item);
        logln!("{:?}", rowmax);
        logln!("{:?}", colmax);
    }

    for rc in v {
        writeln!(out, "{}", maxtbl[rc.0][rc.1]).ok();
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
mod abc224e {
    use super::*;

    #[test]
    fn test_ugly() {
        let input: &[u8] = b"\
3 3 8
1 1 4
1 2 7
2 1 3
2 3 5
3 1 2
3 2 5
3 3 5
2 2 5
";
        let expected = "\
1
0
2
0
3
1
0
1
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        _solve_ugly(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test1() {
        let input: &[u8] = b"\
3 3 7
1 1 4
1 2 7
2 1 3
2 3 5
3 1 2
3 2 5
3 3 5
";
        let expected = "\
1
0
2
0
3
1
0
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
5 7 20
2 7 8
2 6 4
4 1 9
1 5 4
2 2 7
5 5 2
1 7 2
4 6 6
1 4 1
2 1 10
5 6 9
5 3 3
3 7 9
3 6 3
4 3 4
3 3 10
4 2 1
3 5 4
1 2 6
4 7 9
";
        let expected = "\
2
4
1
5
3
6
6
2
7
0
0
4
1
5
3
0
5
2
4
0
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
