use std::{io::{BufRead, BufWriter, Write}, hash::Hash};
#[allow(unused)]
use std::{{collections::*, mem::swap},cmp::Reverse};

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
}

#[cfg(test)]
mod abc999x {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, | $input:expr,) => {
            #[test]
            fn $name() {
                let vi = $input.split("\n").collect::<Vec<_>>();
                let vi = &vi[1..vi.len()-1];
                let mut inp = String::new();
                let mut cnt=0;
                for &line in vi {
                    if line.starts_with("出力例") { break; }
                    cnt+=1;
                    if line.starts_with("入力例") || line.starts_with("Copy") { continue; }
                    inp+=line; inp+="\n";
                }
                let mut exp = String::new();
                for &line in &vi[cnt..] {
                    if line.starts_with("出力例") || line.starts_with("Copy") { continue; }
                    exp+=line; exp+="\n";
                }
                let output = &mut Vec::new();
                let scan = &mut Scanner::new(inp.as_bytes());
                solve(scan, output);
                assert_eq!(exp, std::str::from_utf8(output).unwrap());
            }
        };
        ($name:ident, $($n:ident),*, | $input:expr, $($i:expr),*,) => {
            test_macro!($name, | $input,);
            test_macro!($($n),*, | $($i),*,);
        };
    }

    test_macro!(
est0,
test1,
test2,
test3,
est4,
est5,
est6,
est7,
|
"\
入力例 0 
",
"\
入力例 1 
Copy
5
LRRLR
出力例 1 
Copy
1 2 4 5 3 0
",
"\
入力例 2 
Copy
7
LLLLLLL
出力例 2 
Copy
7 6 5 4 3 2 1 0
",
"\
入力例 3 
",
"\
入力例 4 
",
"\
入力例 5 
",
"\
入力例 6 
",
"\
入力例 7 
",
);

}


fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut n:usize = scan.token();
    let vs = scan.token::<String>().as_bytes().to_vec();
    let mut ans = VecDeque::new();
    ans.push_back(n);
    for &s in vs.iter().rev() {
        n-=1;
        if s==b'R' {
            ans.push_front(n);
        } else {
            ans.push_back(n);
        }
    }
    for i in 0..ans.len() {
        write!(out, "{} ", ans[i]).ok();
    }
    writeln!(out).ok();
}

