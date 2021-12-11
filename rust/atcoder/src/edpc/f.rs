// template

use std::{io::{BufRead, BufWriter, Write}, collections::VecDeque};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let a = scan.token::<String>();
    let b = scan.token::<String>();
    let a = a.as_bytes();
    let b = b.as_bytes();
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0;m+1];n+1];
    for i in 0..n {
        for j in 0..m {
            if a[i] == b[j] {
                println!("ai: {}, i: {}, j: {}", a[i], i, j);
                dp[i+1][j+1] = dp[i][j]+1;
            } else {
                dp[i+1][j+1] = dp[i][j+1].max(dp[i+1][j]);
            }
        }
    }
    println!("{:?}", dp);
    let mut i=n;
    let mut j=m;
    let mut que = VecDeque::with_capacity(dp[n][m]);
    while i > 0 && j > 0 {
        if a[i-1] == b[j-1] {
            println!("{}", a[i-1] as char);
            println!("{}", b[j-1] as char);
            que.push_front(a[i-1]);
            i-=1;
            j-=1;
        } else if dp[i-1][j] == dp[i][j] {
            i-=1;
        } else {
            j-=1;
        }
    }
    writeln!(out, "{}", std::str::from_utf8(que.make_contiguous()).unwrap()).ok();
}


#[cfg(test)]
mod edpc_e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
        axyb
        abyxb
        ";
        let expected = "\
        axb
        ";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
        aa
        xayaz
        ";
        let expected = "\
        aa
        ";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
        a
        z
        ";
        let expected = "\n
        ";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
