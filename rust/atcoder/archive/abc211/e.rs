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

// https://atcoder.jp/contests/abc211/tasks/abc211_e
// WA
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut grid = vec![vec![0;n];n];
    for i in 0..n {
        let s = scan.token::<String>();
        let s = s.as_bytes();
        for j in 0..s.len() {
            if s[j] == b'#' {
                grid[i][j] = 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans += f(i,j,k-1,n, None, None, &grid, i,j);
            grid[i][j] = 1;
            logln!("{}", ans);
        }
    }

    const DX: [i32;4] = [1, 0, -1, 0];
    const DY: [i32;4] = [0, -1, 0, 1];
    fn f(i:usize, j:usize, r:usize, n:usize,
        pi:Option<usize>, pj:Option<usize>, grid: &Vec<Vec<usize>>,
        si:usize, sj: usize) -> usize {

        if r == 0 {
            logln!("{},{}", i,j);
            return 1;
        }
        let mut res = 0;
        for di in 0..4 {
            let ni = i as i32 + DX[di];
            let nj = j as i32 + DY[di];
            if ni<0 || nj<0 || ni>=n as i32 || nj>=n as i32{
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if pi.is_some() && pj.is_some() && ni == pi.unwrap() && nj == pj.unwrap() {
                res += f(ni,nj,r,n,Some(i), Some(j), grid, si,sj);
                continue;
            }
            if grid[ni][nj] == 1 {
                continue;
            }
            res += f(ni,nj,r-1,n,Some(i), Some(j), grid, si,sj);
        }

        return res;
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
mod abc211e {
    use super::*;

    #[test]
    fn test0() {
        let input: &[u8] = b"\
3
2
...
...
...
";
        let expected = "\
10
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        logln!("{}", -1%3);
        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
5
#.#
...
..#
";
        let expected = "\
5
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test2() {
        let input: &[u8] = b"\
2
2
#.
.#
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
8
8
........
........
........
........
........
........
........
........
";
        let expected = "\
64678
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
