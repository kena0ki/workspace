// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

pub fn pow(val:usize, mut power: usize, modulus:usize) -> usize {
    let mut square = val;
    let mut ret = 1;
    while 0 < power {
        if (power & 1) == 1{
            ret *= square;
            ret %= modulus;
        }
        square *= square;
        square %= modulus;
        power >>= 1;
    }
    return ret;
}
pub fn inv(val: usize, modulus:usize) -> usize {
    return pow(val, modulus - 2, modulus);
}


/// About usage, see mod test block below.
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

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out); }

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

const MOD:usize = 1000000007;

#[derive(Clone,Copy)]
struct Entity {
    val: usize,
    size: usize,
}

impl EntitySpec<(Vec<usize>,Vec<usize>)> for Entity {
    fn add_root(&self, _v:usize, _adj: &Vec<Vec<usize>>, _t: &(Vec<usize>,Vec<usize>)) -> Self {
        return Self { val: self.val, size: self.size + 1 };
    }
    fn identity() -> Self {
        return Self { val:1, size:0 };
    }
    fn add(&self, rhs: Self, _v:usize, _adj: &Vec<Vec<usize>>, t: &(Vec<usize>,Vec<usize>)) -> Self {
        let (fct,ifct) = t;
        let size = self.size + rhs.size;
        let val = self.val * rhs.val;
        let val = val % MOD;
        let c = fct[size]*ifct[size-rhs.size]%MOD*ifct[rhs.size]%MOD;
        let val = val * c;
        let val = val % MOD;
        return Self { val, size };
    }
    fn sub(&self, rhs: Self, _v:usize, _adj: &Vec<Vec<usize>>, t: &(Vec<usize>,Vec<usize>)) -> Self {
        let (fct,ifct) = t;
        let val = self.val * inv(rhs.val, MOD);
        let val = val % MOD;
        let c = ifct[self.size]*fct[self.size-rhs.size]%MOD*fct[rhs.size]%MOD;
        let val = val * c;
        let val = val % MOD;
        let size = self.size - rhs.size;
        return Self { val, size };
    }
}

// https://atcoder.jp/contests/abc163/tasks/abc163_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut fct = vec![0usize;n+1];
    fct[0] = 1;
    for i in 0..n {
        fct[i+1] = fct[i] * (i+1) % MOD;
    }
    let mut ifct = vec![0;n+1];
    ifct[n] = inv(fct[n],MOD);
    for i in (1..n+1).rev() {
        ifct[i-1] = ifct[i] * i % MOD;
    }

    let mut re = Rerooting::<_, Entity>::new(n,(fct,ifct));
    for _ in 0..n-1 {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        re.add_edge(u,v);
        re.add_edge(v,u);
    }
    re.rerooting();
    for i in 0..n {
        writeln!(out, "{}", re.dp[i].val).ok();
    }
}

