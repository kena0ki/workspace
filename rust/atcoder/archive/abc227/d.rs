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

// https://atcoder.jp/contests/abc227/tasks/abc227_d
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = Vec::<usize>::with_capacity(n);
    for _ in 0..n {
        let ai = scan.token::<usize>();
        a.push(ai);
    }
    a.sort_unstable();

    let mut i=0;
    let mut ans = 0;
    loop {
        let ai = a[i];
        ans+=ai;
        let mut next = k;
        for j in (0..k).rev(){
            a[i+j] -= ai;
            if a[i+j] > 0 {
                next=j;
            }
        }
        i+=next;
        logln!("{:?}", next);
        if i+k > n{
            break;
        }
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
mod abc227d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3 3
2 3 4
";
        let expected = "\
2
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
4 2
1 1 3 4
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
    fn test3() {
        let input: &[u8] = b"\
4 3
1 1 3 4
";
        let expected = "\
2
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
