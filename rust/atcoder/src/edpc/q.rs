// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, range_query::{StaticArq, specs::AssignMax}};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

//
// Analogous to the Longest Increasing Subsequence problem.
//  https://en.wikipedia.org/wiki/Longest_common_subsequence_problem
//  https://www.geeksforgeeks.org/longest-increasing-subsequence-dp-3/
//
//             h
//             ^
//             |
//           6 |           _
//           5 |     _     |
//           4 |  _  |     |
//           3 |  |  |     |  _
//           2 |  |  |  _  |  |
//           1 |  |  |  |  |  |
//             +------------------------   
// a_i:          a1 a2 a3 a4 a5
// height:        3  4  1  5  2
// value:         2  5  1  3  4
// dp[height]:    2  7  1  8  3  <- dp[height] = max(dp[0~height]) + value
//                                  This is just a conceptual variable.
//                                  The calculation is done by segement tree.
//
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut h = Vec::with_capacity(n);
    let mut maxh = 0;
    for _ in 0..n {
        let hi = scan.token::<usize>();
        h.push(hi);
        maxh = hi.max(maxh);
    }
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let ai = scan.token::<i64>();
        a.push(ai);
    }
    let mut st = StaticArq::<AssignMax>::new(&vec![0;maxh+1][..]);
    for i in 0..n {
        let max = st.query(0,h[i]);
        st.update(h[i], h[i], &(max+a[i]));
    }
    // logln!("{:?}", st.show());
    writeln!(out, "{}", st.query(0, maxh)).ok();
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
mod edpc_q {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4
3 1 4 2
10 20 30 40
";
        let expected = "\
60
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
1
1
10
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
5
1 2 3 4 5
1000000000 1000000000 1000000000 1000000000 1000000000
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
9
4 2 5 8 3 6 1 7 9
6 8 8 4 6 3 5 7 5
";
        let expected = "\
31
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
