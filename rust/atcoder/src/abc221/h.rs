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
// Key word: Integer partitions
//
// P(n,k) = P(n-1,k-1) + P(n,k-1)
//
//    P(7,3) = P(n,k)
//    ^
//  k_|_____________
//    |xx // // //
//    |xx xx // //
//    |xx xx xx xx
//    +--|--|--|--|->
//     1  2  3  4
//
//    P(4,3) = P(n-k,k)
//      ^
//  k___|___________
//    xx|xx // //
//    xx|xx // //
//    xx|xx xx //
//    --+--|--|--|->
//     1  2  3  4
//
//     ^
//     | These two cases are never overlap,
//     | since the column at 2 above has 3 xx's whereas below has 2 xx's
//     v
//
//    P(6,2) = P(n-1,k-1)
//    ^
//    |xx // // //
//  k_|_____________
//    |xx xx // //
//    |xx xx xx xx
//    +--|--|--|--|->
//     1  2  3  4
//
// Reference:
//  https://drken1215.hatenablog.com/entry/2018/01/16/222843
//   This explains the case numbers are divided into k allowing 0.
//   So the formula a bit differnt: P(n,k) = P(n, k-1) + P(n-k,k)
//                                             ^
//                                             Not n-1 here.
//  https://www.youtube.com/watch?v=-q8tUGIxh3I&list=PL6daaZhBDgd-gAEBg1qVvicF-8kwsh0TM
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let a = scan.token::<usize>();
    writeln!(out, "{} 3", a).ok();
}

#[cfg(test)]
mod abc999x {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
1 2
";
        let expected = "\
1 3
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
