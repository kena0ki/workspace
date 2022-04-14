// template

use std::io::{BufRead, BufWriter, Write};
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
mod abc999x {
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

    test_macro!(test0, b"\
4 0
" , "\
-1
");

    test_macro!(test1, b"\
4 5
1 2
2 3
2 4
4 1
4 3
" , "\
3
1
2
4
");

    test_macro!(test2, b"\
4 5
1 2
2 3
2 4
1 4
4 3
" , "\
-1
");

    test_macro!(test3, b"\
6 9
1 2
2 3
3 4
4 5
5 6
5 1
5 2
6 1
6 2
" , "\
4
2
3
4
5
");

}

// https://atcoder.jp/contests/abc142/tasks/abc142_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut adj = vec![Vec::with_capacity(m);n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push(v);
    }
    let mut ans = vec![0;n+1];
    for i in 0..n {
        bfs(&adj,i,&mut ans,n);
        logln!("len:{}",ans.len());
    }
    if ans.len() == n+1 {
        writeln!(out, "{}", -1).ok();
    } else {
        writeln!(out, "{}", ans.len()).ok();
        for i in 0..ans.len() {
            writeln!(out, "{}", ans[i]+1).ok();
        }
    }
    fn bfs(adj: &Vec<Vec<usize>>, i: usize, ans: &mut Vec<usize>, n:usize) {
        let mut pre = vec![None;n];
        let mut f = || {
            let mut que = VecDeque::new();
            que.push_front(i);
            while let Some(u) = que.pop_front() {
                for &v in &adj[u] {
                    if pre[v].is_some() {
                        continue;
                    }
                    pre[v] = Some(u);
                    if v == i {
                        return true;
                    }
                    que.push_back(v);
                }
            }
            return false;
        };
        if ! f() { return }
        logln!("pre:{:?}",pre);
        let mut u = i;
        let mut tmp = Vec::new();
        while let Some(v) = pre[u] {
            tmp.push(v);
            u = v;
            if v == i { break; }
        }
        if tmp.len() < ans.len() {
            *ans = tmp;
        }
    }
}

