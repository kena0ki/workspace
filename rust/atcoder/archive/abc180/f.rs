// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

use std::convert::TryInto;
pub struct Factorial<const M:u64> {
    fact: Vec<ModU64<M>>,
    ifact: Vec<ModU64<M>>,
}
impl <const M:u64> Factorial<M>{
    pub fn new(n:usize) -> Self{
        let zero = ModU64::<M>::new(0);
        let mut fact = Vec::<ModU64<M>>::with_capacity(n+1);
        fact.push(zero+1);
        for i in 1..=n {
            fact.push(fact[i-1] * (i) as u64);
        }
        let mut ifact = vec![zero+1;n+1];
        ifact[n] = fact[n].inv();
        for i in (3..=n).rev() {
            ifact[i-1] = ifact[i] * i as u64;
        }
        return Self { fact, ifact};
    }

    pub fn perm <T: TryInto<usize>>(&self,n:T,k:T) -> ModU64<M> {
        let n = n.try_into().ok().expect("Unable to cast n to usize");
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        if n < k {
            return ModU64::<M>::new(0);
        }
        return self.fact[n]*self.ifact[n-k];
    }

    pub fn combin <T: TryInto<usize>>(&self,n:T,k:T) -> ModU64<M> {
        let n = n.try_into().ok().expect("Unable to cast n to usize");
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        if n < k {
            return ModU64::<M>::new(0);
        }
        return self.fact[n]*self.ifact[k]*self.ifact[n-k];
    }

    pub fn fact<T: TryInto<usize>>(&self,k:T) -> &ModU64<M> {
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        return &self.fact[k];
    }
    pub fn ifact<T: TryInto<usize>>(&self,k:T) -> &ModU64<M> {
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        return &self.ifact[k];
    }
}

impl <const M:u64> Default for Factorial<M>{
    fn default() -> Self {
        return Self::new(1_000_000);
    }
}

pub struct Permutations<T> {
    items: Vec<T>,
    swaps: Vec<usize>,
    i: usize,
}

impl <T:Clone> Permutations<T> {
    pub fn new(items: Vec<T>) -> Permutations<T> {
        let swaps = vec![0; items.len()];
        Permutations { items, swaps, i: 0 }
    }
}

impl <T:Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = 1;
            return Some(self.items.clone());
        }
        loop {
            if self.i >= self.items.len() {
                return None;
            }
            if self.swaps[self.i] < self.i {
                break;
            }
            self.swaps[self.i] = 0;
            self.i += 1;
        }
        self.items.swap(self.i, (self.i & 1) * self.swaps[self.i]);
        self.swaps[self.i] += 1;
        self.i = 1;
        return Some(self.items.clone());
    }
}

use std::ops::{Add,Sub,Mul,Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt;

pub const MOD998244353:u64 = 998244353;
pub const MOD1000000007:u64 = 1000000007;

pub const ZERO_MOD998244353:ModU64<MOD998244353> = ModU64::<MOD998244353>::new(0);
pub const ZERO_MOD1000000007:ModU64<MOD1000000007> = ModU64::<MOD1000000007>::new(0);
pub const fn pow(val:u64, mut power: u64, modulus:u64) -> u64 {
    let mut square = val;
    let mut ret = 1;
    while 0 < power {
        if (power & 1) == 1{
            ret *= square;
            ret %= modulus;
        }
        square *= square;
        square %= modulus;
        power >>= 1;
    }
    return ret;
}
pub const fn inv(val: u64, modulus:u64) -> u64 {
    return pow(val, modulus - 2, modulus);
}

/// Represents a mod N number.
///
/// # Example
/// ```
/// use rustrithm::math::num::ModU64;
/// // modulus = 5
/// let m1 = ModU64::<5>::new(2);
/// let m2 = m1.sibling(4);
///
/// assert_eq!(m1.sibling(1), m1+m2);
/// assert_eq!(m1.sibling(3), 9*m1);
/// assert_eq!(m1.sibling(4), ModU64::<5>::from_i64(-1));
///
/// ```
#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct ModU64<const N:u64>{
    modulus: u64,
    val: u64,
}
impl <const N:u64> ModU64<N> {
    /// Creates a new instance.
    pub const fn new(val: u64) -> Self {
        return Self{ modulus: N, val: val%N };
    }
    /// Creates a new instance from i64.
    /// The result value is guaranteed to be positive by adding the modulus if the given value is negative.
    pub const fn from_i64(val: i64) -> Self {
        let val = val%N as i64;
        let val = if val < 0 { val + N as i64 } else { val };
        return Self { val: val as u64, modulus: N };
    }
    /// Creates a new instance using the same modulus of the current instance.
    pub const fn sibling(self: &Self, val:u64) -> Self {
        return Self {
            modulus: self.modulus,
            val: val%self.modulus,
        };
    }
    /// Gets the underlying value as u64.
    pub const fn val(&self) -> u64 {
        return self.val;
    }
    /// Gets the power of this value.
    pub const fn pow(&self, power: u64) -> Self{
        return Self {
            val:pow(self.val, power, self.modulus),
            modulus: self.modulus,
        };
    }
    /// Gets the inverse of this value.
    pub const fn inv(&self) -> Self {
        return self.pow(self.modulus - 2);
    }
    const fn add_u64(&self, mut lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        lhs += rhs;
        if lhs >= self.modulus {
            lhs -= self.modulus;
        }
        return lhs;
    }
    const fn sub_u64(&self, mut lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        if lhs < rhs {
            lhs += self.modulus - rhs;
        } else {
            lhs -= rhs;
        }
        return lhs;
    }
    const fn mul_u64(&self, lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        return (lhs * rhs) % self.modulus;
    }
    // a^(-1) ≡ a^(p-2)  (mod p)  where p is prime
    // https://en.wikipedia.org/wiki/Modular_arithmetic#Properties
    const fn div_u64(&self, mut lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
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

impl <const N:u64> From<ModU64<N>> for u64 {
    fn from(mu: ModU64<N>) -> Self {
        return mu.val;
    }
}

impl <const N:u64> fmt::Display for ModU64<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}",self.val);
    }
}

