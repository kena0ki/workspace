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

// https://atcoder.jp/contests/abc225/tasks/abc225_c
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut dp = vec![vec![0;m];n];
    for i in 0..n {
        for j in 0..m {
            let b = scan.token::<usize>();
            dp[i][j] = b;

            if i > 0 && dp[i][j] - dp[i-1][j] != 7{
                writeln!(out, "No").ok();
                return;
            }
            if j > 0 && dp[i][j] - dp[i][j-1] != 1{
                writeln!(out, "No").ok();
                return;
            }
        }
    }
    writeln!(out, "Yes").ok();
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
mod abc225c {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 3
1 2 3
8 9 10
";
        let expected = "\
Yes
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test2() {
        let input: &[u8] = b"\
2 1
1
2
";
        let expected = "\
No
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test3() {
        let input: &[u8] = b"\
10 4
1346 1347 1348 1349
1353 1354 1355 1356
1360 1361 1362 1363
1367 1368 1369 1370
1374 1375 1376 1377
1381 1382 1383 1384
1388 1389 1390 1391
1395 1396 1397 1398
1402 1403 1404 1405
1409 1410 1411 1412
";
        let expected = "\
Yes
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
