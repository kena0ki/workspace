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
3 2
0 0 1
" , "\
1
");

    test_macro!(test2, b"\
3 2
1 1 1
" , "\
0
");

    test_macro!(test3, b"\
3 2
0 1 0
" , "\
2
");

    test_macro!(test4, b"\
7 3
0 0 1 2 0 1 0
" , "\
2
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut va = Vec::with_capacity(n);
    let mut cnt = vec![0;n];
    let mut amax = 0;
    for _ in 0..n {
        let a = scan.token::<usize>();
        va.push(a);
        amax = amax.max(a);
    }
    for i in 0..m {
        cnt[va[i]] += 1;
    }
    logln!("{:?}", cnt);
    let mut set = BTreeSet::new();
    for i in 0..m {
        if cnt[i] == 0 {
            set.insert(i);
        }
    }
    set.insert(amax+1);
    logln!("{:?}", set);
    let mut ans = *set.iter().next().unwrap();
    for i in 0..(n-m) {
        cnt[va[i]] -=1;
        cnt[va[i+m]] +=1;
        if cnt[va[i]] == 0 {
            set.insert(va[i]);
        }
        if cnt[va[i+m]] == 1 {
            set.remove(&va[i+m]);
        }
        let &mex = set.iter().next().unwrap();
        ans = ans.min(mex);
    }
    writeln!(out, "{}", ans).ok();
}

