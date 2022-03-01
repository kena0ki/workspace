// template

use std::io::{BufRead, BufWriter, Write};
use std::collections::*;

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

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc203/tasks/abc203_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut mpp = BTreeMap::<usize,BTreeSet<usize>>::new();
    for _ in 0..m {
        let x = scan.token::<usize>();
        let y = scan.token::<usize>();
        let p = mpp.entry(x).or_default();
        p.insert(y);
    }
    let mut now = BTreeSet::new();
    now.insert(n);
    for (_,ys) in mpp.iter() {
        logln!("ys:{:?}", ys);
        let mut add = vec![];
        for y in ys {
            logln!("{}", y.wrapping_sub(1));
            if now.contains(&y.wrapping_sub(1)) || now.contains(&(y+1)){
                add.push(*y);
            }
        }
        for y in ys {
            now.remove(y);
        }
        for y in add {
            now.insert(y);
        }
        logln!("{:?}", now);
    }
    writeln!(out, "{}a", now.len()).ok();
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
}

#[cfg(test)]
mod abc203e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 4
1 1
1 2
2 0
4 2
";
        let expected = "\
3
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test2() {
        let input: &[u8] = b"\
1 1
1 1
";
        let expected = "\
0
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
