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
// ai: a1 a2 a3 a4  a1 a2 a3 a4
//             |         |
//          |               |
//       |            |
//
// ai: a1 a2 a3 a4  a1 a2 a3 a4
//             1         3
//          2               4
//       5
//
// ai: a1 a2 a3 a4
//       |     |
//          |
//
// ai: a1 a2 a3 a4
//       6     7
//          8
//
// ai: a1 a2 a3 a4  a1 a2 a3 a4
//       |               |
//          |         |
//             |            |
//
// ai: a1 a2 a3 a4  a1 a2 a3 a4
//       9              11
//         10        12
//                         13
//
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0;n];
    for i in 0..n {
        let ai = scan.token::<usize>();
        a[i] = ai;
    }
    let mut dp = vec![vec![None;n];n];
    let mut cum = vec![0;n+1];
    for i in 1..=n {
        cum[i]+=a[i-1]+cum[i-1];
    }
    logln!("{:?}", cum);
    let ans = f(&mut dp, 0, n-1,&cum);
    fn f(dp: &mut Vec<Vec<Option<usize>>>, l: usize, r:usize, cum: &Vec<usize>) -> usize {
        if dp[l][r].is_some() {
            return dp[l][r].unwrap();
        }
        let weight = cum[r+1] - cum[l];
        if l == r {
            dp[l][r] = Some(0);
            return 0;
        }
        let mut min = usize::MAX;
        for i in l..r {
            min = min.min(f(dp,l,i,cum) + f(dp,i+1,r,cum));
        }
        let cost = min + weight;
        dp[l][r] = Some(cost);
        return cost;
    }
    logln!("{:?}", dp);
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
mod edpc_n {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4
10 20 30 40
";
        let expected = "\
190
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
10 10 10 10 10
";
        let expected = "\
120
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
3
1000000000 1000000000 1000000000
";
        let expected = "\
5000000000
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test4() {
        let input: &[u8] = b"\
6
7 6 8 6 1 1
";
        let expected = "\
68
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
