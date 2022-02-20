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

// https://atcoder.jp/contests/abc223/tasks/abc223_d
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut adj = vec![Vec::<usize>::new(); n];
    let mut deg = vec![0;n];
    for _ in 0..m {
        let u = scan.token::<usize>() -1;
        let v = scan.token::<usize>() -1;
        adj[u].push(v);
        deg[v]+=1;
    }
    let mut que = BinaryHeap::<Reverse<usize>>::with_capacity(n);
    for i in 0..deg.len() {
        if deg[i] == 0 {
            que.push(Reverse(i));
        }
    }
    let mut ans = Vec::with_capacity(n);
    while let Some(u) = que.pop() {
        ans.push(u.0);
        for &v in &adj[u.0] {
            deg[v]-=1;
            if deg[v] == 0 {
                que.push(Reverse(v));
            }
        }
    }
    if ans.len() < n {
        writeln!(out, "-1").ok();
    } else {
        for a in &ans[..n-1] {
            write!(out, "{} ", a+1).ok();
        }
        writeln!(out, "{}", ans[n-1]+1).ok();

    }

}

#[cfg(test)]
mod abc223d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4 3
2 1
3 4
2 4
";
        let expected = "\
2 1 3 4
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
        println!();
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
2 3
1 2
1 2
2 1
";
        let expected = "\
-1
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
        println!();
    }
}
