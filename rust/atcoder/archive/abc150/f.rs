// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

/// Single-pattern matching with the Knuth-Morris-Pratt algorithm
pub struct Kmp<'a, C: Eq> {
    /// The string pattern to search for.
    pub pattern: &'a [C],
    /// KMP match failure automaton: fail[i] is the length of the longest
    /// string that's both a proper prefix and a proper suffix of pattern[0..=i].
    pub fail: Vec<usize>,
}

impl<'a, C: Eq> Kmp<'a, C> {
    /// Precomputes the automaton that allows linear-time string matching.
    ///
    /// # Example
    ///
    /// ```
    /// use rustrithm::string_proc::Kmp;
    /// let byte_string: &[u8] = b"hello";
    /// let utf8_string: &str = "hello";
    /// let vec_char: Vec<char> = utf8_string.chars().collect();
    ///
    /// let match_from_byte_literal = Kmp::new(byte_string);
    /// let match_from_utf8 = Kmp::new(utf8_string.as_bytes());
    /// let match_from_chars = Kmp::new(&vec_char);
    ///
    /// let vec_int = vec![4, -3, 1];
    /// let match_from_ints = Kmp::new(&vec_int);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if pattern is empty.
    pub fn new(pattern: &'a [C]) -> Self {
        let mut fail = Vec::with_capacity(pattern.len());
        fail.push(0);
        let mut len = 0;
        for ch in &pattern[1..] {
            while len > 0 && pattern[len] != *ch {
                len = fail[len - 1];
            }
            if pattern[len] == *ch {
                len += 1;
            }
            fail.push(len);
        }
        Self { pattern, fail }
    }

    /// KMP algorithm, sets @return[i] = length of longest prefix of pattern
    /// matching a suffix of text[0..=i].
    pub fn kmp_match(&self, text: impl IntoIterator<Item = C>) -> Vec<usize> {
        let mut len = 0;
        text.into_iter()
            .map(|ch| {
                if len == self.pattern.len() {
                    len = self.fail[len - 1];
                }
                while len > 0 && self.pattern[len] != ch {
                    len = self.fail[len - 1];
                }
                if self.pattern[len] == ch {
                    len += 1;
                }
                len
            })
            .collect()
    }
}


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
3
0 2 1
1 2 3
" , "\
1 3
");

    test_macro!(test2, b"\
5
0 0 0 0 0
2 2 2 2 2
" , "\
0 2
1 2
2 2
3 2
4 2
");

    test_macro!(test3, b"\
6
0 1 3 7 6 4
1 5 4 6 2 3
" , "\
2 2
5 5
");

    test_macro!(test4, b"\
2
1 2
0 0
" , "\
");

    test_macro!(test5, b"\
3
0 2 1
1 2 3
" , "\
1 3
");

}

// https://atcoder.jp/contests/abc150/tasks/abc150_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut va = vec![0;n];
    for i in 0..n {
        let a = scan.token::<usize>();
        va[i] = a;
    }
    va.reverse();
    let mut vb = vec![0;n];
    for i in 0..n {
        let b = scan.token::<usize>();
        vb[i] = b;
    }
    vb.reverse();
    let vb = [vb.clone(),vb[..n-1].to_vec()].concat();
    let f = |vx: &Vec<usize>| {
        let mut res = Vec::with_capacity(n);
        let mut pre = vx[0];
        for i in 1..vx.len() {
            let v = pre^vx[i];
            res.push(v);
            pre=vx[i];
        }
        return res;
    };
    let vax = f(&va);
    let vbx = f(&vb);
    logln!("{:?}", vax);
    logln!("{:?}", vbx);

    let kmp = Kmp::new(&vax);
    let res = kmp.kmp_match(vbx);
    logln!("{:?}", res);
    let mut ans = Vec::with_capacity(n);
    let m = vax.len();
    for i in 0..res.len() {
        let num = res[i];
        if num == m {
            let k = i+1-m;
            let x = va[0]^vb[k];
            ans.push((k,x));
        }
    }
    ans.sort_unstable();
    for (k,x) in ans {
        writeln!(out, "{} {}", k,x).ok();
    }
}

