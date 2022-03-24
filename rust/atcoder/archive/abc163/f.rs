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
1 2 1
1 2
2 3
" , "\
5
4
0
");

    test_macro!(test2, b"\
1
1
" , "\
1
");

    test_macro!(test3, b"\
2
1 2
1 2
" , "\
2
2
");

    test_macro!(test4, b"\
5
1 2 3 4 5
1 2
2 3
3 4
3 5
" , "\
5
8
10
5
5
");

    test_macro!(test5, b"\
8
2 7 2 5 4 1 7 5
3 1
1 2
2 7
4 5
5 6
6 8
7 8
" , "\
18
15
0
14
23
0
23
0
");

}

// https://atcoder.jp/contests/abc163/tasks/abc163_f
// WA
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vc = Vec::with_capacity(n);
    for _ in 0..n {
        let c = scan.token::<usize>()-1;
        vc.push(c);
    }
    let mut adj = vec![Vec::with_capacity(n-1);n];
    for _ in 0..n-1 {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut dp = vec![(0,0);n];
    dfs(0,usize::max_value(),&adj, &vc, &mut dp, n);
    fn dfs(u:usize, p:usize, adj: &Vec<Vec<usize>>, vc: &Vec<usize>, dp: &mut Vec<(usize,usize)>, n:usize) -> (usize,HashMap<usize,usize>) {
        let c = vc[u];
        if adj[u].len() == 1 && p != usize::max_value() {
            dp[c] = (n,1);
            let mut mp = HashMap::new();
            mp.insert(c,1);
            return (1,mp);
        }
        let mut chs = 0;
        let mut mp = HashMap::new();
        mp.insert(c,0);
        let mut sum = 0;
        for &v in &adj[u] {
            if p == v { continue; }
            let (ch, mut mpc) = dfs(v,u,adj,vc,dp,n);
            sum += (ch-(*mpc.entry(c).or_default()))*(chs-mp[&c]);
            if mp.len() < mpc.len() {
                std::mem::swap(&mut mp, &mut mpc);
            }
            for (&key,&val) in &mpc {
                *mp.entry(key).or_default() += val;
            }
            chs += ch;
        }
        let (tot,par) = dp[c];
        let cpar = par - mp[&c];
        let cchs = chs - mp[&c];
        sum += cchs * (n-chs-1-cpar) + cchs + (n-chs-cpar);
        dp[c] = (tot+sum, par+chs+1);
        mp.insert(c,chs+1);
        logln!("{:?}",dp);
        return (mp[&c],mp);
    }
    for i in 0..n {
        writeln!(out, "{}", dp[i].0).ok();
    }
}

