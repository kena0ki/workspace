// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc232/tasks/abc232_d
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let mut c = vec![vec![true;w];h];
    for i in 0..h {
        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<_>>();
        for j in 0..w {
            c[i][j] = s[j] == '.';
        }
    }
    let mut memo = vec![vec![0;w];h];
    let ans = f(&mut memo, 0,0,h,w, &c);
    fn f(memo: &mut Vec<Vec<usize>>, hi: usize, wj:usize, h:usize, w:usize, c: &Vec<Vec<bool>>) -> usize {
        if memo[hi][wj] > 0 {
            return memo[hi][wj];
        }
        let mut max = 1;
        if hi + 1 < h && c[hi+1][wj] {
            max=max.max(f(memo,hi+1, wj,h,w,c)+1);
        }
        if wj + 1 < w && c[hi][wj+1] {
            max=max.max(f(memo,hi,wj+1,h,w,c)+1);
        }
        memo[hi][wj] = max;
        logln!("{:?}",memo);
        return max;
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
mod abc232d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3 4
.#..
..#.
..##
";
        let expected = "\
4
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
1 1
.
";
        let expected = "\
1
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
5 5
.....
.....
.....
.....
.....
";
        let expected = "\
9
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
