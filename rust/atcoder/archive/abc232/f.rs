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
// E.g. 1 2 3 4 5 -> 5 4 3 2 1
//
//
// swap = 0
//
// 1 2 3 4 5
//         |
// +-------+  swap +=0
// |
// v
// 5 _ _ _ _
//
// 1 2 3 4 5
//       |
//   +---+    swap +=1
//   |
//   v
// 5 4 _ _ _
//
// 1 2 3 4 5
//     |
//     |      swap +=2
//     |
//     v
// 5 4 3 _ _
//
// 1 2 3 4 5
//   |
//   +---+    swap +=3
//       |
//       v
// 5 4 3 2 _
//
// 1 2 3 4 5
// |
// +-------+  swap +=4
//         |
//         v
// 5 4 3 2 _
//
// The total swap count is 10
//
// https://atcoder.jp/contests/abc232/tasks/abc232_f
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let x = scan.token::<i64>();
    let y = scan.token::<i64>();
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for _ in 0..n {
        let ai = scan.token::<i64>();
        a.push(ai);
    }
    for _ in 0..n {
        let bi = scan.token::<i64>();
        b.push(bi);
    }
    let nn = 1usize<<n;
    let mut dp = vec![i64::MAX;nn];
    dp[0] = 0;
    for s in 0..nn {
        let j = s.count_ones() as usize;
        for i in 0..n {
            if (s >> i) & 1 == 1{
                continue;
            }
            let mut cost = dp[s];
            cost += (a[i] - b[j]).abs()*x;
            cost += (s >> i).count_ones() as i64 * y;
            dp[s|1<<i] = dp[s|1<<i].min(cost);
        }
    }
    logln!("{:?}",dp);
    writeln!(out, "{}", dp[nn -1]).ok();
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
mod abc232f {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4 3 5
4 2 5 2
6 4 2 1
";
        let expected = "\
16
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
5 12345 6789
1 2 3 4 5
1 2 3 4 5
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
18 20719114 5117250357733867
10511029 36397527 63027379 44706927 47672230 79861204 57882493 42931589 51053644 52300688 43971370 26515475 62139996 41282303 34022578 12523039 6696497 64922712
14720753 4621362 25269832 91410838 86751784 32741849 6602693 60719353 28911226 88280613 18745325 80675202 34289776 37849132 99280042 73760634 43897718 40659077
";
        let expected = "\
13104119429316474
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
