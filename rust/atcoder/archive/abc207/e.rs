// template

use std::io::{BufRead, BufWriter, Write};

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}
impl<R: ::std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, buffer: vec![] }
    }
    pub fn token<T: ::std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
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
mod abc207e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4
1 2 3 4
";
        let expected = "\
3
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
5
8 6 3 3 3
";
        let expected = "\
5
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
10
791754273866483 706434917156797 714489398264550 918142301070506 559125109706263 694445720452148 648739025948445 869006293795825 718343486637033 934236559762733
";
        let expected = "\
15
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}

// https://atcoder.jp/contests/abc207/tasks/abc207_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD: usize = 1000000007;
    let n = scan.token::<usize>();
    let mut a = vec![0;n];
    let mut c = vec![0;n+1];
    for i in 0..n {
        let ai = scan.token::<usize>();
        a[i] = ai;
        c[i+1] = c[i]+ ai;
    }
    let mut dp = vec![vec![0;n+1];n+1];
    let mut dp2 = vec![vec![0;n+1];n+1];
    dp[0][0] = 1;
    dp2[0][0] = 1;
    for i in 0..n+1 {
        //// O(n^3)
        //for j in (0..n).rev() { for k in i+1..n+1 {
        //    if (c[k] - c[i]) % (j+1) == 0{
        //        dp[k][j+1] += dp[i][j];
        //        dp[k][j+1] %= MOD;
        //    }
        //} }
        //// O(n^3)
        //for j in 0..n {
        //    let mut sum = 0;
        //    for k in 0..i {
        //        if (c[i] - c[k]) % (j+1) == 0{
        //            sum += dp[k][j] % MOD;
        //        }
        //    }
        //    dp[i][j+1] += sum;
        //}
        for j in 0..n {
            dp[i][j+1] += dp2[j+1][c[i]%(j+1)] % MOD;
            dp2[j+1][c[i]%(j+1)] += dp[i][j] % MOD;
        }
        logln!("{:?}", dp[i]);
    }
    logln!("{:?}", dp2);
    let ans = dp[n].iter().copied().sum::<usize>();
    let ans = ans%MOD;
    writeln!(out, "{}", ans).ok();
}

