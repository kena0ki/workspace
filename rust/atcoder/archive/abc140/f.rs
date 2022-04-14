// template

use std::{io::{BufRead, BufWriter, Write}, cmp::Reverse};
#[allow(unused)]
use std::collections::*;

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
        if let Some(&v) = r.rev().next() {
            self.s.insert((val,v.1 +1));
        } else {
            self.s.insert((val,0));
        }
    }
    pub fn remove_one(&mut self, val: T) -> bool {
        let r = self.s.range((Included(&(val,0)),Included(&(val,usize::max_value()))));
        if let Some(&v) = r.rev().next() {
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
            if let Some(&last) = r.rev().next() {
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
    pub fn last(&self) -> Option<T> {
        if let Some(v) = self.s.iter().rev().next() {
            return Some(v.0);
        } else {
            return None;
        }
    }
    pub fn first(&self) -> Option<T> {
        if let Some(v) = self.s.iter().next() {
            return Some(v.0);
        } else {
            return None;
        }
    }
    pub fn iter(&self) -> Map<Iter<'_, (T,usize)>, impl FnMut(&(T,usize)) -> T> {
        return self.s.iter().map(Self::filter);
    }
    fn filter(v: &(T,usize)) -> T{
        return v.0;
    }

    // this method is slow for some reason.
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
2
4 2 3 1
" , "\
Yes
");

    test_macro!(test2, b"\
2
1 2 3 1
" , "\
Yes
");

    test_macro!(test3, b"\
1
1 1
" , "\
No
");

    test_macro!(test4, b"\
5
4 3 5 3 1 2 7 8 7 4 6 3 7 2 3 6 2 7 3 2 6 7 3 4 6 7 3 4 2 5 2 3
" , "\
No
");

}

// https://atcoder.jp/contests/abc140/tasks/abc140_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let n2 = 1<<n;
    let mut heap = BinaryHeap::with_capacity(n2);
    for _ in 0..n2 {
        let s = scan.token::<usize>();
        heap.push(s);
    }
    let mut done = MultiSet::new();
    done.insert(Reverse(heap.peek().copied().unwrap()));
    for _ in 0..n {
        let pre = done.clone();
        let mut bk = BinaryHeap::with_capacity(n2);
        for Reverse(v) in pre.iter() {
            loop {
                let w = heap.pop();
                if w.is_none() {
                    writeln!(out, "No").ok();
                    return;
                }
                let w = w.unwrap();
                if v > w {
                    done.insert(Reverse(w));
                    break;
                } else {
                    bk.push(w);
                }
            }
            logln!("loop");
        }
        while let Some(w) = bk.pop() {
            heap.push(w);
        }
        logln!("{:?}",done);
    }
    writeln!(out, "Yes").ok();
}

