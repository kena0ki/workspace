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
test0,
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
Copy
aaaaaa
出力例 0 
Copy
",
"\
入力例 1 
Copy
kasaka
出力例 1 
Copy
Yes
",
"\
入力例 2 
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
    let s = scan.token::<String>().as_bytes().to_vec();
    let n=s.len();
    let mut r = n-1;
    while r>0 && s[r]==b'a' { r-=1 };
    let mut l = 0;
    while l<n-1-r && s[l]==b'a' { l+=1 };
    if l>r {
        writeln!(out, "Yes").ok();
        return;
    }
    let t = s[l..r+1].to_vec();
    let mut tr = t.clone();
    tr.reverse();
    if t==tr {
        writeln!(out, "Yes").ok();
    } else {
        writeln!(out, "No").ok();
    }
    logln!("{}",String::from_utf8_lossy(&t));
    logln!("{}",String::from_utf8_lossy(&tr));
}

