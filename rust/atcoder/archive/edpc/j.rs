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

fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = Vec::with_capacity(n);
    let mut c = vec![0;4];
    for _ in 0..n {
        let ai = scan.token::<usize>();
        c[ai]+=1;
        a.push(ai);
    }
    let mut dp = vec![vec![vec![None;n+1];n+1];n+1];
    dp[0][0][0] = Some(0f64);
    f(&mut dp, n, c[1],c[2],c[3]);
    fn f(dp: &mut Vec<Vec<Vec<Option<f64>>>>, n: usize, c1: usize, c2: usize, c3:usize) -> f64{
        println!("n {}", n);
        println!("c {},{},{}", c1,c2,c3);
        if dp[c1][c2][c3].is_some() {
            return dp[c1][c2][c3].unwrap();
        }
        let sum = (c1+c2+c3) as f64;
        let mut exp = 1f64;
        if c1 > 0 { exp += f(dp,n,c1-1,c2,c3) * (c1 as f64/n as f64)};
        if c2 > 0 { exp += f(dp,n,c1+1,c2-1,c3) * (c2 as f64/n as f64)};
        if c3 > 0 { exp += f(dp,n,c1,c2+1,c3-1) * (c3 as f64/n as f64)};
        println!("c1 {}", c1);
        println!("exp {}", exp);
        exp *= n as f64 /sum;
        println!("exp2 {}", exp);
        dp[c1][c2][c3] = Some(exp);
        return exp;
    }
    println!("{:?}", dp);
    writeln!(out, "{}", dp[c[1]][c[2]][c[3]].unwrap()).ok();

}
// https://blog.hamayanhamayan.com/entry/2019/01/09/001607
// maybe this is incorrect
fn _solve_maybe_wrong(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = Vec::with_capacity(n);
    let mut c = vec![0;4];
    for _ in 0..n {
        let ai = scan.token::<usize>();
        c[ai]+=1;
        a.push(ai);
    }
    println!("c: {:?}", c);
    let mut dp = vec![vec![vec![0f64; n+2];n+2];n+2];
    for c3 in 0..n+1 { for c2 in 0..n+1 { for c1 in 0..n+1 {
        let sm = c1+c2+c3;
        if sm == 0 {
            continue;
        }
        dp[c1][c2][c3] = n as f64 / sm as f64;
        if c1 > 0 { dp[c1][c2][c3] += dp[c1-1][c2][c3] * ( c1 as f64 / sm as f64); }
        if c2 > 0 { dp[c1][c2][c3] += dp[c1+1][c2-1][c3] * ( c2 as f64 / sm as f64); }
        if c3 > 0 { dp[c1][c2][c3] += dp[c1][c2+1][c3-1] * ( c3 as f64 / sm as f64); }
    }}}
    //println!("{:?}", dp);
    let ans = dp[c[1]][c[2]][c[3]];
    writeln!(out, "{}", ans).ok();
}

#[cfg(test)]
mod edpc_j {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
1 1 1
";
        let expected = "\
5.5
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
3
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
    fn test3() {
        let input: &[u8] = b"\
2
1 2
";
        let expected = "\
4.5
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test4() {
        let input: &[u8] = b"\
10
1 3 2 3 3 2 3 2 1 3
";
        let expected = "\
54.48064457488221
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        let output = std::str::from_utf8(output).unwrap();
        assert_eq!(
            format!("{:.9}", expected),
            format!("{:.9}", output),
        );
    }
}
