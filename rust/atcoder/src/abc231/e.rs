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

// https://atcoder.jp/contests/abc231/tasks/abc231_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let x = scan.token::<usize>();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let ai = scan.token::<usize>();
        a.push(ai);
    }

    let mut memo = HashMap::<usize,usize>::new();
    let ans = f(x,0,&a,&mut memo);
    fn f(x:usize, i:usize, a: &Vec<usize>, memo: &mut HashMap<usize,usize>) -> usize{
        if memo.contains_key(&x) {
            return memo[&x];
        }
        if i == a.len()-1{
            return x / a[i];
        }
        let cur = a[i];
        let nxt = a[i+1];
        let rest = x%nxt;
        logln!("{}, {}, {}", x,rest,nxt);
        let pay = f(x-rest,i+1,a,memo)+(rest/cur);
        let ext = nxt - rest;
        let exc = f(x+ext,i+1,a,memo)+(ext/cur);
        let ret = pay.min(exc);
        memo.insert(x,ret);
        return ret;
    }
    writeln!(out, "{}", ans).ok();
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
mod abc231e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3 87
1 10 100
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
2 49
1 7
";
        let expected = "\
7
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
10 123456789012345678
1 100 10000 1000000 100000000 10000000000 1000000000000 100000000000000 10000000000000000 1000000000000000000
";
        let expected = "\
233
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
