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

// https://atcoder.jp/contests/abc223/tasks/abc223_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let x = scan.token::<u64>();
    let y = scan.token::<u64>();
    let a = scan.token::<u64>();
    let b = scan.token::<u64>();
    let c = scan.token::<u64>();

    let mut yn = false;
    yn = yn || check3(x,y,a,b,c);
    logln!("{}", yn);
    yn = yn || check3(x,y,b,a,c);
    logln!("{}", yn);
    yn = yn || check3(x,y,c,b,a);
    logln!("{}", yn);
    yn = yn || check3(y,x,a,b,c);
    logln!("{}", yn);
    yn = yn || check3(y,x,b,a,c);
    logln!("{}", yn);
    yn = yn || check3(y,x,c,b,a);
    logln!("{}", yn);
    fn check3(x:u64,y:u64,a:u64,b:u64,c:u64) -> bool {
        let mut yn=false;
        let x1 = (a+(y-1))/y; //round up by y-1
        logln!("x {}", x);
        logln!("y {}", y);
        logln!("x1 {}", x1);
        let x2 = x.checked_sub(x1);
        if x2.is_none() || x2.unwrap() == 0 {
            return false;
        }
        let x2 = x2.unwrap();

        // ||
        yn = yn || check2(x2,y,b,c);
        logln!("check3 {}", yn);
        // |-
        yn = yn || check2(y,x2,b,c);
        logln!("check3 {}", yn);

        fn check2(x:u64,y:u64,a:u64,b:u64) -> bool {
            let x1 = (a+(y-1))/y; //round up by y-1
            let x2 = x.checked_sub(x1);
            if x2.is_none() || x2.unwrap() == 0 {
                return false;
            }
            let x2 = x2.unwrap();
            let s2 = x2 * y;
            if s2 < b {
                return false;
            } else {
                return true;
            }
        }
        return yn;
    }
    if yn {
        writeln!(out, "Yes").ok();
    } else {
        writeln!(out, "No").ok();
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
mod abc223e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3 3 2 2 3
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
3 3 4 4 1
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
1000000000 1000000000 1000000000000000000 1000000000000000000 1000000000000000000
";
        let expected = "\
No
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
