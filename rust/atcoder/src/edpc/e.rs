// template

use std::{io::{BufRead, BufWriter, Write}, collections::HashMap};
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
    let w = scan.token::<usize>();
    let mut dp = Vec::with_capacity(n+1);
    dp.push(HashMap::<usize,usize>::with_capacity((n+1)*1000));
    dp[0].insert(0,0);
    let mut val_max=0;
    for i in 1..=n {
        let wi = scan.token::<usize>();
        let vi = scan.token::<usize>();
        let pre_map = &dp[i-1];
        let mut new_map = dp[i-1].clone();
        for (&k,&wj) in pre_map.iter() {
            let new = wj+wi;
            if new > w {
                continue;
            }
            let prev = pre_map.get(&(k+vi)).copied().unwrap_or(usize::MAX);
            new_map.insert(k+vi, prev.min(new));
            val_max = val_max.max(k+vi);
        }
        dp.push(new_map);
    }
    writeln!(out, "{}", val_max).ok();
}

fn _solve_d_opt(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let w = scan.token::<usize>();
    let mut values  = vec![0usize; w+1];
    for _ in 0..n {
        let wi= scan.token::<usize>();
        let vi= scan.token::<usize>();
        for j in (0..=w-wi).rev() {
            values[j+wi] = (values[j+wi]).max(values[j]+vi);
        }
    }
    println!("values: {:?}", values);
    let ans = values.iter().max();
    writeln!(out, "{}", ans.unwrap()).ok();
}

fn _solve_d(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let w = scan.token::<usize>();
    let mut values  = vec![vec![0usize; w+1]; n+1];
    for i in 1..=n {
        let wi= scan.token::<usize>();
        let vi= scan.token::<usize>();
        for j in 0..=w {
            if j < wi {
                values[i][j] = values[i-1][j];
            } else {
                values[i][j] = values[i-1][j].max(values[i-1][j-wi]+vi);
            }
        }
    }
    println!("values: {:?}", values);
    let ans = values[n].iter().max();
    writeln!(out, "{}", ans.unwrap()).ok();
}

#[cfg(test)]
mod edpc_e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3 8
3 30
4 50
5 60
";
        let expected = "\
90
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
1 1000000000
1000000000 10
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
6 15
6 5
5 6
6 4
6 6
3 5
7 2
";
        let expected = "\
17
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
