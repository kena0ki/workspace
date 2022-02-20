// template

use std::{io::{BufRead, BufWriter, Write}, collections::{HashMap, VecDeque}};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc224/tasks/abc224_d
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n=9;
    let m = scan.token::<usize>();
    let mut adj = vec![Vec::<usize>::with_capacity(m);n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut s = vec![None;n];
    for i in 0..n-1 {
        let p = scan.token::<usize>()-1;
        s[p] = Some(i);
    }
    let mut t = vec![None;n];
    for i in 0..n-1 {
        t[i] = Some(i);
    }
    let mut dist = HashMap::<Vec<Option<usize>>,usize>::new();
    let mut que = VecDeque::<Vec<Option<usize>>>::new();
    dist.insert(s.clone(),0);
    que.push_back(s);
    while let Some(prev) = que.pop_front() {
        let u = prev.iter().position(|item| item.is_none()).unwrap();
        for &v in adj[u].iter() {
            let mut new = prev.clone();
            new.swap(u,v);
            if dist.contains_key(&new) {
                continue;
            }
            let cnt = dist[&prev] + 1;
            dist.insert(new.clone(),cnt);
            que.push_back(new);
        }
    }

    if let Some(d) = dist.get(&t) {
        writeln!(out, "{}", d).ok();
    } else {
        writeln!(out, "-1").ok();
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
mod abc224d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
5
1 2
1 3
1 9
2 9
3 9
3 9 2 4 5 6 7 8
";
        let expected = "\
5
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
5
1 2
1 3
1 9
2 9
3 9
1 2 3 4 5 6 7 8
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
12
8 5
9 6
4 5
4 1
2 5
8 9
2 1
3 6
8 7
6 5
7 4
2 3
1 2 3 4 5 6 8 7
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
12
6 5
5 4
4 1
4 7
8 5
2 1
2 5
6 9
3 6
9 8
8 7
3 2
2 3 4 6 1 9 7 8
";
        let expected = "\
16
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
