// template

use std::{io::{BufRead, BufWriter, Write}, collections::{HashSet, HashMap}};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc182/tasks/abc182_f
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let x = scan.token::<usize>();
    let mut a = vec![0;n];
    for i in 0..n {
        a[i] = scan.token::<usize>();
    }

    let mut memo = HashMap::<(usize,usize),usize>::new();
    let ans = f(x, 1, &a, n, &mut memo);
    logln!("{:?}", memo);
    writeln!(out, "{}", ans).ok();


    fn f(x:usize, i:usize, a:&Vec<usize>, n:usize,memo: &mut HashMap<(usize,usize),usize>) -> usize {
        //logln!("x: {} {}",x, i);
        if memo.contains_key(&(i,x)) {
            return memo[&(i,x)];
        }
        if i >= n {
            memo.insert((i,x),1);
            return 1;
        }
        if x == 0 {
            logln!("{}",i);
            memo.insert((i,x),1);
            return 1;
        }
        let mut res = 0;
        if x % a[i] > 0 {
            let nx = x/a[i] * a[i];
            res += f(nx, i+1, a, n, memo);
            let nx = (x+a[i]-1)/a[i] * a[i];
            res += f(nx, i+1, a, n, memo);
        } else {
            res += f(x, i+1, a, n, memo);
        }
        memo.insert((i,x), res);
        return res;
    }
}

// https://atcoder.jp/contests/abc182/tasks/abc182_f
fn _solve_not_opti(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let x = scan.token::<usize>();
    let mut a = vec![0;n];
    for i in 0..n {
        a[i] = scan.token::<usize>();
    }

    let mut set = HashSet::<usize>::new();
    let mut memo = HashSet::<(usize,usize)>::new();
    f(0, x, 0, &a, n, &mut set, x, &mut memo);
    //logln!("{:?}", set);
    writeln!(out, "{}", set.len()).ok();


    fn f(y:usize, x:usize, i:usize, a:&Vec<usize>, n:usize, set: &mut HashSet<usize>, xx:usize, memo: &mut HashSet<(usize,usize)>) {
        if memo.contains(&(i,y)) {
            return;
        }
        memo.insert((i,y));
        if i >= n {
            //logln!("x,y: {} {} {}",x, y, i);
            let ny = y + (x+a[i-1]-1)/a[i-1] * a[i-1];
            set.insert(ny);
            return;
        }
        if y >= xx {
            //logln!("{} {} {}", y, x, i);
            set.insert(y);
            return;
        }
        if x <= a[i] {
            //logln!("{} {} {}", y, x, i);
            for j in i..n {
                set.insert(y+a[j]);
            }
            return;
        }
        if x % a[i] > 0 {
            let nx = x/a[i] * a[i];
            let ny = y+x%a[i];
            f(ny, nx, i+1, a, n, set, xx, memo);
            let nx = (x+a[i]-1)/a[i] * a[i];
            f(y, nx, i+1, a, n, set, xx, memo);
        } else {
            f(y, x, i+1, a, n, set, xx, memo);
        }
        let ny = y+(x+a[i]-1)/a[i] * a[i];
        f(ny, 0, i+1, a, n, set, xx, memo);
        return;
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
mod abc182f {
    use super::*;

    //#[test]
    fn _test0() {
        let input: &[u8] = b"\
50 1001
1
2
4
8
16
32
64
128
256
512
1024
2048
4096
8192
16384
32768
65536
131072
262144
524288
1048576
2097152
4194304
8388608
16777216
33554432
67108864
134217728
268435456
536870912
1073741824
2147483648
4294967296
8589934592
17179869184
34359738368
68719476736
137438953472
274877906944
549755813888
1099511627776
2199023255552
4398046511104
8796093022208
17592186044416
35184372088832
70368744177664
140737488355328
281474976710656
562949953421312
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
3 9
1 5 10
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
5 198
1 5 10 50 100
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
    fn test3() {
        let input: &[u8] = b"\
4 44
1 4 20 100
";
        let expected = "\
4
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test4() {
        let input: &[u8] = b"\
9 11837029798
1 942454037 2827362111 19791534777 257289952101 771869856303 3859349281515 30874794252120 216123559764840
";
        let expected = "\
20
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
