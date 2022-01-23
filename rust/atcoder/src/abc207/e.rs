// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, math::modulo::ModU64};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc207/tasks/abc207_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD: u64 = 1000000007;
    let n = scan.token::<usize>();
    let mut a = vec![0;n];
    let mut c = vec![0;n+1];
    for i in 0..n {
        let ai = scan.token::<usize>();
        a[i] = ai;
        c[i+1] = c[i]+ ai;
    }
    let mut dp = vec![vec![vec![None;n];n];n];
    f(&mut dp,&c,0,n-1,0);
    fn f(dp:&mut Vec<Vec<Vec<Option<ModU64<MOD>>>>>, c: &Vec<usize>,
        l:usize, r:usize, i:usize) -> ModU64<MOD> {
        if dp[l][r][i].is_some() {
            return dp[l][r][i].unwrap();
        }
        let mut res = ModU64::<MOD>::new(0);
        let lsum = c[r+1] - c[l];
        if lsum % (i+1) == 0 {
            logln!("l,r,i,c[r],c[l] {} {} {} {} {}", l,r,i,c[r],c[l]);
            res += 1;
        }
        for j in 0..r-l {
            let lsum = c[l+j+1] - c[l];
            if lsum % (i+1) == 0 {
                res += f(dp,c,l+j+1,r,i+1);
            }
        }
        dp[l][r][i] = Some(res);
        return res;
    }
    let mut ans = ModU64::<MOD>::new(0);
    for i in 0..n {
        if dp[0][n-1][i].is_some() {
            ans += dp[0][n-1][i].unwrap();
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
mod abc208e {
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
        let scan = &mut scanner::Scanner::new(input);
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
        let scan = &mut scanner::Scanner::new(input);
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
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
