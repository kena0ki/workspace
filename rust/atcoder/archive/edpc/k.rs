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

fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let ai = scan.token::<usize>();
        a.push(ai);
    }
    let mut dp = vec![0; k+1];
    for i in 1..=k {
        for j in 0..n {
            if i>=a[j] && dp[i-a[j]] == 0 { dp[i] = 1 };
        }
    }

    let ans = if dp[k] == 1 {
        "First"
    } else {
        "Second"
    };
    writeln!(out, "{}", ans).ok();
}

#[cfg(test)]
mod edpc_k {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 4
2 3
";
        let expected = "\
First
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test2() {
        let input: &[u8] = b"\
2 5
2 3
";
        let expected = "\
Second
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test3() {
        let input: &[u8] = b"\
2 7
2 3
";
        let expected = "\
First
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test4() {
        let input: &[u8] = b"\
3 20
1 2 3
";
        let expected = "\
Second
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test5() {
        let input: &[u8] = b"\
3 21
1 2 3
";
        let expected = "\
First
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test6() {
        let input: &[u8] = b"\
1 100000
1
";
        let expected = "\
Second
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
