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

// https://atcoder.jp/contests/abc178/tasks/abc178_f
// seg tree
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0;n];
    let mut cnt = vec![(0usize,0usize);n+1];
    for i in 0..n {
        a[i] = scan.token::<usize>();
        let cnta = cnt[a[i]];
        cnt[a[i]] = (cnta.0+1, a[i]);
    }
    let mut bmap = HashMap::<usize,usize>::with_capacity(n);
    for _ in 0..n {
        let b = scan.token::<usize>();
        let cntb = cnt[b];
        cnt[b] = (cntb.0+1, b);
        if cnt[b].0 > n {
            writeln!(out, "No").ok();
            return;
        }
        *bmap.entry(b).or_default() += 1;
    }
    logln!("{:?}", cnt);
    writeln!(out, "Yes").ok();
    let mut seg = StaticArq::<RangeMax>::new(&cnt);
    logln!("{:?}", seg.show());
    let mut s=vec![" ";n];
    s[n-1]="\n";
    for i in 0..n {
        let mx = seg.query(0,n);
        logln!("mx {:?}", mx);
        let mut b = mx.1;
        if a[i] == b || !bmap.contains_key(&b) {
            let mut iter = bmap.iter();
            b = *iter.next().unwrap().0;
            if a[i] == b {
                b = *iter.next().unwrap().0;
            }
        }
        seg.update(b,b,&-1);
        seg.update(a[i],a[i],&-1);
        *bmap.entry(b).or_default() -= 1;
        if bmap[&b] == 0 {
            bmap.remove(&b);
        }
        write!(out, "{}{}", b,s[i]).ok();
    }
    logln!("{:?}", seg.show());
}

pub trait ArqSpec {
    type S: Clone;
    type F: Clone;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    fn identity() -> Self::S;
    fn compose(f: &Self::F, g: &Self::F) -> Self::F;
    fn apply(f: &Self::F, a: &Self::S, size: i64) -> Self::S;
}

pub enum RangeMax {}
impl ArqSpec for RangeMax {
    type S = (usize,usize);
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.max(b)
    }
    fn identity() -> Self::S {
        (0,0)
    }
    fn compose(&_: &Self::F, _: &Self::F) -> Self::F {
        unimplemented!()
    }
    fn apply(&f: &Self::F, a: &Self::S, _: i64) -> Self::S {
        ((a.0 as i64 + f) as usize, a.1)
    }
}

pub struct StaticArq<T: ArqSpec> {
    val: Vec<T::S>,
    app: Vec<Option<T::F>>,
}

impl<T: ArqSpec> StaticArq<T> {
    /// Initializes a static balanced binary tree on top of the given sequence.
    pub fn new(init_val: &[T::S]) -> Self {
        let size = init_val.len();
        let mut val = vec![T::identity(); size];
        val.extend_from_slice(init_val);
        let app = vec![None; size];

        let mut arq = Self { val, app };
        for p in (0..size).rev() {
            arq.pull(p);
        }
        arq
    }

    fn apply(&mut self, p: usize, f: &T::F, s: i64) {
        self.val[p] = T::apply(f, &self.val[p], s);
        if let Some(lazy) = self.app.get_mut(p) {
            let h = match *lazy {
                Some(ref g) => T::compose(f, g),
                None => f.clone(),
            };
            *lazy = Some(h);
        }
    }

    fn push(&mut self, p: usize) {
        if let Some(ref f) = self.app[p].take() {
            let s = ((self.app.len() + p - 1) / p / 2).next_power_of_two() as i64;
            self.apply(p << 1, f, s);
            self.apply(p << 1 | 1, f, s);
        }
    }

    fn pull(&mut self, p: usize) {
        self.val[p] = T::op(&self.val[p << 1], &self.val[p << 1 | 1]);
    }

    fn push_to(&mut self, p: usize) {
        let one_plus_floor_log_p = (p + 1).next_power_of_two().trailing_zeros();
        for i in (1..one_plus_floor_log_p).rev() {
            self.push(p >> i);
        }
    }

    fn pull_from(&mut self, mut p: usize) {
        while p > 1 {
            p >>= 1;
            self.pull(p);
        }
    }

