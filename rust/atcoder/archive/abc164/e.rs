// template

use std::{io::{BufRead, BufWriter, Write}, cmp::Reverse};
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
}

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}
impl<R: ::std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, buffer: vec![] }
    }
    pub fn token<T: ::std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

#[cfg(test)]
mod abc164e {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let output = &mut Vec::new();
                let scan = &mut Scanner::new($input as &[u8]);
                solve(scan, output);
                assert_eq!($expected, std::str::from_utf8(output).unwrap());
            }
        };
    }

    test_macro!(test1, b"\
3 2 1
1 2 1 2
1 3 2 4
1 11
1 2
2 5
" , "\
2
14
");

    test_macro!(test2, b"\
4 4 1
1 2 1 5
1 3 4 4
2 4 2 2
3 4 1 1
3 1
3 1
5 2
6 4
" , "\
5
5
7
");

    test_macro!(test3, b"\
6 5 1
1 2 1 1
1 3 2 1
2 4 5 1
3 5 11 1
1 6 50 1
1 10000
1 3000
1 700
1 100
1 1
100 1
" , "\
1
9003
14606
16510
16576
");

    test_macro!(test4, b"\
4 6 1000000000
1 2 50 1
1 3 50 5
1 4 50 7
2 3 50 2
2 4 50 4
3 4 50 3
10 2
4 4
5 5
7 7
" , "\
1
3
5
");

    test_macro!(test5, b"\
2 1 0
1 2 1 1
1 1000000000
1 1
" , "\
1000000001
");

}

// https://atcoder.jp/contests/abc164/tasks/abc164_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let s = scan.token::<usize>();
    let mut adj = vec![Vec::with_capacity(m);n];
    for _ in 0..m {
        let u = scan.token::<usize>() -1;
        let v = scan.token::<usize>() -1;
        let a = scan.token::<usize>();
        let b = scan.token::<usize>();
        adj[u].push((v,a,b));
        adj[v].push((u,a,b));
    }
    let mut vc = Vec::with_capacity(n);
    for _ in 0..n {
        let c = scan.token::<usize>();
        let d = scan.token::<usize>();
        vc.push((c,d));
    }

    let maxa = 5000;
    let inf = 1<<60;
    let mut que = BinaryHeap::<Reverse<(usize,usize,usize)>>::with_capacity(maxa * n);
    let mut dist = vec![vec![inf;maxa+1];n];
    let push = |v:usize,i:usize,dis:usize,dist:&mut Vec<Vec<_>>, que: &mut BinaryHeap<_>| {
        if dist[v][i] <= dis { return; }
        dist[v][i] = dis;
        for j in (0..i).rev() {
            dist[v][j] = dist[v][j].min(dist[v][j+1]);
        }
        que.push(Reverse((dis,v,i)));
    };
    push(0,s.min(maxa),0,&mut dist, &mut que);
    while let Some(Reverse((dis,u,i))) = que.pop() {
        logln!("{},{},{}", dis,u,i);
        if dist[u][i] < dis { continue; }
        for &(v,a,b) in &adj[u] {
            if i >= a {
                push(v,i-a,dis+b,&mut dist,&mut que);
            }
        }
        let (c,d) = vc[u];
        let t = dis+d;
        let x = i+c;
        push(u,x.min(maxa),t,&mut dist,&mut que);
    }
    for v in 1..n {
        //let ans = dist[v].iter().min().copied().unwrap();
        let ans = dist[v][0];
        writeln!(out, "{}",ans).ok();
    }
}

