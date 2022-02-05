// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc180/tasks/abc180_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        let z = scan.token::<i64>();
        a.push((x,y,z));
    }
    let cost = |a: (i64,i64,i64), b:(i64,i64,i64)| {
        return (b.0-a.0).abs() + (b.1-a.1).abs() + 0.max(b.2-a.2);
    };
    let n2 = 1<<n;
    const INF:i64 = i64::MAX >> 10;
    let mut dp = vec![vec![INF; n]; n2];
    dp[1][0] = 0;
    for i in 0..n2 { for j in 0..n { if dp[i][j] < INF && i >> j & 1 == 1 { for k in 0..n {
        if i >> k & 1 == 1 {
            continue;
        }
        logln!("{:?}", dp);
        logln!("{},{},{}", i,j,k);
        let c = cost(a[j],a[k]);
        let ni = i|1<<k;
        dp[ni][k] = dp[ni][k].min(dp[i][j]+c);
    }}}}

    logln!("{:?}", dp);
    let mut ans = INF;
    for i in 1..n {
        let c = cost(a[i],a[0]);
        ans = ans.min(dp[n2-1][i]+c);
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
mod abc180e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2
0 0 0
1 2 3
";
        let expected = "\
9
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
3
0 0 0
1 1 1
-1 -1 -1
";
        let expected = "\
10
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
17
14142 13562 373095
-17320 508075 68877
223606 -79774 9979
-24494 -89742 783178
26457 513110 -64591
-282842 7124 -74619
31622 -77660 -168379
-33166 -24790 -3554
346410 16151 37755
-36055 51275 463989
37416 -573867 73941
-3872 -983346 207417
412310 56256 -17661
-42426 40687 -119285
43588 -989435 -40674
-447213 -59549 -99579
45825 7569 45584
";
        let expected = "\
6519344
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
