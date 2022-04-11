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

    test_macro!(test1, b"\
3
1 2
2 3
" , "\
2
1
2
");

    test_macro!(test2, b"\
8
1 2
2 3
2 4
2 5
4 7
5 6
6 8
" , "\
4
1
2
3
4
1
1
2
");

    test_macro!(test3, b"\
6
1 2
1 3
1 4
1 5
1 6
" , "\
5
1
2
3
4
5
");

}

// https://atcoder.jp/contests/abc146/tasks/abc146_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut adj = vec![Vec::with_capacity(n);n];
    let mut deg = vec![0;n];
    for i in 0..n-1 {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push((v,i));
        adj[v].push((u,i));
        deg[u] += 1;
        deg[v] += 1;
    }
    let maxd = deg.iter().max().copied().unwrap();
    let mut ans = vec![0;n-1];
    dfs(0,&adj,usize::max_value(),0,maxd,&mut ans);
    fn dfs(u:usize, adj: &Vec<Vec<(usize,usize)>>, p:usize, mut c:usize,maxd:usize, ans: &mut Vec<usize>) {
        for &(v,i) in &adj[u] {
            logln!("v,i:{},{}",v,i);
            if v == p { continue; }
            c=(c+1)%maxd;
            ans[i] = c;
            dfs(v,adj,u,c,maxd,ans);
        }
    }
    logln!("{:?}",ans);
    writeln!(out, "{}", maxd).ok();
    for i in 0..n-1 {
        writeln!(out, "{}", ans[i]+1).ok();
    }
}

