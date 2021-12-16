use rustrithm::scanner::Scanner;
use seg_tree::static_arq::StaticArq;
use seg_tree::specs::ArqSpec;

struct ArqImpl;
impl ArqSpec for ArqImpl {
    type S = i64;
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        return a.min(b);
    }
    fn identity() -> Self::S {
        return i64::MAX/2000000;
    }
    fn compose(&f: &Self::F, g: &Self::F) -> Self::F {
        return f+g
    }
    fn apply(&f: &Self::F, a: &Self::S, _: i64) -> Self::S {
        return f+a;
    }
}


// https://atcoder.jp/contests/abc223/tasks/abc223_f
// segtree lazy segment tree
// parentheses checking
//
// - input:
// 8 8
// (()(()))
// 2 2 7
// 2 2 8
// 1 2 5
// 2 3 4
// 1 3 4
// 1 3 5
// 1 1 4
// 1 6 8
// - expected:
// Yes
// No
// No
fn main(){
    let sin = std::io::stdin();
    let mut scan = Scanner::new(sin.lock());
    let _n:usize = scan.token();
    let q:usize = scan.token();
    let st:String = scan.token();
    let mut v = Vec::<i64>::with_capacity(st.len());
    let mut sum: i64=0;
    for c in st.chars() {
        sum += if c == '(' { 1 } else { -1 };
        v.push(sum);
    }
    let mut seg = StaticArq::<ArqImpl>::new(&v);
    println!("{:?}", seg.show());
    println!("q: {:?}", seg.query(0,2));
    let mut chrs = st.chars().collect::<Vec<_>>();
    for _ in 0..q {
        let o:usize = scan.token();
        let l:usize = scan.token();
        let r:usize = scan.token();
        let l =l-1;
        let r =r-1;
        if o == 1 {
            let l_chr = chrs[l];
            let r_chr = chrs[r];
            if l_chr == '(' && r_chr == ')' {
                seg.update(l,r,&-2);
            } else if l_chr == ')' && r_chr == '(' {
                seg.update(l,r,&2);
            }
            chrs.swap(l, r);
            println!("{:?}", seg.show());
        } else {
            let pre = if l >= 1 { seg.query(l-1,l-1) } else { 0 };
            let min = seg.query(l,r);
            println!("pre,min: {:?},{:?}", pre,min);
            if pre == min {
                println!("Yes");
            } else {
                println!("No");
            }
        }
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
            // for debug
            pub fn show(self: &Self) -> &[T::S] {
                return &self.val[self.app.len()..];
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

