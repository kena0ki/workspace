// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc173/tasks/abc173_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = vec![(0,0);n];
    let mut negcnt = 0;
    for i in 0..n {
        let ai = scan.token::<i64>();
        a[i] = (ai.abs(),ai);
        if ai < 0 { negcnt+=1;}
    }
    a.sort_unstable_by(|a,b| b.cmp(a));
    const MOD:i64 = 1000000007;
    if negcnt == n && k %2 == 1 ||
        negcnt % 2 == 1 && k == n{
        let mut ans = 1;
        for i in (n-k..n).rev() {
            ans = (ans * a[i].1) % MOD;
        }
        writeln!(out, "{}", ans+MOD).ok();
        return ;
    }
    let mut sign = true;
    for i in 0..k {
        sign = if a[i].1 >= 0 { sign } else { !sign };
    }
    let f = |a: &Vec<(i64,i64)>| {
        let mut prod = 1;
        for i in 0..k {
            prod = (prod * a[i].1) % MOD;
        }
        return prod;
    };
    if sign {
        let ans = f(&a);
        writeln!(out, "{}", ans).ok();
        return;
    }
    let g = |r:Box<dyn Iterator<Item=usize>>, findneg:bool, a:&Vec<(i64,i64)>| {
        for i in r {
            if (a[i].1 < 0i64) == findneg{
                return Some(i);
            }
        }
        return None;
    };
    let h = |yn:bool| {
        let idx1 = g(Box::new(k..n),yn,&a);
        let idx2 = g(Box::new((0..k).rev()),!yn,&a);
        logln!("{:?}, {:?}", idx1, idx2);
        if idx1.is_some() && idx2.is_some() {
            return Some((idx1.unwrap(), idx2.unwrap()));
        }
        return None;
    };
    let ans;
    let h1 = h(true);
    let h2 = h(false);
    if h1.is_some() && h2.is_some() {
        let (idx11,idx21) = h1.unwrap();
        let (idx12,idx22) = h2.unwrap();
        if a[idx11].0 * a[idx22].0 > a[idx12].0 * a[idx21].0 {
            a.swap(idx11, idx21);
            ans = f(&a);
        } else {
            a.swap(idx12, idx22);
            ans = f(&a);
        }
    } else if h1.is_some() {
        let (idx11,idx21) = h1.unwrap();
        a.swap(idx11, idx21);
        ans = f(&a);
    } else if h2.is_some(){
        let (idx12,idx22) = h2.unwrap();
        a.swap(idx12, idx22);
        ans = f(&a);
    } else {
        unreachable!();
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

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}
impl<R: ::std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, buffer: vec![] }
    }
    pub fn token<T: ::std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

#[cfg(test)]
mod abc173d {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let output = &mut Vec::new();
                let scan = &mut Scanner::new($input as &[u8]);
                solve(scan, output);
                assert_eq!($expected, std::str::from_utf8(output).unwrap());
            }
        };
    }

    test_macro!(test1, b"\
4 2
1 2 -3 -4
" , "\
12
");
    test_macro!(test2, b"\
4 3
-1 -2 -3 -4
" , "\
1000000001
");
    test_macro!(test3, b"\
2 1
-1 1000000000
" , "\
1000000000
");
    test_macro!(test4, b"\
10 10
1000000000 100000000 10000000 1000000 100000 10000 1000 100 10 1
" , "\
999983200
");

}
