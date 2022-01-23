// template

use std::{io::{BufRead, BufWriter, Write}, collections::HashMap};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc208/tasks/abc208_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let s = scan.token::<String>();
    let s = s.as_bytes().iter()
        .map(|v| ((v - b'0') as usize)).collect::<Vec<_>>();
    let n = s.len();
    let k = scan.token::<usize>();
    let mut dp = HashMap::<usize,usize>::with_capacity(1001001);
    let mut ceil = 1usize;
    logln!("s: {:?}", s);
    for i in 0..n {
        let mut pre = HashMap::<usize,usize>::with_capacity(1001001);
        std::mem::swap(&mut dp, &mut pre);
        for j in 0..10 {
            for (&key,&val) in &pre {
                let k = key * j;
                *dp.entry(k).or_default() += val;
            }
        }
        let limit = if i == 0 { s[i] } else { 10 };
        for j in 1..limit {
            *dp.entry(j as usize).or_default() += 1;
        }
        if i != 0 {
            for j in 0..s[i] {
                *dp.entry(ceil * j).or_default() += 1;
            }
        }
        ceil *= s[i] as usize;
        //logln!("dp: {:?}", dp);
    }
    *dp.entry(ceil).or_default() += 1;

    let mut ans = 0;
    for (&key,&val) in &dp {
        if key <= k {
            ans += val;
        }
    }
    writeln!(out , "{}", ans).ok();
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
mod abc208e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
13 2
";
        let expected = "\
5
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
100 80
";
        let expected = "\
99
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
1000000000000000000 1000000000
";
        let expected = "\
841103275147365677
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
