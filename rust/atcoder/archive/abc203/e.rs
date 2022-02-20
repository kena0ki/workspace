// template

use std::{io::{BufRead, BufWriter, Write}, collections::{BTreeSet, BTreeMap, VecDeque, HashSet}};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc204/tasks/abc204_e
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let n2 = n*2;
    let m = scan.token::<usize>();
    let mut map = BTreeMap::<usize,BTreeSet<usize>>::new();
    for _ in 0..m {
        let x = scan.token::<usize>();
        let y = scan.token::<usize>();
        let e = map.entry(y).or_default();
        e.insert(x);
    }

    let e = map.entry(n).or_default();
    e.insert(0);

    let mut que = VecDeque::<(usize,usize)>::with_capacity(m);
    let mut vis = HashSet::<(usize,usize)>::with_capacity(m);
    que.push_back((0,n));
    vis.insert((0,n));
    let mut ans = 0;
    while let Some((x,y)) = que.pop_front() {
        let mut f = |y| {
            if let Some(s) = map.get(&(y)) {
                if let Some(&nx) = s.range(x+1..).next() {
                    if nx > x+1 {
                        return;
                    }
                    if vis.contains(&(nx,y)) {
                        return;
                    }
                    vis.insert((nx,y));
                    if nx == n2 {
                        ans += 1;
                    } else {
                        que.push_back((nx,y));
                    }
                }
            }
        };
        if y>0 { f(y-1); }
        if y<n2 { f(y+1); }
        let mut f2 = |y,lx:usize,rx:usize| {
            if lx > rx {
                return;
            }
            if let Some(s) = map.get(&(y)) {
                for &nx in s.range(lx..=rx) {
                    if ! vis.contains(&(nx,y)) {
                        if nx == n2 {
                            ans+=1;
                            vis.insert((nx,y));
                        } else {
                            vis.insert((nx,y));
                            que.push_back((nx,y));
                        }
                    }
                }
            }
        };
        let nx = map[&y].range(x+1..).next();
        if nx.is_none() {
            if y>0 { f2(y-1,x+1,n2); }
            if y<n2 { f2(y+1,x+1,n2); }
            if ! vis.contains(&(n2,y)) {
                vis.insert((n2,y));
                ans += 1;
            }
        } else {
            let &nx = nx.unwrap();
            logln!("{}, {:?},{:?}", x,y, nx);
            if y>0 { f2(y-1,x+1,nx); }
            if y<n2 { f2(y+1,x+1,nx); }
            if ! vis.contains(&(nx-1,y)) {
                vis.insert((nx-1,y));
                que.push_back((nx-1,y));
            }
        }
    }
    logln!("{:?}", vis);
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
mod abc204e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 4
1 1
1 2
2 0
4 2
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
1 1
1 1
";
        let expected = "\
0
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
