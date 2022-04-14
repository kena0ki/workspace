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
12 18
" , "\
3
");

    test_macro!(test2, b"\
420 660
" , "\
4
");

    test_macro!(test3, b"\
1 2019
" , "\
1
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut a = scan.token::<usize>();
    let mut b = scan.token::<usize>();
    if a < b {
        swap(&mut a, &mut b)
    }
    let mut seta = BTreeSet::new();
    let mut i = 1;
    while i*i <= a {
        if a%i == 0 {
            seta.insert(i);
            seta.insert(a/i);
        }
        i+=1;
    }
    let mut setb = HashSet::new();
    let mut i = 1;
    while i*i <= a && i<=b {
        if b%i == 0 {
            setb.insert(i);
            setb.insert(b/i);
        }
        i+=1;
    }
    let mut vd = Vec::new();
    for &sb in &setb {
        if seta.contains(&sb) {
            vd.push(sb);
        }
    }
    //let mut mx = HashMap::<usize,usize>::new();
    //for &d in &vd {
    //    let mut j = d;
    //    while j*j <= a {
    //        *mx.entry(j).or_default() += 1;
    //        j+=d;
    //    }
    //}
    vd.sort_unstable();
    logln!("{:?}",vd);
    let mut vcnt = vec![0;vd.len()];
    for i in 0..vd.len() {
        if vd[i] == 1 { continue; }
        for j in i+1..vd.len() {
            if vd[j]%vd[i] == 0 {
                vcnt[j]+=1;
            }
        }
    }
    logln!("{:?}",vcnt);
    let mut ans = 0;
    for i in 0..vd.len() {
        if vcnt[i] == 0 {
            ans +=1;
        }
    }
    writeln!(out, "{}", ans).ok();
}

