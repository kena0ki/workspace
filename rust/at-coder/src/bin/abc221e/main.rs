use text_io::read;
use seg_tree::static_arq::StaticArq;
use seg_tree::specs::ArqSpec;
use util::compress;
use modulo::{ModUsize,ModUsizeFactory};

// const MOD: usize = 53;
const MOD: usize = 998244353;

struct ArqImpl;
impl ArqSpec for ArqImpl {
    type S = ModUsize;
    type F = ModUsize;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        return a+b;
    }
    fn identity() -> Self::S {
        return ModUsizeFactory::new(MOD).create(0);
    }
    fn compose(&f: &Self::F, _: &Self::F) -> Self::F {
        return f;
    }
    fn apply(&f: &Self::F, _: &Self::S, _: i64) -> Self::S {
        return f;
    }
}


// https://atcoder.jp/contests/abc221/tasks/abc221_e
// - input:
// 10
// 198495780 28463047 859606611 212983738 946249513 789612890 782044670 700201033 367981604 302538501
// - expected:
// 830
fn main(){
    let n:usize = read!();
    let mut a = vec![0usize;n];
    for i in 0..n {
        a[i]=read!();
    }
    let (arr,m) = compress(&mut a);
    let f = ModUsizeFactory::new(MOD);
    let v = vec![f.create(0); m];
    let mut seg = StaticArq::<ArqImpl>::new(&v);
    let mut ans = f.create(0);
    let mut m2 = f.create(2);
    let mut m2_inv = m2.inv();
    for i in 1..n {
        seg.update(arr[i-1], arr[i-1], &m2_inv);
        let sum = seg.query(0, arr[i]) * m2;
        ans = ans + sum;
        m2.mul_by(2);
        m2_inv.div_by(2);
    }
    println!("{}", ans);
}


pub mod modulo {
    use core::fmt;
    use std::ops::{Add,Sub,Mul,Div};

    type NumberType = usize;
    pub struct ModUsizeFactory (NumberType);
    impl ModUsizeFactory {
        pub fn new(modulus:usize) -> Self{
            return Self(modulus);
        }
        pub fn create(self: &Self, val: NumberType) -> ModUsize{
            return ModUsize {
                modulus: self.0,
                val: val%self.0,
            };
        }
    }

    #[derive(Debug,Clone,Copy)]
    pub struct ModUsize{
        modulus: NumberType,
        pub val: NumberType,
    }
    impl ModUsize {
        pub fn sibling(self: &Self, val:usize) -> Self {
            return Self {
                modulus: self.modulus,
                val: val%self.modulus,
            };
        }
        pub fn set_val(self: &mut Self, val: usize) {
            self.val = val %self.modulus;
        }
        pub fn add_by(self: &mut Self, rhs: NumberType) {
            self.val = self.add_premitive(self.val, rhs%self.modulus);
        }
        pub fn sub_by(self: &mut Self, rhs: NumberType) {
            self.val = self.sub_premitive(self.val, rhs%self.modulus);
        }
        pub fn mul_by(self: &mut Self, rhs: NumberType) {
            self.val = self.mul_premitive(self.val, rhs%self.modulus);
        }
        pub fn div_by(self: &mut Self, rhs: NumberType) {
            self.val = self.div_premitive(self.val, rhs%self.modulus);
        }
        pub fn pow(self: Self, mut power: NumberType) -> Self{
            let mut square = self.val;
            let mut ret = 1;
            while 0 < power {
                if (power & 1) == 1{
                    ret *= square;
                    ret %= self.modulus;
                }
                square *= square;
                square %= self.modulus;
                power >>= 1;
            }
            return Self {
                val:ret,
                modulus: self.modulus,
            };
        }
        pub fn inv(self: Self) -> Self {
            return self.pow(self.modulus - 2);
        }
        fn add_premitive(self: &Self, mut lhs: NumberType, rhs: NumberType) -> NumberType{ // lhs and rhs should not be greater than modulus.
            lhs += rhs;
            if lhs >= self.modulus {
                lhs -= self.modulus;
            }
            return lhs;
        }
        fn sub_premitive(self: &Self, mut lhs: NumberType, rhs: NumberType) -> NumberType{ // lhs and rhs should not be greater than modulus.
            if lhs < rhs {
                lhs += self.modulus - rhs;
            } else {
                lhs -= rhs;
            }
            return lhs;
        }
        fn mul_premitive(self: &Self, lhs: NumberType, rhs: NumberType) -> NumberType{ // lhs and rhs should not be greater than modulus.
            return (lhs * rhs) % self.modulus;
        }
        fn div_premitive(self: &Self, mut lhs: NumberType, rhs: NumberType) -> NumberType{ // lhs and rhs should not be greater than modulus.
            let mut power = self.modulus - 2;
            let mut square = rhs;
            while 0 < power {
                if (power & 1) == 1{
                    lhs *= square;
                    lhs %= self.modulus;
                }
                square *= square;
                square %= self.modulus;
                power >>= 1;
            }
            return lhs;
        }
    }
    impl Add for ModUsize {
        type Output = Self;
        fn add(mut self: Self, rhs: Self) -> Self {
            self.val = self.add_premitive(self.val, rhs.val);
            return self;
        }
    }
    impl Sub for ModUsize {
        type Output = Self;
        fn sub(mut self: Self, rhs: Self) -> Self {
            self.val = self.sub_premitive(self.val, rhs.val);
            return self;
        }
    }
    impl Mul for ModUsize {
        type Output = Self;
        fn mul(mut self: Self, rhs: Self) -> Self {
            self.val = self.mul_premitive(self.val, rhs.val);
            return self;
        }
    }
    impl Div for ModUsize {
        type Output = Self;
        fn div(mut self: Self, rhs: Self) -> Self {
            self.val = self.div_premitive(self.val, rhs.val);
            return self;
        }
    }
    impl fmt::Display for ModUsize {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            return write!(f, "{}",self.val);
        }
    }
}

