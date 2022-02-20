// template

use std::{io::{BufRead, BufWriter, Write}, collections::BinaryHeap, cmp::Reverse};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc204/tasks/abc204_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut a: Vec<Vec::<(usize,usize,usize)>> = vec![vec![];n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        let c = scan.token::<usize>();
        let d = scan.token::<usize>();
        a[u].push((v,c,d));
    }

    const INF:usize = 1001001001;
    let f = |t:usize,c:usize,d:usize| {
        let mut l = 0;
        let mut r = INF;
        while l+1 < r {
            let x = (l+r) /2;
            logln!("{},{}",l,r);
            if x + c+(d/(x+1)) < t + c+(d/(t+1)) {
                l = x;
            } else {
                r = x;
            }
        }
        return l;
    };

    let mut que = BinaryHeap::<Reverse<(usize,usize)>>::with_capacity(m);
    que.push(Reverse((0,0)));
    let mut dist = vec![INF;n];
    dist[0] = 0;
    let mut vis = vec![false;n];
    while let Some(Reverse((du, u))) = que.pop() {
        vis[u] = true;
        if dist[u] < du {
            continue;
        }
        for &(v,c,d) in &a[u] {
            if vis[v] {
                continue;
            }
            let t = f(du,c,d);
            let dv = t + c+(d/(t+1));
            if dv < dist[v] {
                dist[v] = dv;
                que.push(Reverse((dv,v)));
            }
        }
    }
    logln!("{:?}",dist);

    if dist[n-1] == INF {
        writeln!(out, "{}" , -1).ok();
    } else {
        writeln!(out, "{}" , dist[n-1]).ok();
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
mod abc204e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 1
1 2 2 3
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
2 3
1 2 2 3
1 2 2 1
1 1 1 1
";
        let expected = "\
3
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test3() {
        let input: &[u8] = b"\
4 2
1 2 3 4
3 4 5 6
";
        let expected = "\
-1
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test4() {
        let input: &[u8] = b"\
6 9
1 1 0 0
1 3 1 2
1 5 2 3
5 2 16 5
2 6 1 10
3 4 3 4
3 5 3 10
5 6 1 100
4 2 0 110
";
        let expected = "\
20
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
