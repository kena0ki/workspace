use std::{io::{BufRead, BufWriter, Write}, hash::Hash};
#[allow(unused)]
use std::{{collections::*, mem::swap},cmp::Reverse};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve_wrapper(scan, out, solve);
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
}

trait MapX<K,V:Default> {
    fn update_with<F: FnOnce(&mut V) -> V>(&mut self, key:K, get_new_value:F);
}
macro_rules! update_with { () => {
    fn update_with<F: FnOnce(&mut V) -> V>(&mut self, key:K, get_new_value:F) {
        let v = self.entry(key).or_default();
        let nv = get_new_value(v);
        *v = nv;
    }
} }
impl <K:Ord,V:Default> MapX<K,V> for BTreeMap<K,V> {
    update_with!();
}
impl <K:Hash+Eq,V:Default> MapX<K,V> for HashMap<K,V> {
    update_with!();
}

trait UsizeX {
    fn i64(self) -> i64;
}
impl UsizeX for usize {
    fn i64(self) -> i64 {
        self as i64
    }
}

trait I64X {
    fn usize(self) -> usize;
}
impl I64X for i64 {
    fn usize(self) -> usize {
        self as usize
    }
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
    pub fn line<T: ::std::str::FromStr>(&mut self) -> Vec<T> {
        if !self.buffer.is_empty() {
            panic!("Consume token buffer before read a line.");
        }
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input.split_whitespace().map(|v| v.parse().ok().expect("Failed parse")).collect()
    }
    pub fn line_with<T: ::std::str::FromStr,F: FnMut(&T)->T>(&mut self, f: F) -> Vec<T> {
        return self.line().iter().map(f).collect::<Vec<_>>();
    }
}

#[cfg(test)]
mod abc999x {
    use super::*;
    use std::io::Read;

    macro_rules! test_macro {
        ($name:ident,) => {
            #[test]
            fn $name() {
                let fn_name = stringify!($name);
                let test_no = fn_name.as_bytes().last().copied().unwrap() as char;
                let bname = env!("CARGO_BIN_NAME");
                let fname = format!("src/contest/{}/in{}",bname,test_no);
                let f = std::fs::File::open(fname);
                if f.is_err() {
                    panic!("No input file");
                }
                let mut f = f.unwrap();
                let mut inp = Vec::new();
                f.read_to_end(&mut inp).unwrap();
                let fname = format!("src/contest/{}/out{}",bname,test_no);
                let f = std::fs::File::open(fname);
                let mut exp = Vec::new();
                if let Ok(mut f) = f {
                    f.read_to_end(&mut exp).unwrap();
                }
                let exp = String::from_utf8_lossy(&exp);
                let out = &mut Vec::new();
                let scan = &mut Scanner::new(&*inp);
                solve_wrapper(scan, out, solve);
                let out = String::from_utf8_lossy(&out);
                assert_eq!(exp, out);
            }
        };
        ($name:ident, $($n:ident),*,) => {
            test_macro!($name,);
            test_macro!($($n),*,);
        };
    }

    test_macro!(
test1,
test2,
test3,
);

}

fn solve_wrapper<B,W,F>(scan: &mut Scanner<B>, out: &mut W, mut solve: F)
    where B: BufRead, W: Write, F:FnMut(&mut Scanner<B>, &mut W)
    {
    let t = 1;
    //let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan,out);
    }
}

fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    writeln!(out, "{}", n/0).ok();
}




