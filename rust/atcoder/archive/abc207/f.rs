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
1 3
1 2
" , "\
1
0
2
5
");

    test_macro!(test2, b"\
5
1 3
4 5
1 5
2 3
" , "\
1
0
2
5
7
17
");

    test_macro!(test3, b"\
10
6 10
1 8
2 7
5 6
3 8
3 4
7 10
4 9
2 8
" , "\
1
0
3
8
15
32
68
110
196
266
325
");

}

const MOD:usize = 1000000007;

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut va = vec![Vec::new();n];
    for _ in 0..n-1 {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        va[u].push(v);
        va[v].push(u);
    }
    let mp = dfs(&va, 0, usize::max_value());
    for i in 0..n+1 {
        let mut ans = 0;
        let v = mp.get(&i);
        if v.is_none() {
            writeln!(out, "{}", ans).ok();
            continue;
        }
        let v = v.unwrap();
        for i in 0..3 {
            ans += v[i];
            ans %= MOD;
        }
        writeln!(out, "{}", ans).ok();
    }

    fn dfs(va: &Vec<Vec<usize>>, u:usize, p:usize) -> HashMap<usize,Vec<usize>> {
        let mut res = HashMap::new();
        res.insert(0,vec![1,0,0]);
        res.insert(1,vec![0,0,1]);
        for &v in &va[u] {
            if v == p { continue; }
            let chl = dfs(va,v,u);
            let pre = res.clone();
            let mut nxt = HashMap::new();
            for (&pk,pv) in &pre { for (&ck,cv) in &chl {
                for pi in 0..3 { for ci in 0..3 {
                    if pv[pi]*cv[ci] == 0 { continue; }
                    let mut ni = pi;
                    if ni == 0 && ci == 2 {
                        ni = 1;
                    }
                    let mut add = 0;
                    if (pi == 2 && ci == 0 ) || (ci == 2 && pi == 0 ) {
                        add=1;
                    }
                    let nv = nxt.entry(pk+ck+add).or_insert(vec![0,0,0]);
                    nv[ni] += pv[pi]*cv[ci];
                    nv[ni] %= MOD;
                }}
            }}
            res = nxt;
            logln!("{},{:?}",u,res);
        }
        return res;
    }
}

