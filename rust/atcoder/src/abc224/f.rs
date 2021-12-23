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
const MOD:u64 = 998244353;

//
// https://atcoder.jp/contests/abc224/tasks/abc224_f
//
// 1 2 3 4 5 6
// ^ ^ ^ ^ ^ ^
// | | | | | |
// | | | | | +-- 6*10^0*2^n-1
// | | | | +---- 5*10^0*2^n-1-1 + 5*10^1*2^n-1-1
// | | | +------ 4*10^0*2^n-1-1 + 4*10^1*2^n-1-2 + 4*10^2*2^n-1-2
// | | +-------- 3*10^0*2^n-1-1 + 3*10^1*2^n-1-2 + 3*10^2*2^n-1-3 + 3*10^3*2^n-1-3
// | +---------- 2*10^0*2^n-1-1 + 2*10^1*2^n-1-2 + 2*10^2*2^n-1-3 + 2*10^3*2^n-1-4 + 2*10^4*2^n-1-4
// +------------ 1*10^0*2^n-1-1 + 1*10^1*2^n-1-2 + 1*10^2*2^n-1-3 + 1*10^3*2^n-1-4 + 1*10^4*2^n-1-4 + 1*10^5*2^n-1-4
//
// x[i] = s[i] * (sum((10/2)^j:{j=0~i-1})*(1/2)^(n-2) + (10/2)^i*(1/2)^(n-1))
//      = s[i] * 2^(n-2) * (sum((10/2)^j[j=0~i-1]) + (10/2)^i*2)
// ans = sum(x[i]:{i=0~n-1})
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let zero:ModU64<MOD> = ModU64::<MOD>::new(0);
    let s = scan.token::<String>();
    let s = s.chars().rev().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>();
    let n = s.len();
    if n == 1 {
        writeln!(out, "{}", s[0]).ok();
        return;
    }
    let mut a = vec![zero+1;n];
    let mut b = vec![zero;n];
    for i in 0..n-1 {
        a[i+1] = a[i]*(10/2);
        b[i+1] = b[i] + a[i];
    }
    logln!("{:?}", s);
    logln!("{:?}", a);
    logln!("{:?}", b);
    let mut ans=zero;
    for i in 0..s.len() {
        let c = b[i] + a[i]*2;
        ans += s[i] as u64 * c;
        logln!("{}: {}", i, ans);
    }
    logln!("{}", ans);
    let two = zero+2;
    ans *= two.pow((n-2) as u64);
    writeln!(out, "{}", ans).ok();
}

// c[i] = sum((10/2)^j * 1/2 :{j=0~i})
// c[i+1] = sum((10/2)^j * 1/2 :{j=0~i+1})
//        = sum((10/2)^j * 1/2 :{j=0~i}) * 10/2 + 1/2
//        = c[i] * 10/2 + 1/2
//
// ans = sum(s[i]*c[i]:{i=0~n-1}) * 2^(n-1)
//
fn _solve_elegant(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let zero:ModU64<MOD> = ModU64::<MOD>::new(0);
    let s = scan.token::<String>();
    let s = s.chars().rev().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>();
    let n = s.len();
    let mut ans = zero+0;
    let mut c = zero+1;
    let two = zero+2;
    let twoinv = two.inv();
    for i in 0..n {
        ans += s[i] * c;
        c = (c*10 + 1)*twoinv;
    }
    ans *= two.pow((n-1) as u64);
    writeln!(out ,"{}", ans).ok();
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
mod abc224f {
    use super::*;

    #[test]
    fn test0() {
        let input: &[u8] = b"\
56
";
        let expected = "\
67
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test1() {
        let input: &[u8] = b"\
1234
";
        let expected = "\
1736
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
";
        let expected = "\
1
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
31415926535897932384626433832795
";
        let expected = "\
85607943
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
