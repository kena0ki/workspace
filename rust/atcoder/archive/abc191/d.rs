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
mod abc999x {
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
0.2 0.8 1.1
" , "\
3
");

    test_macro!(test2, b"\
100 100 1
" , "\
5
");

    test_macro!(test3, b"\
42782.4720 31949.0192 99999.99
" , "\
31415920098
");

}

// https://atcoder.jp/contests/abc191/tasks/abc191_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let x=-7;
    let y=3;
    let z=7;
    writeln!(out, "{},{}",(x-(((x%y)+y)%y))/y,(z-(((z%y)+y)%y))/y).ok();
    let x = scan.token::<f64>();
    let y = scan.token::<f64>();
    let r = scan.token::<f64>();
    let base = 10000;
    let x = ((x+0.000001)*base as f64).round() as i64;
    let y = ((y+0.000001)*base as f64).round() as i64;
    let r = ((r+0.000001)*base as f64).round() as i64;
    let x = ((x%base)+base)%base;
    let y = ((y%base)+base)%base;
    logln!("xyr:{},{},{}",x,y,r);

    let f = |x1:i64,yi:i64,x:i64,y:i64| {
        let mut fl = x1;
        let mut fr = (x+r)/base*base + base;
        let dy = yi-y;
        //logln!("lrx:{},{},{},{}",fl,fr,x,y);
        let dx = x1-x;
        if dx*dx + dy*dy > r*r { return 0 };
        while fl+base < fr {
            let xi = (fl+fr)/2/base*base;
            let dx = xi-x;
            if dy*dy+dx*dx <= r*r { fl=xi; }
            else { fr=xi; }
        }
        //logln!("{},{}",fl,x1);
        return (fl-x1)/base + 1;
    };
    //let f = |x1:i64,yi:i64,x:i64,y:i64| {
    //    let mut xi = x1;
    //    let mut dx = xi-x;
    //    let mut dy = yi-y;
    //    while dx*dx+dy*dy <= r*r {
    //        xi+=base;
    //        dx = xi-x;
    //        dy = yi-y;
    //    }
    //    return xi;
    //};
    let mut ans = 0;
    let ox = x;
    let oy = y;
    let x1 = (x+base)/base*base;
    let y1 = (y+base)/base*base;
    for i in 0..4 {
        let mut x = ox;
        let mut y = oy;
        if i & 1 == 1 { x = base-ox;}
        if i & 2 == 2 { y = base-oy;}
        let y2 = (y+r)/base*base;
        let xi=x1;
        for yi in (y1..=y2+base).rev().step_by(base as usize) {
            //logln!("{}",yi);
            //let res = f(xi,yi,x,y);
            //ans+=(res-x1)/base;
            //xi=res;
            ans+=f(xi,yi,x,y);
        }
        logln!("ans:{}",ans);
    }
    writeln!(out, "{}", ans).ok();
}

