// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

/// Colloquially known as a "segtree" in the sport programming literature, it
/// represents a sequence of elements a_i (0 <= i < size) from a monoid (S, +)
/// on which we want to support fast range operations:
///
/// - update(l, r, f) replaces a_i (l <= i <= r) by f(a_i) for an endomorphism f
/// - query(l, r) returns the aggregate a_l + a_{l+1} + ... + a_r
///
/// This compact representation is based on a [blog post by Al.Cash]
/// (http://codeforces.com/blog/entry/18051). All nodes have 0 or 2 children.
/// Hence, trees whose size is not a power of two will have multiple roots.
///
/// Future work: ArqTree would lend itself naturally to Rust's ownership system.
/// Initially, we should only have access to the root nodes:
///            if size is a power of two, there is a unique root at index 1.
/// arq.push(i) locks i and acquires access to its children.
/// arq.pull(i) is called when the lock on i is released.
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
mod abc165f {
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
10
1 2 5 3 4 6 7 3 2 4
1 2
2 3
3 4
4 5
3 6
6 7
1 8
8 9
9 10
" , "\
1
2
3
3
4
4
5
2
2
3
");

    test_macro!(test0, b"\
6
2 3 1 1 2 3
1 2
2 3
3 4
4 5
5 6
" , "\
1
2
2
2
2
3
"
);

}

pub enum AssignMax {}
impl ArqSpec for AssignMax {
    type S = i64;
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.max(b)
    }
    fn identity() -> Self::S {
        i64::min_value()
    }
    fn compose(&f: &Self::F, _: &Self::F) -> Self::F {
        f
    }
    fn apply(&f: &Self::F, _: &Self::S, _: i64) -> Self::S {
        f
    }
}


// https://atcoder.jp/contests/abc165/tasks/abc165_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut va = Vec::with_capacity(n);
    for _ in 0..n {
        let a = scan.token::<usize>();
        va.push(a);
    }
    let (va,m) =  {
        let mut set = BTreeSet::new();
        for i in 0..n {
            set.insert(va[i]);
        }
        let mut mp = BTreeMap::new();
        let mut i = 0;
        for &k in &set {
            mp.insert(k,i);
            i+=1;
        }
        let mut nva = Vec::with_capacity(n);
        for i in 0..n {
            nva.push(mp[&va[i]]);
        }
        (nva,mp.len())
    };
    logln!("{:?},{}", va,m);
    let mut adj = vec![Vec::<usize>::with_capacity(n);n];
    for _ in 0..n-1 {
        let u = scan.token::<usize>() -1;
        let v = scan.token::<usize>() -1;
        adj[u].push(v);
        adj[v].push(u);
    }
    let ini = vec![0;m];
    let mut seg = StaticArq::<AssignMax>::new(&ini);
    let mut ans = vec![0;n];
    let mut vis = vec![false;n];
    f(&mut ans, &va, &mut seg, &adj, 0,None,n,m,&mut vis);
    for i in 0..n {
        writeln!(out, "{}", ans[i]).ok();
    }
    fn f(ans: &mut Vec<usize>, va: &Vec<usize>, seg: &mut StaticArq<AssignMax>, adj: &Vec<Vec<usize>>, u: usize, p: Option<usize>, n:usize, m:usize, vis: &mut Vec<bool>) {
        if vis[u] { return; }
        vis[u] = true;
        let vau = va[u];
        let pre = seg.query(vau,vau);
        let mx = if vau >= 1 { seg.query(0,vau-1) } else { 0 };
        seg.update(vau,vau,&pre.max(mx+1));
        ans[u] = seg.query(0,m-1) as usize;
        for &v in &adj[u] {
            if p.is_some() && p.unwrap() == v {
                continue;
            }
            f(ans, va, seg, adj, v, Some(u),n,m,vis);
        }
        seg.update(vau,vau,&pre);
    }

}
