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

const DX:&[i64] = &[1,0,-1,0];
const DY:&[i64] = &[0,1,0,-1];

// https://atcoder.jp/contests/abc170/tasks/abc170_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let k = scan.token::<usize>();
    let x1 = scan.token::<usize>()-1;
    let y1 = scan.token::<usize>()-1;
    let x2 = scan.token::<usize>()-1;
    let y2 = scan.token::<usize>()-1;
    let mut grid = Vec::with_capacity(h);
    for _ in 0..h {
        let b = scan.token_bytes();
        grid.push(b);
    }
    let mut que = BinaryHeap::with_capacity(h*w);
    let mut dist = HashMap::with_capacity(h*w);
    let push = |i,j,d,l,v,dist:&mut HashMap<_,_>,que:&mut BinaryHeap<_>| {
        if let Some(&(pd,pl,pv)) = dist.get(&(i,j)) {
            if pd < d || pd == d && pl <= l && pv == v {
                return;
            }
        }
        dist.insert((i,j),(d,l,v));
        que.push(Reverse((d,l,i,j,v)));
    };
    push(x1,y1,0,0,5,&mut dist, &mut que);
    while let Some(Reverse((d,l,i,j,pv))) = que.pop() {
        if dist.contains_key(&(i,j))  && dist[&(i,j)].0 < d {
            continue;
        }
        for v in 0..4 {
            let ni = i as i64 + DX[v];
            let nj = j as i64 + DY[v];
            if ni < 0|| nj < 0 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if ni >= h ||nj >= w || grid[ni][nj] == b'@' {
                continue;
            }
            let nl;
            let mut nd = d;
            if l == k || v != pv{
                nd = d+1;
                nl = 1;
            } else {
                nl = l+1;
            };
            logln!("{},{}",nl,l);
            push(ni,nj,nd,nl,v,&mut dist, &mut que);
        }
    }
    logln!("{:?}", dist);
    if let Some(d) = dist.get(&(x2,y2)) {
        writeln!(out, "{}",d.0).ok();
    } else{
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
mod abc170f {
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
1 2 1
1 1 1 2
..
" , "\
2
");

    test_macro!(test1, b"\
3 5 2
3 2 3 4
.....
.@..@
..@..
" , "\
5
");

    test_macro!(test2, b"\
1 6 4
1 1 1 6
......
" , "\
2
");

    test_macro!(test3, b"\
3 3 1
2 1 2 3
.@.
.@.
.@.
" , "\
-1
");

}
