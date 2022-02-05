// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, math::modulo::ModU64};
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc179/tasks/abc179_d
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = Vec::with_capacity(k);
    const MOD:u64 = 998244353;
    const ZERO:ModU64<MOD> = ModU64::<MOD>::new(0);
    for _ in 0..k {
        let l = scan.token::<usize>();
        let r = scan.token::<usize>();
        a.push((l,r));
    }
    a.sort_unstable();
    let mut dp = vec![ZERO;n+1];
    let mut dps = vec![ZERO;n+2];
    dp[n] = ZERO+1;
    dps[n] = ZERO+1;
    for i in (1..=n).rev(){
        for j in 0..k {
            let l = a[j].0;
            if n-i < l { break; }
            let r = (n-i).min(a[j].1);
            if l > r { continue; }
            logln!("{} {}", i+l, i+r+1);
            dp[i] += dps[i+l] - dps[i+r+1];
        }
        dps[i] = dps[i+1] + dp[i];
    }
    logln!("{:?}", dp);
    logln!("{:?}", dps);
    writeln!(out, "{}", dp[1]).ok();
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
mod abc179d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
5 2
1 1
3 4
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
5 2
3 3
5 5
";
        let expected = "\
0
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
5 1
1 2
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
    fn test4() {
        let input: &[u8] = b"\
60 3
5 8
1 3
10 15
";
        let expected = "\
221823067
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
