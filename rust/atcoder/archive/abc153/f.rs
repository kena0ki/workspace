// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

pub trait ArqSpec {
    /// Type of underlying array elements.
    type S: Clone;
    /// Type of data representing an endomorphism.
    // Note that while a Fn(S) -> S may seem like a more natural representation
    // for an endomorphism, compositions would then have to delegate to each of
    // their parts. This representation is more efficient.
    type F: Clone;

    /// Must satisfy the Associative Law:
    /// For all a,b,c, op(a, op(b, c)) = op(op(a, b), c)
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    /// Must satisfy the Identity Law:
    /// For all a, op(a, identity()) = op(identity(), a) = a
    fn identity() -> Self::S;
    /// Must satisfy the Composition Law:
    /// For all f,g,a, apply(compose(f, g), a) = apply(f, apply(g, a))
    fn compose(f: &Self::F, g: &Self::F) -> Self::F;
    /// Must satisfy the Distributive Law:
    /// For all f,a,b, apply(f, op(a, b), s+t) = op(apply(f, a, s), apply(f, b, t))
    /// The `size` parameter makes this law easier to satisfy in certain cases.
    fn apply(f: &Self::F, a: &Self::S, size: i64) -> Self::S;

    // The following relaxations to the laws may apply.
    // If only point updates are made, the Composition and Distributive Laws
    // no longer apply.
    // - compose() is never called, so it can be left unimplemented!().
    // - apply() is only ever called on leaves, i.e., with size == 1.
    // If only point queries are made, the Associative and Distributive Laws
    // no longer apply.
    // - op()'s result only matters when identity() is an argument.
    // - apply()'s result only matters on leaves, i.e., with size == 1.
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
pub enum ArqSum {}
impl ArqSpec for ArqSum {
    type S = i64;
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a + b
    }
    fn identity() -> Self::S {
        0
    }
    fn compose(&f: &Self::F, g: &Self::F) -> Self::F {
        f+g
    }
    fn apply(&f: &Self::F, a: &Self::S, size: i64) -> Self::S {
        (f+a) * size
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
3 3 2
1 2
5 4
9 2
" , "\
2
");

    test_macro!(test2, b"\
9 4 1
1 5
2 4
3 3
4 2
5 1
6 2
7 3
8 4
9 5
" , "\
5
");

    test_macro!(test3, b"\
3 0 1
300000000 1000000000
100000000 1000000000
200000000 1000000000
" , "\
3000000000
");

}

// https://atcoder.jp/contests/abc153/tasks/abc153_f
fn _solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let d = scan.token::<i64>();
    let a = scan.token::<i64>();
    let mut vx = vec![0;n];
    let mut mh = HashMap::with_capacity(n);
    let mut set = BTreeSet::new();
    for i in 0..n {
        let x = scan.token::<i64>();
        let h = scan.token::<i64>();
        vx[i] =x;
        mh.insert(x,h);
        set.insert(x);
    }
    let mut mp = BTreeMap::new();
    let mut mp2 = BTreeMap::new();
    let mut cnt = 0;
    for &x in &set {
        mp.insert(x,cnt);
        mp2.insert(cnt,x);
        cnt+=1;
    }
    let m = mp.len();
    let mut ini = vec![0;m+1];
    for (x,&c) in &mp {
        ini[c] = mh[x];
    }
    let mut seg = StaticArq::<ArqSum>::new(&ini);
    let mut ans = 0;
    for i in 0..m {
        let s = seg.query(i,i);
        logln!("i,s:{},{}",i,s);
        if s <= 0 { continue; }
        let lx = mp2[&i];
        let r = mp.range(lx..=(lx+2*d)).rev().next();
        if r.is_none() { continue; }
        let &r = r.unwrap().1;
        logln!("r:{}",r);
        let ceil = (s+a-1)/a;
        let val = ceil * a;
        seg.update(i,r,&-val);
        ans += ceil;
    }
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc153/tasks/abc153_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let d = scan.token::<i64>();
    let a = scan.token::<i64>();
    let mut vx = vec![(0,0);n];
    for i in 0..n {
        let x = scan.token::<i64>();
        let h = scan.token::<i64>();
        vx[i] =(x,h);
    }
    vx.sort_unstable();
    let mut vc = vec![0;n+1];
    let mut cum = 0;
    let mut r = 0;
    let mut ans = 0;
    for i in 0..n {
        let (x,h) = vx[i];
        cum += vc[i];
        if h <= cum { continue; }
        let rem = h-cum;
        let ceil = (rem+a-1)/a;
        ans+=ceil;
        let val=ceil*a;
        cum+=val;
        while r<n && vx[r].0<=(x+2*d) {
            r+=1;
        }
        vc[r]-=val;
    }
    writeln!(out, "{}", ans).ok();
}

