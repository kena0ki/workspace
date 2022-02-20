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

// https://atcoder.jp/contests/abc202/tasks/abc202_d
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let a = scan.token::<usize>();
    let b = scan.token::<usize>();
    let n = a+b;
    let k = scan.token::<usize>();
    let mut dp = vec![vec![vec![0;b+2];a+2];n+1];
    dp[0][0][0] = 1;
    for i in 0..(a+b) { for j in 0..a+1 { for k in 0..b+1 {
        if i == j+k {
            dp[i+1][j+1][k] = dp[i+1][j+1][k] + dp[i][j][k];
            dp[i+1][j][k+1] = dp[i+1][j][k+1] + dp[i][j][k];
        }
    } } }
    //logln!("{:?}", dp);
    let mut ai = a;
    let mut bi = b;
    let mut ans = String::new();
    let mut kk=k;
    let mut ni =n-1;
    loop {
        if ai > 0 && kk <= dp[ni][ai-1][bi] {
            //logln!("a {},{},{}", ni, ai, bi);
            ans = ans + "a";
            if ni == 0 { break; }
            ai -=1;
        } else {
            //logln!("b {},{},{},{}", ni, ai, bi, kk);
            ans = ans + "b";
            if ni == 0 { break; }
            if ai>0 { kk -= dp[ni][ai-1][bi]; }
            bi -=1;
        }
        ni-=1;
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
mod abc204e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 2 4
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
    fn test2() {
        let input: &[u8] = b"\
30 30 118264581564861424
";
        let expected = "\
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