impl <const N:u64> fmt::Debug for ModU64<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}",self.val);
    }
}

impl <const N:u64> Default for ModU64<N> {
    fn default() -> Self {
        return Self { modulus: N, val:0 };
    }
}

macro_rules! assign_binop {
    (impl $imp:ident, $method:ident for $t:ident, $internal_method:ident) => {
        impl <const N:u64> $imp for $t<N> {
            #[inline]
            fn $method(&mut self, rhs: Self) {
                self.val = self.$internal_method(self.val, rhs.val);
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ident, $u:ty, $internal_method:ident) => {
        impl <const N:u64> $imp<$u> for $t<N> {
            #[inline]
            fn $method(&mut self, rhs: $u) {
                self.val = self.$internal_method(self.val, rhs%self.modulus);
            }
        }
    };
}

assign_binop!(impl AddAssign, add_assign for ModU64, add_u64);
assign_binop!(impl SubAssign, sub_assign for ModU64, sub_u64);
assign_binop!(impl MulAssign, mul_assign for ModU64, mul_u64);
assign_binop!(impl DivAssign, div_assign for ModU64, div_u64);
assign_binop!(impl AddAssign, add_assign for ModU64, u64, add_u64);
assign_binop!(impl SubAssign, sub_assign for ModU64, u64, sub_u64);
assign_binop!(impl MulAssign, mul_assign for ModU64, u64, mul_u64);
assign_binop!(impl DivAssign, div_assign for ModU64, u64, div_u64);

macro_rules! binop {
    (impl $imp:ident, $method:ident for $t:ident, $internal_method:ident) => {
        impl <const N:u64> $imp for $t<N> {
            type Output = Self;
            #[inline]
            fn $method(mut self: Self, rhs: Self) -> Self {
                self.val = self.$internal_method(self.val, rhs.val);
                return self;
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ident, $u:ty, $internal_method:ident) => {
        impl <const N:u64> $imp<$u> for $t<N> {
            type Output = Self;
            #[inline]
            fn $method(mut self: Self, rhs: $u) -> Self {
                self.val = self.$internal_method(self.val, rhs%self.modulus);
                return self;
            }
        }
        impl <const N:u64> $imp<$t<N>> for $u {
            type Output = $t<N>;
            #[inline]
            fn $method(self: Self, mut rhs: $t<N>) -> $t<N> {
                rhs.val = rhs.$internal_method(self%rhs.modulus, rhs.val);
                return rhs;
            }
        }
    };
}
binop!(impl Add, add for ModU64, add_u64);
binop!(impl Sub, sub for ModU64, sub_u64);
binop!(impl Mul, mul for ModU64, mul_u64);
binop!(impl Div, div for ModU64, div_u64);
binop!(impl Add, add for ModU64, u64, add_u64);
binop!(impl Sub, sub for ModU64, u64, sub_u64);
binop!(impl Mul, mul for ModU64, u64, mul_u64);
binop!(impl Div, div for ModU64, u64, div_u64);


// https://stackoverflow.com/questions/38811387/how-to-implement-idiomatic-operator-overloading-for-values-and-references-in-rus/38815035#38815035
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ident) => {
        impl<'a, const N:u64> $imp<$t<N>> for &'a $t<N> {
            type Output = <$t<N> as $imp<$t<N>>>::Output;

            #[inline]
            fn $method(self, other: $t<N>) -> <$t<N> as $imp<$t<N>>>::Output {
                $imp::$method(*self, other)
            }
        }
        impl<'a, const N:u64> $imp<&'a $t<N>> for $t<N> {
            type Output = <$t<N> as $imp<$t<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a $t<N>) -> <$t<N> as $imp<$t<N>>>::Output {
                $imp::$method(self, *other)
            }
        }
        impl<'a, 'b, const N:u64> $imp<&'a $t<N>> for &'b $t<N> {
            type Output = <$t<N> as $imp<$t<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a $t<N>) -> <$t<N> as $imp<$t<N>>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ident, $u:ty) => {
        impl<'a, const N:u64> $imp<$u> for &'a $t<N> {
            type Output = <$t<N> as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t<N> as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }
        impl<'a, const N:u64> $imp<&'a $u> for $t<N> {
            type Output = <$t<N> as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t<N> as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }
        impl<'a, 'b, const N:u64> $imp<&'a $u> for &'b $t<N> {
            type Output = <$t<N> as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t<N> as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

forward_ref_binop! {impl Add, add for ModU64}
forward_ref_binop! {impl Sub, sub for ModU64}
forward_ref_binop! {impl Mul, mul for ModU64}
forward_ref_binop! {impl Div, div for ModU64}
forward_ref_binop! {impl Add, add for ModU64, u64}
forward_ref_binop! {impl Sub, sub for ModU64, u64}
forward_ref_binop! {impl Mul, mul for ModU64, u64}
forward_ref_binop! {impl Div, div for ModU64, u64}


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
3 2 3
" , "\
3
");

    test_macro!(test2, b"\
4 3 2
" , "\
6
");

    test_macro!(test3, b"\
300 290 140
" , "\
211917445
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let l = scan.token::<usize>();
    let zero = ZERO_MOD1000000007;
    let f = Factorial::<MOD1000000007>::new(300);
    let mut c = vec![vec![zero;n+1];n+1];
    c[0][0]=zero+1;
    for i in 0..n { for j in 0..n {
        c[i+1][j] = c[i+1][j]+c[i][j];
        c[i+1][j+1] = c[i+1][j+1]+c[i][j];
    }}
    //logln!("{:?}",c);
    let mut path = vec![zero;n+1];
    path[1]=zero+1; path[2]=zero+1;
    for i in 3..n+1 {
        //path[i] = path[i-1] * i as u64;
        path[i] = f.fact[i]/2;
    }
    //logln!("{:?}",path);
    let mut cycle = vec![zero;n+1];
    for i in 2..n+1 {
        cycle[i] = path[i-1];
    }
    //logln!("{:?}",cycle);
    let itwo = (zero+2).inv();
    let g = |l:usize| {
        let mut dp = vec![vec![zero;m+2];n+1];
        dp[0][0] = zero+1;
        for i in 0..n { for j in 0..m+1 {
            for k in 1..l+1 {
                let ni = i+k;
                let nj = j+k-1;
                if ni > n || nj > m { break; }
                //let mul = c[n-i-1][k-1]*path[k];
                let mul = f.combin(n-i-1,k-1)*f.fact[k];
                let mul = if k>=2 { mul*itwo } else { mul };
                //logln!("{},{},{},{}",i,j,k,mul);
                dp[ni][nj] = dp[ni][nj] + dp[i][j]*mul;
                if k<=1 { continue; }
                let ni = i+k;
                let nj = j+k;
                if ni > n || nj > m { break; }
                let mul = c[n-i-1][k-1]*f.fact[k-1];
                let mul = if k>=3 { mul*itwo } else { mul };
                dp[ni][nj] = dp[ni][nj] + dp[i][j]*mul;
            }
        } }
        //logln!("{:?}",dp);
        return dp[n][m];
    };
    let a = g(l);
    let b = g(l-1);
    logln!("{},{}", a,b);
    let ans = a-b;
    writeln!(out, "{}",ans).ok();
}

