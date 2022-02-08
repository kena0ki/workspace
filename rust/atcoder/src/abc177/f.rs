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

// https://atcoder.jp/contests/abc177/tasks/abc177_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let mut cur = BTreeMap::<usize,usize>::new();
    let mut lines = Vec::with_capacity(h);
    for _ in 0..h {
        let a = scan.token::<usize>();
        let b = scan.token::<usize>();
        lines.push((a,b));
    }
    let mut ms = MultiSet::<usize>::new();
    for i in 0..w {
        cur.insert(i+1, i+1);
        ms.insert(0);
    }
    for i in 0..h {
        let r = cur.range(lines[i].0..=lines[i].1);
        logln!("{:?}", lines[i]);
        logln!("{:?}", r);
        let ents = r.map(|e| (*e.0,*e.1)).collect::<Vec<_>>();
        let len = ents.len();
        for i in 0..len {
            cur.remove(&ents[i].0);
            ms.remove_one(ents[i].0 - ents[i].1);
        }
        if lines[i].1 < w && ents.len() > 0{
            let last = ents[ents.len()-1];
            let adv = lines[i].1+1 - last.1;
            cur.insert(last.1 + adv, last.1);
            ms.insert(adv);
        }
        logln!("{} {:?} {:?}", i, cur, ms);
        if let Some(ans) =ms.iter().next() {
            writeln!(out, "{}", ans+i+1).ok();
        } else {
            writeln!(out, "-1").ok();
        }
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

use std::collections::BTreeSet;
use std::collections::btree_set::{Iter, Range};
use std::iter::Map;
use std::ops::Bound::{Included, self,Excluded};
use std::ops::RangeBounds;


#[derive(Debug,Clone)]
pub struct MultiSet<T:Ord+Copy> {
    s: BTreeSet<(T,usize)>
}

impl <T:Ord+Copy> MultiSet<T> {
    pub fn new() -> Self {
        return Self{s: BTreeSet::<(T,usize)>::new()};
    }
    pub fn insert(&mut self, val: T) {
        let r = self.s.range((Included(&(val,0)),Included(&(val,usize::max_value()))));
        if let Some(&v) = r.last() {
            self.s.insert((val,v.1 +1));
        } else {
            self.s.insert((val,0));
        }
    }
    pub fn remove_one(&mut self, val: T) -> bool {
        let r = self.s.range((Included(&(val,0)),Included(&(val,usize::max_value()))));
        if let Some(&v) = r.last() {
            return self.s.remove(&v);
        }
        return false;
    }
    pub fn remove_all(&mut self, val: T) -> usize {
        let r = self.s.range((Included(&(val,0)),Included(&(val,usize::max_value()))));
        let len = self.s.len();
        let vec = r.copied().collect::<Vec<_>>();
        for v in &vec {
            self.s.remove(v);
        }
        return len - self.s.len();
    }
    pub fn get(&self, val: T) -> Option<T> {
        if let Some(v) = self.s.get(&(val,0)) {
            return Some(v.0);
        }
        return None;
    }
    pub fn count(&self, val: T) -> usize {
        let mut r = self.s.range((Included(&(val,0)),Included(&(val,usize::max_value()))));
        if let Some(&first) = r.next() {
            if let Some(&last) = r.last() {
                return last.1 - first.1 + 1;
            }
            return 1;
        }
        return 0;
    }
    pub fn contains(&self, val: T) -> bool {
        return self.get(val).is_some();
    }
    pub fn len(&self) -> usize {
        return self.s.len();
    }
    pub fn is_empty(&self) -> bool {
        return self.s.is_empty();
    }
    pub fn iter(&self) -> Map<Iter<'_, (T,usize)>, impl FnMut(&(T,usize)) -> T> {
        return self.s.iter().map(Self::filter);
    }
    fn filter(v: &(T,usize)) -> T{
        return v.0;
    }

    pub fn range<R>(&self, range: R) -> Map<Range<'_, (T,usize)>, impl FnMut(&(T,usize)) -> T>
    where
        R: RangeBounds<T>,
    {
        let start = match range.start_bound() {
            Bound::Unbounded => Bound::Unbounded,
            Included(&b) => Included((b,0)),
            Excluded(&b) => Excluded((b,usize::max_value())),
        };
        let end = match range.end_bound() {
            Bound::Unbounded => Bound::Unbounded,
            Included(&b) => Included((b,usize::max_value())),
            Excluded(&b) => Excluded((b,0)),
        };
        return self.s.range((start,end)).map(Self::filter);
    }
    pub fn multiset(&self) -> &BTreeSet<(T,usize)> {
        return &self.s;
    }
}
#[cfg(test)]
mod abc177f {
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
4 4
2 4
1 1
2 3
2 4
" , "\
1
3
6
-1
");

}
