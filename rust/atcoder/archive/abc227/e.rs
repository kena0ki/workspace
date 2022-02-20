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

// https://atcoder.jp/contests/abc227/tasks/abc227_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let s = scan.token::<String>();
    let s = s.as_bytes();
    let k = scan.token::<isize>();
    let mut memo = HashMap::with_capacity(100000);
    let ans = f(&mut memo, &s.to_vec(), k);

    fn f(memo: &mut HashMap<(Vec<u8>,isize),u64>, s: &Vec<u8>, k:isize) -> u64{
        if s.len() == 0 {
            return 1;
        }
        if k < 0 {
            return 0;
        }
        let key = (s.clone(),k);
        if memo.contains_key(&key) {
            return memo[&key];
        }
        let mut res = 0;
        for &c in b"KEY".iter() {
            for i in 0..s.len() {
                if s[i] == c {
                    logln!("s: {:?}", &s[i+1..]);
                    let clone = [&s[..i],&s[i+1..]].concat();
                    res += f(memo, &clone, k-i as isize);
                    break;
                }
            }
        }
        memo.insert(key, res);
        return res;
    }
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc227/tasks/abc227_e
fn _solve_ugly(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let s = scan.token::<String>();
    let s = s.as_bytes().iter().map(|&b| {
        for i in 0..3 {
            if b"KEY"[i] == b {
                return i;
            }
        }
        unreachable!();
    }).collect::<Vec<_>>();
    let n = s.len();
    let k = scan.token::<usize>();

    let mut p = vec![Vec::<usize>::with_capacity(30);3];
    for i in 0..n {
        p[s[i]].push(i);
    }
    logln!("{:?}", p);
    let isz = p[0].len();
    let jsz = p[1].len();
    let ksz = p[2].len();
    let m = k.min(450);
    let mut dp = vec![vec![vec![vec![0;m+1];ksz+1];jsz+1];isz+1];
    dp[0][0][0][0] = 1;
    for i in 0..=isz { for j in 0..=jsz { for k in 0..=ksz {
        if i < isz {
            let mut x = 0;
            for jj in 0..j {
                if p[0][i] < p[1][jj] {
                    x+=1;
                }
            }
            for kk in 0..k {
                if p[0][i] < p[2][kk] {
                    x+=1;
                }
            }
            //logln!("{},{}", m,x);
            for l in 0..(m+1).checked_sub(x).unwrap_or(0) {
                dp[i+1][j][k][l+x] += dp[i][j][k][l];
            }
        }
        if j < jsz {
            let mut x = 0;
            for ii in 0..i {
                if p[1][j] < p[0][ii] {
                    x+=1;
                }
            }
            for kk in 0..k {
                if p[1][j] < p[2][kk] {
                    x+=1;
                }
            }
            for l in 0..(m+1).checked_sub(x).unwrap_or(0) {
                dp[i][j+1][k][l+x] += dp[i][j][k][l];
            }
        }
        if k < ksz {
            let mut x = 0;
            for ii in 0..i {
                if p[2][k] < p[0][ii] {
                    x+=1;
                }
            }
            for jj in 0..j {
                if p[2][k] < p[1][jj] {
                    x+=1;
                }
            }
            logln!("{},{}",m,x);
            for l in 0..(m+1).checked_sub(x).unwrap_or(0) {
                dp[i][j][k+1][l+x] += dp[i][j][k][l];
            }
        }
    } } }
    logln!("{:?}", dp);
    let mut ans = 0;
    for l in 0..=m {
        ans += dp[isz][jsz][ksz][l];
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
mod abc227e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
KEY
1
";
        let expected = "\
3
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
KKEE
2
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
KKEEYY
1000000000
";
        let expected = "\
90
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
