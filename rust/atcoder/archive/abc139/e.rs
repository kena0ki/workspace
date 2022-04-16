// template

use std::{io::{BufRead, BufWriter, Write}, mem::swap};
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
2 3
1 3
1 2
" , "\
3
");

    test_macro!(test2, b"\
4
2 3 4
1 3 4
4 1 2
3 1 2
" , "\
4
");

    test_macro!(test3, b"\
3
2 3
3 1
1 2
" , "\
-1
");

}

// https://atcoder.jp/contests/abc139/tasks/abc139_e
fn _solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vva = vec![VecDeque::with_capacity(n);n];
    for i in 0..n { for _ in 0..n-1 {
        let a = scan.token::<usize>()-1;
        vva[i].push_back(a);
    }}
    let mut cnt = 0;
    let max = n*(n-1)/2;
    let mut vd = vec![0;n];
    loop {
        let precnt = cnt;
        for i in 0..n {
            if let Some(&a) = vva[i].front() {
                if let Some(&b) = vva[a].front() {
                    if b == i {
                        vva[i].pop_front();
                        vva[a].pop_front();
                        let d = vd[i].max(vd[a]) + 1;
                        vd[i] = d;
                        vd[a] = d;
                        cnt+=1;
                        logln!("{},{},{}",cnt,a,b);
                    }
                }
            }
        }
        if precnt == cnt {
            writeln!(out, "-1").ok();
            return;
        }
        if cnt >= max {
            break;
        }
    }
    let ans = vd.iter().max().copied().unwrap();
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc139/tasks/abc139_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vva = vec![Vec::with_capacity(n);n];
    for i in 0..n { for _ in 0..n-1 {
        let a = scan.token::<usize>()-1;
        vva[i].push(a);
    }}
    let nv = n*(n-1)/2;
    let mut mp = HashMap::with_capacity(nv);
    let mut adj = vec![Vec::new();nv];
    let mut deg = vec![0;nv];
    let mut cnt=0;
    for i in 0..n { for j in 0..n-1 {
        let mut a = i;
        let mut b = vva[i][j];
        if a > b {
            swap(&mut a, &mut b);
        }
        let v;
        if mp.contains_key(&(a,b)) {
            v = mp[&(a,b)];
        } else {
            v=cnt;
            mp.insert((a,b),v);
            cnt+=1;
        }
        if j>=1 {
            let mut pa = i;
            let mut pb = vva[i][j-1];
            if pa > pb {
                swap(&mut pa, &mut pb);
            }
            //logln!("{:?}",mp);
            //logln!("{},{},{},{}", i,j,pa,pb);
            let u = mp[&(pa,pb)];
            adj[u].push(v);
            deg[v] += 1;
        }
    } }
    let mut topo = Vec::with_capacity(n);
    let mut que = VecDeque::with_capacity(nv);
    for i in 0..nv {
        if deg[i]==0 {
            que.push_back(i);
        }
    }
    while let Some(u) = que.pop_front() {
        topo.push(u);
        for &v in &adj[u] {
            deg[v] -= 1;
            if deg[v] == 0 {
                que.push_back(v);
            }
        }
    }
    if topo.len() < nv {
        writeln!(out, "-1").ok();
        return;
    }
    let mut dp = vec![0;nv];
    for i in 0..nv {
        let u = topo[i];
        for &v in &adj[u] {
            dp[v] = dp[v].max(dp[u]+1);
        }
    }
    let ans = dp.iter().max().copied().unwrap();
    writeln!(out, "{}", ans+1).ok();
}

