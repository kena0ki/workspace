// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

use rustrithm::math::{combin::Factorial, modulo::{MOD1000000007, ZERO_MOD1000000007, ModU64}};

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
1 3
" , "\
2
1
1
");

    test_macro!(test2, b"\
2
1 2
" , "\
1
1
");

    test_macro!(test3, b"\
5
1 2
2 3
3 4
3 5
" , "\
2
8
12
3
3
");

    test_macro!(test4, b"\
8
1 2
2 3
3 4
3 5
3 6
6 7
6 8
" , "\
40
280
840
120
120
504
72
72
");

}

pub trait EntitySpec<T>: Copy {
    fn identity() -> Self;
    fn add(&self, rhs: Self, v:usize, adj: &Vec<Vec<usize>>, t: &T) -> Self;
    fn sub(&self, rhs: Self, v:usize, adj: &Vec<Vec<usize>>, t: &T) -> Self;
    fn add_root(&self, v:usize, adj: &Vec<Vec<usize>>, t: &T) -> Self;
}

pub struct Rerooting<T, E:EntitySpec<T>> {
    pub t:T,
    pub dp:Vec<E>,
    pub adj:Vec<Vec<usize>>,
}

impl <T, E:EntitySpec<T>> Rerooting<T, E> {
    pub fn new(n: usize, t:T) -> Self {
        Self {
            t,
            dp:vec![E::identity();n],
            adj:vec![Vec::with_capacity(n-1);n],
        }
    }
    pub fn add_edge(&mut self, u:usize, v:usize) {
        self.adj[u].push(v);
    }
    pub fn rerooting(&mut self) {
        Self::dfs1(0,usize::max_value(), &self.adj, &mut self.dp, &self.t);
        Self::dfs2(0,usize::max_value(), &self.adj, &mut self.dp, &self.t);
    }
    fn dfs1(u:usize, p:usize, adj: &Vec<Vec<usize>>, dp:&mut Vec<E>, t: &T) {
        let mut val = E::identity();
        for &v in &adj[u] {
            if p == v { continue; }
            Self::dfs1(v,u,adj,dp,t);
            let dp_v = dp[v].add_root(v,adj, t);
            val = val.add(dp_v, v, adj, t);
        }
        dp[u] = val;
    }
    fn dfs2(u:usize, p:usize, adj: &Vec<Vec<usize>>, dp:&mut Vec<E>, t:&T) {
        for &v in &adj[u] {
            if p == v { continue; }
            let dp_v = dp[v].add_root(v,adj,t);
            let dp_u = dp[u].sub(dp_v,v,adj,t);
            let dp_u = dp_u.add_root(v,adj,t);
            dp[v] = dp[v].add(dp_u,v,adj,t);
            Self::dfs2(v,u,adj,dp,t);
        }
    }
}

type FactM = Factorial<MOD1000000007>;
#[derive(Clone,Copy)]
pub struct Entity {
    val:ModU64<MOD1000000007>,
    size:u64,
}
impl EntitySpec<FactM> for Entity {
    fn identity() -> Self {
        return Self { val:ZERO_MOD1000000007+1, size:0 };
    }
    fn add(&self, rhs: Self, _:usize, _adj: &Vec<Vec<usize>>, t:&FactM) -> Self {
        let newsize = self.size+rhs.size;
        let mut newval = self.val;
        newval *= rhs.val;
        newval *= t.combin(newsize, rhs.size);
        return Self { val:newval, size: newsize };
    }
    fn sub(&self, rhs: Self, _:usize, _adj: &Vec<Vec<usize>>, t:&FactM) -> Self {
        let mut newval = self.val;
        newval /= rhs.val;
        newval /= t.combin(self.size, rhs.size);
        logln!("{},{}", self.size, rhs.size);
        let newsize = self.size-rhs.size;
        return Self { val:newval, size: newsize };
    }
    fn add_root(&self, _v:usize, _adj: &Vec<Vec<usize>>, _:&FactM) -> Self {
        return Self { val: self.val, size: self.size+1 };
    }
}

// https://atcoder.jp/contests/abc160/tasks/abc160_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let fact = Factorial::<MOD1000000007>::new(2001001);
    let mut r = Rerooting::<_,Entity>::new(n,fact);
    for _ in 0..n-1 {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        r.add_edge(u,v);
        r.add_edge(v,u);
    }
    r.rerooting();
    for i in 0..n {
        writeln!(out, "{}", r.dp[i].val).ok();
    }
}

