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
5 5
.....
.###.
..##.
.###.
.....
" , "\
4
");

    test_macro!(test2, b"\
7 6
......
.###..
..###.
.####.
.####.
.#.##.
......
" , "\
4
");

    test_macro!(test3, b"\
3 3
...
.#.
...
" , "\
4
");

}

// https://atcoder.jp/contests/abc191/tasks/abc191_c
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let mut vvs = Vec::with_capacity(h);
    for _ in 0..h {
        let vs = scan.token_bytes();
        vvs.push(vs);
    }
    let mut si = 0;
    let mut sj = 0;
    'outer: for i in 0..h { for j in 0..w {
        if vvs[i][j] == b'#' {
            si=i; sj=j;
            break 'outer;
        }
    } }
    let mut i=si;
    let mut j=sj;
    let di = [0,1,0,-1];
    let dj = [1,0,-1,0];
    let ei = [-1,0,1,0];
    let ej = [0,1,0,-1];
    let mut v=0;
    let mut ans = 0;
    //let mut cnt = 0;
    loop {
        logln!("{},{}", i,j);
        let ci=(i as i32+ei[v]) as usize;
        let cj=(j as i32+ej[v]) as usize;
        let ni=(i as i32+di[v]) as usize;
        let nj=(j as i32+dj[v]) as usize;
        logln!("{}", vvs[ni][nj] as char);
        if vvs[ci][cj]!=b'.' {
            v=(v+3)%4;
            i=(i as i32+di[v]) as usize;
            j=(j as i32+dj[v]) as usize;
            ans+=1;
        } else if vvs[ni][nj]!=b'#' {
            v=(v+1)%4;
            ans+=1;
        } else {
            i=(i as i32+di[v]) as usize;
            j=(j as i32+dj[v]) as usize;
        }
        if i == si && j == sj && v==0{
            break;
        }
        //if cnt>30 { break; }
        //cnt+=1;
    }
    writeln!(out, "{}", ans).ok();
}