pub mod util {
    use std::collections::{BTreeMap,BTreeSet};
    pub fn compress<T:Ord+Clone+Copy>(a: &mut Vec<T>) -> (Vec<usize>, usize) {
        let mut set = BTreeSet::<T>::new();
        for i in 0..a.len() {
            set.insert(a[i]);
        }
        let mut size = 0;
        let mut mem = BTreeMap::<T,usize>::new();
        for key in set {
            mem.insert(key, size);
            size+=1;
        }
        let mut ret = vec![0; a.len()];
        for i in 0..a.len() {
            ret[i] = *mem.get(&a[i]).unwrap();
        }
        return (ret, size);
    }
}


pub mod seg_tree {
    pub mod specs {
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

        /// Range Minimum Query (RMQ), a classic application of ARQ.
        /// update(l, r, &f) sets all entries a[l..=r] to f.
        /// query(l, r) finds the minimum value in a[l..=r].
        //
        // Exercises: try augmenting this struct to find the index of a minimum element
        // in a range query, as well as the number of elements equal to the minimum.
        // Then instead of overwriting values with a constant assignment a[i] = f,
        // try supporting addition: a[i] += f.
        pub enum AssignMin {}
        impl ArqSpec for AssignMin {
            type S = i64;
            type F = i64;
            fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
                a.min(b)
            }
            fn identity() -> Self::S {
                i64::max_value()
            }
            fn compose(&f: &Self::F, _: &Self::F) -> Self::F {
                f
            }
            fn apply(&f: &Self::F, _: &Self::S, _: i64) -> Self::S {
                f
            }
        }

        /// Range Sum Query, a slightly trickier classic application of ARQ.
        /// update(l, r, &f) sets all entries a[l..=r] to f.
        /// query(l, r) sums all the entries a[l..=r].
        ///
        /// # Panics
        ///
        /// Associated functions will panic on overflow.
        //
        // Note that while the `size` parameter seems necessary to satisfy the
        // Distributive Law, it is merely a convenience: in essence what we've done
        // is move to the product monoid of tuples (value, size_of_subtree).
        //
        // In mathematical jargon, we say that constant assignment f(a) = f is not an
        // endomorphism on (i64, +) because f(a+b) = f != 2*f = f(a) + f(b).
        // On the other hand, f((a, s)) = (f*s, s) is indeed an endomorphism on pairs
        // with vector addition: f((a, s) + (b, t)) = f((a+b, s+t)) = (f*(s+t), s+t)
        //                       = (f*s, s) + (f*t, t) = f((a,s)) + f((b,t)).
        pub enum AssignSum {}
        impl ArqSpec for AssignSum {
            type S = i64;
            type F = i64;
            fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
                a + b
            }
            fn identity() -> Self::S {
                0
            }
            fn compose(&f: &Self::F, _: &Self::F) -> Self::F {
                f
            }
            fn apply(&f: &Self::F, _: &Self::S, size: i64) -> Self::S {
                f * size
            }
        }

        /// Supply & Demand, based on https://codeforces.com/gym/102218/problem/F
        /// update(i, i, &(p, o)) increases supply by p and demand by o at time i.
        /// query(l, r) computes total supply and demand at times l to r, as well as
        //              how much of the supply is subsequently met by the demand.
        //
        // Note that the apply() operation is only correct when applied to leaf nodes.
        // Therefore, update() must only be used in "eager" mode, i.e., with l == r.
        // compose() should be unimplemented!() to prevent accidental "lazy" updates.
        pub enum SupplyDemand {}
        impl ArqSpec for SupplyDemand {
            type S = (i64, i64, i64); // production, orders, sales
            type F = (i64, i64);
            fn op((p1, o1, s1): &Self::S, (p2, o2, s2): &Self::S) -> Self::S {
                let extra = (p1 - s1).min(o2 - s2);
                (p1 + p2, o1 + o2, s1 + s2 + extra)
            }
            fn identity() -> Self::S {
                (0, 0, 0)
            }
            fn compose(_: &Self::F, _: &Self::F) -> Self::F {
                unimplemented!()
            }
            fn apply(&(p_add, o_add): &Self::F, &(p, o, _): &Self::S, s: i64) -> Self::S {
                assert_eq!(s, 1);
                let p = p + p_add;
                let o = o + o_add;
                (p, o, p.min(o))
            }
        }
    }


    pub mod static_arq {
        use super::specs::ArqSpec;

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
        }

        /// An example of binary search to find the first position whose element is negative.
        /// In this case, we use RMQ to locate the leftmost negative element.
        /// To ensure the existence of a valid root note (i == 1) from which to descend,
        /// the tree's size must be a power of two.
        pub fn first_negative(arq: &mut StaticArq<super::specs::AssignMin>) -> Option<usize> {
            assert!(arq.app.len().is_power_of_two());
            let mut p = 1;
            if arq.val[p] >= 0 {
                None
            } else {
                while p < arq.app.len() {
                    arq.push(p);
                    p <<= 1;
                    if arq.val[p] >= 0 {
                        p |= 1;
                    }
                }
                Some(p - arq.app.len())
            }
        }
    }
}

