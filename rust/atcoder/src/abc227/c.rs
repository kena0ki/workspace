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

// https://atcoder.jp/contests/abc227/tasks/abc227_c
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<u64>();
    let mut ans = 0u64;
    let mut i = 1;
    loop {
        if i*i*i > n {
            break;
        }
        let mut j = i;
        loop {
            if i*j*j > n {
                break;
            }
            let max = n/(i*j);
            ans += max-j+1;
            j+=1;
        }
        i+=1;
    }
    writeln!(out, "{}", ans).ok();
}

fn _solve_tle(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<u64>();
    let mut i = n;
    let mut cnt = 0;
    loop {
        let mut tmpcnt=0;
        let mut j=1;
        loop {
            if i*j > n {
                let prev = i;
                i=n/j;
                cnt += tmpcnt*(prev - i);
                logln!("tmpcnt: {}", tmpcnt);
                logln!("cnt: {}", cnt);
                break;
            } else if i < j {
                cnt +=tmpcnt;
                i-=1;
                break;
            }
            for k in 1..=j {
                if i*j*k <= n {
                    tmpcnt +=1;
                }
            }
            j+=1;
        }
        if i == 0 {
            break;
        }
    }
    writeln!(out, "{}", cnt).ok();
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
mod abc227c {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4
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
100
";
        let expected = "\
323
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
100000000000
";
        let expected = "\
5745290566750
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
