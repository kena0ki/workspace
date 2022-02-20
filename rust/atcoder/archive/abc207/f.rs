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

// https://atcoder.jp/contests/abc207/tasks/abc207_f
// WA
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD: u64 = 1000000007;
    let n = scan.token::<usize>();
    let mut a = vec![Vec::<usize>::with_capacity(n);n];
    for _ in 0..n {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        a[u].push(v);
    }

    const ZERO: ModU64::<MOD> = ModU64::<MOD>::new(0);
    let dp = f(&a, 0, None, n);
    fn f(a: &Vec<Vec<usize>>, u: usize, p:Option<usize>,n:usize) -> Vec<Vec<ModU64::<MOD>>> {
        if p.is_some() && a[u].len() == 0 {
            let mut dp = vec![vec![ZERO; 3]; n];
            dp[2][2] = dp[2][2] + 1;
            return dp;
        }
        let mut res;
        let mut dp = Vec::<Vec<ModU64::<MOD>>>::new();
        for &v in &a[u] {
            if p.is_some() && v == p.unwrap() {
                continue;
            }
            res = f(a,v,Some(u),n);
            for i in 0..n {
                if dp.len() > 0 {
                    for j in 0..n-i {
                        dp[i+j][0] += res[j][0];
                        dp[i+j][0] += res[j][1];
                        dp[i+j][0] += res[j][2];
                        dp[i+j][1] += res[j][0];
                        dp[i+j][2] += res[j][0];
                    }
                }
                dp[i][0] += res[i][0];
            }
        }
        return dp;
    }
    let mut ans = ZERO;
    for i in 0..n {
        for j in 0..3 {
            ans = ans + dp[i][j];
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
3
1 3
1 2
";
        let expected = "\
1
0
2
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
5
1 3
4 5
1 5
2 3
";
        let expected = "\
1
0
2
5
7
17
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
6 10
1 8
2 7
5 6
3 8
3 4
7 10
4 9
2 8
";
        let expected = "\
1
0
3
8
15
32
68
110
196
266
325
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
