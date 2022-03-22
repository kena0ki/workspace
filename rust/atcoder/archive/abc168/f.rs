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
5 6
1 2 0
0 1 1
0 2 2
-3 4 -1
-2 6 3
1 0 1
0 1 2
2 0 2
-1 -4 5
3 -2 4
1 2 4
" , "\
13
");

    test_macro!(test2, b"\
6 1
-3 -1 -2
-3 -1 1
-2 -1 2
1 4 -2
1 4 -1
1 4 1
3 1 4
" , "\
INF
");

}

// https://atcoder.jp/contests/abc168/tasks/abc168_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let inf = 1<<60;
    let mut vh = Vec::with_capacity(2*n+2*m);
    let mut vv = Vec::with_capacity(2*n+2*m);
    let mut va = Vec::with_capacity(n);
    vh.push(-inf);
    vv.push(-inf);
    vh.push(0);
    vv.push(0);
    for _ in 0..n {
        let mut a = scan.token::<i64>();
        let mut b = scan.token::<i64>();
        let c = scan.token::<i64>();
        if a > b {
            swap(&mut a, &mut b)
        }
        va.push((a,b,c));
        vv.push(a);
        vv.push(b);
        vh.push(c);
    }
    let mut vd = Vec::with_capacity(n);
    for _ in 0..m {
        let d = scan.token::<i64>();
        let mut e = scan.token::<i64>();
        let mut f = scan.token::<i64>();
        if e > f {
            swap(&mut e, &mut f)
        }
        vd.push((d,e,f));
        vv.push(d);
        vh.push(e);
        vh.push(f);
    }
    let gz = |vec: &mut Vec<i64>| {
        let mut set = BTreeSet::new();
        for &v in vec.iter() {
            set.insert(v);
        }
        let mut map = BTreeMap::new();
        let mut vw = vec![0;set.len()];
        let mut i = 0usize;
        let mut pre=-inf;
        for &v in &set {
            map.insert(v,i);
            vw[i] = v-pre;
            i+=1;
            pre=v;
        }
        return (map,vw);
    };
    let (mh,wh) = gz(&mut vh);
    let (mv,wv) = gz(&mut vv);
    let hlen = mh.len();
    let vlen = mv.len();
    let mut judge = vec![vec![0;vlen+2];hlen+2];
    for i in 0..n {
        let (a,b,c) = &va[i];
        judge[mh[c]][mv[a]] += 1;
        judge[mh[c]][mv[b]+1] -= 1;
        judge[mh[c]+1][mv[a]] -= 1;
        judge[mh[c]+1][mv[b]+1] += 1;
    }
    for i in 0..m {
        let (d,e,f) = &vd[i];
        judge[mh[e]][mv[d]] += 1;
        judge[mh[f]+1][mv[d]] -= 1;
        judge[mh[e]][mv[d]+1] -= 1;
        judge[mh[f]+1][mv[d]+1] += 1;
    }
    for i in 0..hlen {
        for j in 0..vlen {
            judge[i][j+1] += judge[i][j];
        }
    }
    for i in 0..hlen {
        for j in 0..vlen {
            judge[i+1][j] += judge[i][j];
        }
    }
    logln!("{:?}",judge);
    let mut que = VecDeque::with_capacity(hlen*vlen);
    let mut vis = vec![vec![false;vlen+2];hlen+2];
    const DI:[i32;4] = [1,0,-1,0];
    const DJ:[i32;4] = [0,-1,0,1];
    que.push_back((mh[&0],mv[&0]));
    while let Some((i,j)) = que.pop_front() {
        if vis[i][j] {
            continue;
        }
        vis[i][j] = true;
        for v in 0..4 {
            let ni = i as i32 + DI[v];
            let nj = j as i32 + DJ[v];
            if ni <= 0 || nj <= 0 || ni > hlen as i32 || nj > vlen as i32 {
                writeln!(out, "INF").ok();
                return;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if v==0 && judge[ni+1][nj] > 0 && judge[ni+1][nj+1] > 0 { continue; }
            if v==1 && judge[ni][nj] > 0 && judge[ni+1][nj] > 0 { continue; }
            if v==2 && judge[ni][nj] > 0 && judge[ni][nj+1] > 0 { continue; }
            if v==3 && judge[ni][nj+1] > 0 && judge[ni+1][nj+1] > 0 { continue; }
            que.push_back((ni,nj));
        }
    }
    logln!("{:?}",vis);
    let mut ans = 0;
    for i in 0..hlen { for j in 0..vlen { if vis[i][j] {
        ans += wh[i+1] * wv[j+1];
    } } }
    writeln!(out, "{}",ans).ok();
}