    /// Applies the endomorphism f to all entries from l to r, inclusive.
    /// If l == r, the updates are eager. Otherwise, they are lazy.
    ///
    /// # Panics
    ///
    /// Panics if r >= size. Note that l > r is valid, meaning an empty range.
    pub fn update(&mut self, mut l: usize, mut r: usize, f: &T::F) {
        l += self.app.len();
        r += self.app.len();
        if l < r {
            self.push_to(l);
        }
        self.push_to(r);
        let (mut l0, mut r0, mut s) = (1, 1, 1);
        while l <= r {
            if l & 1 == 1 {
                self.apply(l, f, s);
                l0 = l0.max(l);
                l += 1;
            }
            if r & 1 == 0 {
                self.apply(r, f, s);
                r0 = r0.max(r);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
            s <<= 1;
        }
        self.pull_from(l0);
        self.pull_from(r0);
    }

    /// Returns the aggregate range query on all entries from l to r, inclusive.
    ///
    /// # Panics
    ///
    /// Panics if r >= size. Note that l > r is valid, meaning an empty range.
    pub fn query(&mut self, mut l: usize, mut r: usize) -> T::S {
        l += self.app.len();
        r += self.app.len();
        if l < r {
            self.push_to(l);
        }
        self.push_to(r);
        let (mut l_agg, mut r_agg) = (T::identity(), T::identity());
        while l <= r {
            if l & 1 == 1 {
                l_agg = T::op(&l_agg, &self.val[l]);
                l += 1;
            }
            if r & 1 == 0 {
                r_agg = T::op(&self.val[r], &r_agg);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(&l_agg, &r_agg)
    }

    /// For debug
    pub fn show(self: &Self) -> &[T::S] {
        return &self.val[self.app.len()..];
    }
}

// https://atcoder.jp/contests/abc178/tasks/abc178_f
// heap
fn _solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![Vec::new();n];
    let mut heap = BinaryHeap::with_capacity(n);
    for i in 0..n {
        let ai = scan.token::<usize>()-1;
        a[ai].push(i);
    }
    let mut b = vec![Vec::new();n];
    for i in 0..n {
        let bi = scan.token::<usize>()-1;
        b[bi].push(i);
        if b[bi].len() + a[bi].len() > n {
            writeln!(out, "No").ok();
            return;
        }
    }
    for i in 0..n {
        if b[i].len() == 0 && a[i].len() == 0 { continue; };
        heap.push((b[i].len() + a[i].len() , i));
    }

    writeln!(out, "Yes").ok();

    logln!("{:?}", heap);
    let mut ans = vec![0;n];
    let mut heap2 = BinaryHeap::with_capacity(n);
    for _ in 0..n {
        logln!("heap : {:?}",heap);
        logln!("heap2: {:?}",heap2);
        let mut fst = if heap2.len() > 0 {
            heap2.pop().unwrap()
        } else {
            heap.pop().unwrap()
        };
        let sec = (|| {
            let mut sec;
            loop {
                sec = heap.pop().unwrap();
                if a[fst.1].len() > 0 && b[sec.1].len() > 0 ||
                   b[fst.1].len() > 0 && a[sec.1].len() > 0 {
                    break;
                }
                if sec.0 > fst.0 {
                    std::mem::swap(&mut sec,&mut fst);
                }
                heap2.push(sec);
            }
            return sec;
        })();
        if a[fst.1].len() > 0 && b[sec.1].len() > 0{
            ans[a[fst.1].pop().unwrap()] = sec.1+1;
            b[sec.1].pop();
        } else {
            ans[a[sec.1].pop().unwrap()] = fst.1+1;
            b[fst.1].pop();
        }
        heap.push((fst.0 - 1, fst.1));
        heap.push((sec.0 - 1, sec.1));
        logln!("{:?}", ans);
    }
    let mut s = vec![" "; n];
    s[n-1] = "\n";
    for i in 0..n {
        write!(out, "{}{}", ans[i],s[i]).ok();
    }
}

// https://atcoder.jp/contests/abc178/tasks/abc178_f
// multiset
fn _solve_ms(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut va = Vec::with_capacity(n);
    let mut cnt = vec![0;n+1];
    for _ in 0..n {
        let a = scan.token::<usize>();
        va.push(a);
        cnt[a] += 1;
    }
    let mut vb = Vec::with_capacity(n);
    let mut cntb = vec![0;n+1];
    for _ in 0..n {
        let b = scan.token::<usize>();
        vb.push(b);
        cnt[b] += 1;
        cntb[b] += 1;
    }
    let mut ms = MultiSet::new();
    let mut msb = MultiSet::new();
    for i in 1..n+1 {
        if cnt[i] > 0 {
            ms.insert((cnt[i],i));
        }
        if cntb[i] > 0 {
            msb.insert((cntb[i],i));
        }
    }
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        logln!("ms :{:?}",ms);
        logln!("msb:{:?}",msb);
        let (max,v) = ms.last().unwrap();
        if max > n-i {
            writeln!(out, "No").ok();
            return;
        }
        let a = va[i];
        ms.remove_one((cnt[a],a));
        cnt[a] -= 1;
        if cnt[a] > 0 {
            ms.insert((cnt[a],a));
        }
        let mut bp;
        if v != a && msb.contains((cntb[v],v)) {
            bp = (cntb[v],v);
        } else {
            let mut iter = msb.iter().rev();
            bp = iter.next().unwrap();
            if bp.1 == a {
                if iter.len() == 0 {
                    writeln!(out, "No").ok();
                    return;
                }
                bp = iter.next().unwrap();
            }
        }
        let (_, b) = bp;
        ans.push(b);
        msb.remove_one((cntb[b],b));
        cntb[b] -= 1;
        if cntb[b] > 0 {
            msb.insert((cntb[b],b));
        }
        ms.remove_one((cnt[b],b));
        cnt[b] -= 1;
        if cnt[b] > 0 {
            ms.insert((cnt[b],b));
        }
    }
    writeln!(out, "Yes").ok();
    let mut delim = vec![" ";n];
    delim[n-1] = "\n";
    for i in 0..n {
        write!(out, "{}{}", ans[i], delim[i]).ok();
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

#[cfg(test)]
mod abc178f {
    use super::*;

    #[test]
    fn test0() {
        let input: &[u8] = b"\
6
1 1 2 2 3 3
1 1 1 4 5 6
";
        let expected = "\
Yes
2 3 2 1 1 1
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test1() {
        let input: &[u8] = b"\
6
1 1 1 2 2 3
1 1 1 2 2 3
";
        let expected = "\
Yes
2 3 2 1 1 1
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
3
1 1 2
1 1 3
";
        let expected = "\
No
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
4
1 1 2 3
1 2 3 3
";
        let expected = "\
Yes
3 3 1 2
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
