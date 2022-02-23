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
mod abc164d {
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
50
" , "\
23
");

    test_macro!(test1, b"\
15
" , "\
23
");

    test_macro!(test2, b"\
1
" , "\
1
");

    test_macro!(test3, b"\
13
" , "\
21
");

    test_macro!(test4, b"\
100000
" , "\
3234566667
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let k = scan.token::<usize>();
    let mut vb = vec![0usize;20];
    let mut cnt = 0;
    let mut b = 1;
    let is_ok = |x,y| {
        return (x + 1 <= y + 1) && (x + 1 +1 >= y);
    };
    while cnt < k {
        for i in 0..b+1 {
            if vb[i] < 9 {
                if i==0 && i==b-1 ||
                   i==b ||
                   i==0 && is_ok(vb[i],vb[i+1]) ||
                   i==b-1 && is_ok(vb[i],vb[i-1])  ||
                   i>=1 && is_ok(vb[i],vb[i-1]) && is_ok(vb[i],vb[i+1])
                {
                    vb[i] += 1;
                    for j in (0..i).rev() {
                        vb[j] = vb[j+1].saturating_sub(1);
                    }
                    if i == b {
                        b+=1;
                    }
                    break;
                }
            }
        }
        cnt += 1;
        //logln!("{:?}", vb);
    }
    for i in (0..b).rev() {
        write!(out, "{}", vb[i]).ok();
    }
    writeln!(out, "").ok();
}

