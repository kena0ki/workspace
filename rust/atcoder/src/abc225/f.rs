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

//
// not a < b
// but ab > ba
//  = a*10^|b| + b > b*10^|a| + a
//  = a*(1/(10^|a| - 1)) > b*(1/(10^|b| - 1))
// e.g.
// not 2 < 21
// but 221 > 212
//  = 2 * 1/9 > 21 * 1/99
//  = 22/99 > 21/99
//
// https://atcoder.jp/contests/abc225/tasks/abc225_f
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut s = Vec::<String>::with_capacity(n);
    for _ in 0..n {
        let si = scan.token::<String>();
        s.push(si);
    }
    s.sort_unstable_by(|a,b| {
        (String::from(a) + b.as_str()).cmp(&(String::from(b)+a.as_str()))
        //(String::from(b) + a.as_str()).cmp(&(String::from(a)+b.as_str()))
    });
    let mut dp = vec![String::with_capacity(51*51);k+1];
    for i in 0..n {
        let end = (i+1).min(k);
        for j in (1..=end).rev() {
            // let mut new = s[i].clone();
            // new.push_str(dp[j-1].as_str());
            let mut new = dp[j-1].clone();
            new.push_str(s[i].as_str());
            if dp[j] == "" || new < dp[j]  {
                dp[j] = new;
            }
        }
    }
    writeln!(out, "{}", dp[k]).ok();
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
mod abc225f {
    use super::*;

    #[test]
    fn test0() {
        let input: &[u8] = b"\
5 2
c
baa
ba
bab
b
";
        let expected = "\
baab
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test1() {
        let input: &[u8] = b"\
4 3
ode
zaaa
r
atc
";
        let expected = "\
atcoder
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test2() {
        let input: &[u8] = b"\
5 2
z
z
zzz
z
zzzzzz
";
        let expected = "\
zz
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
