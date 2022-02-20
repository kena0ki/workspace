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

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    for _ in 0..n {
        let xi = scan.token::<f64>();
        let yi = scan.token::<f64>();
        x.push(xi);
        y.push(yi);
    }
    let ans= f(n,0,0,vec![], &x,&y);
    writeln!(out, "{}", ans).ok();
    fn f(n:usize, i:usize, j:usize, ids: Vec<usize>, x:&Vec<f64>, y: &Vec<f64>) -> usize {
        if i == 3 {
            if check(ids,x,y) {
                return 1;
            } else {
                return 0;
            }
        }
        if j == n-2 {
            return 0;
        }
        let mut cnt=0;
        let newids = [&ids[..], &[i+j][..]].concat();
        cnt += f(n, i+1, j,newids, x, y);
        cnt += f(n, i, j+1,ids, x, y);
        return cnt;
    }
    fn check(ids: Vec<usize>, x: &Vec<f64>, y: &Vec<f64>) -> bool {
        // logln!("{:?}", x);
        // logln!("{:?}", y);
        logln!("{:?}", ids);
        let x1 = x[ids[0]];
        let y1 = y[ids[0]];
        let x2 = x[ids[1]];
        let y2 = y[ids[1]];
        let x3 = x[ids[2]];
        let y3 = y[ids[2]];
        if x2-x1 == 0f64 && x3-x1 == 0f64{
            return true;
        }
        let tan1 = (y2 - y1) / (x2 - x1);
        let tan2 = (y3 - y1) / (x3 - x1);
        // logln!("tan1 {}", tan1);
        // logln!("tan2 {}", tan2);
        let tan1 = format!("{:.9}", tan1);
        let tan2 = format!("{:.9}", tan2);
        return tan1 != tan2;
    }
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
mod abc224c {
    use super::*;

    #[test]
    fn test0() {
        let input: &[u8] = b"\
5
0 1
1 3
1 1
-1 -1
0 0
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
    fn test1() {
        let input: &[u8] = b"\
4
0 1
1 3
1 1
-1 -1
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
20
224 433
987654321 987654321
2 0
6 4
314159265 358979323
0 0
-123456789 123456789
-1000000000 1000000000
124 233
9 -6
-4 0
9 5
-7 3
333333333 -333333333
-9 -1
7 -10
-1 5
324 633
1000000000 -1000000000
20 0
";
        let expected = "\
1124
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
