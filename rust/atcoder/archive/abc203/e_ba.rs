// template

use std::io::{BufRead, BufWriter, Write};
use std::collections::*;

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

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, ShrAssign, Shr};


#[derive(Debug,Clone,Default)]
pub struct BitArray {
    bits: Vec<u128>,
    num_bits: usize,
    num_arr: usize,
}

impl BitArray {
    pub const BITS_PER_UNIT:usize = u128::BITS as usize;

    /// Initializes a bit array.
    pub fn new(size: usize) -> Self {
        let num_arr = size/Self::BITS_PER_UNIT + 1;
        return Self{
            bits: vec![0;num_arr],
            num_bits: size,
            num_arr,
        };
    }

    /// Initializes from a u8 slice.
    pub fn from_u8slice(bits: &[u8]) -> Self {
        return Self::from_u8slice_with_size(bits,bits.len());
    }

    /// Initializes from a u8 slice with a size.
    pub fn from_u8slice_with_size(bits: &[u8], size: usize) -> Self {
        Self::panic_if_out_of_input_range(size, bits.len());
        let mut new = Self::new(size);
        for i in 0..new.num_arr {
            let start = i*Self::BITS_PER_UNIT;
            let end = bits.len().min(start+Self::BITS_PER_UNIT);
            for j in start..end {
                new.bits[i] |= (bits[j] as u128) << (j-start);
            }
        }
        return new;
    }

    /// Gets the length of bits.
    pub fn len(&self) -> usize {
        return self.num_bits;
    }

    /// Sets the specified bit to true. Index is zero-based.
    pub fn set_bit_at(&mut self, at: usize) {
        self.panic_if_out_of_range(at);
        self.bits[at/Self::BITS_PER_UNIT] |= 1<<(at%Self::BITS_PER_UNIT);
    }

    /// Unsets the specified bit to false. Index is zero-based.
    pub fn unset_bit_at(&mut self, at: usize) {
        self.panic_if_out_of_range(at);
        self.bits[at/Self::BITS_PER_UNIT] &= 0<<(at%Self::BITS_PER_UNIT);
    }

    /// Sets the bits in the range from the offset to the offset + 128 using the u128 number. Index is zero-based.
    pub fn set_bits_with_u128(&mut self, num: u128, offset: usize) {
        self.panic_if_out_of_range(offset + Self::BITS_PER_UNIT);
        let q = offset / Self::BITS_PER_UNIT;
        let m = offset % Self::BITS_PER_UNIT;
        let s = Self::BITS_PER_UNIT - m;
        if m == 0 {
            self.bits[q] = num;
        } else {
            self.bits[q] = (self.bits[q] >> s) << s;
            self.bits[q] |= num << m;
            self.bits[q+1] = (self.bits[q+1] << m) >> m;
            self.bits[q+1] |= num >> s;
        }
    }

    /// Test whether the specified bit is true.
    pub fn test_bit(&self, at: usize) -> bool {
        self.panic_if_out_of_range(at);
        return self.bits[at/Self::BITS_PER_UNIT] & (1<<(at%Self::BITS_PER_UNIT)) > 0;
    }

    fn panic_if_out_of_input_range(num_bits: usize, at:usize) {
        if at > num_bits {
            panic!("Index {} out of range: {}.", at, num_bits);
        }
    }

    fn panic_if_out_of_range(&self, at:usize) {
        Self::panic_if_out_of_input_range(self.num_bits, at);
    }

    /// Converts the bit array to a binary representative string.
    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.num_bits);
        let b = self.bits[self.num_arr-1];
        let sub = Self::BITS_PER_UNIT - self.num_bits%Self::BITS_PER_UNIT;
        s.push_str(&format!("{:0128b}", b).as_str()[sub..]);
        for b in self.bits.iter().rev().skip(1) {
            s.push_str(format!("{:0128b}", b).as_str());
        }
        return s;
    }
}

impl BitAnd for &BitArray {
    type Output = BitArray;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut new = BitArray::new(self.num_bits);
        for i in 0..self.num_arr {
            new.bits[i] = self.bits[i] & rhs.bits[i];
        }
        return new;
    }
}
impl BitAndAssign<&Self> for BitArray {
    fn bitand_assign(&mut self, rhs: &Self) {
        let new = (&*self).bitand(&rhs);
        self.bits = new.bits;
    }
}
impl BitOr for &BitArray {
    type Output = BitArray;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut new = BitArray::new(self.num_bits);
        for i in 0..self.num_arr {
            new.bits[i] = self.bits[i] | rhs.bits[i];
        }
        return new;
    }
}
impl BitOrAssign<&Self> for BitArray {
    fn bitor_assign(&mut self, rhs: &Self) {
        let new = (&*self).bitor(&rhs);
        self.bits = new.bits;
    }
}
impl BitXor for &BitArray {
    type Output = BitArray;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new = BitArray::new(self.num_bits);
        for i in 0..self.num_arr {
            new.bits[i] = self.bits[i] ^ rhs.bits[i];
        }
        return new;
    }
}
impl BitXorAssign<&Self> for BitArray {
    fn bitxor_assign(&mut self, rhs: &Self) {
        let new = (&*self).bitxor(&rhs);
        self.bits = new.bits;
    }
}

impl Shl<usize> for &BitArray {
    type Output = BitArray;
    fn shl(self, rhs: usize) -> Self::Output {
        let mut new = BitArray::new(self.num_bits);
        if rhs != 0 {
            let shift = rhs / Self::Output::BITS_PER_UNIT;
            let offset = rhs % Self::Output::BITS_PER_UNIT;
            let sub_offset = Self::Output::BITS_PER_UNIT - offset;

            if offset == 0 {
                for i in (shift..self.num_arr).rev() {
                    new.bits[i] = self.bits[i - shift];
                }
            } else {
                for i in (shift+1..self.num_arr).rev() {
                    new.bits[i] = (self.bits[i - shift] << offset)
                         | (self.bits[i - shift - 1] >> sub_offset);
                }
                new.bits[shift] = self.bits[0] << offset;
            }
            //new.bits[0..shift].fill(0);
            let unused_range = Self::Output::BITS_PER_UNIT - self.num_bits%Self::Output::BITS_PER_UNIT;
            new.bits[self.num_arr-1] &= !0 >> unused_range;
        }
        return new;
    }
}
impl ShlAssign<usize> for BitArray {
    fn shl_assign(&mut self, rhs: usize) {
        let new = (&*self) << rhs;
        self.bits = new.bits;
    }
}

impl Shr<usize> for &BitArray {
    type Output = BitArray;
    fn shr(self, rhs: usize) -> Self::Output {
        let mut new = BitArray::new(self.num_bits);
        if rhs != 0 {
            let shift = rhs / Self::Output::BITS_PER_UNIT;
            let offset = rhs % Self::Output::BITS_PER_UNIT;
            let sub_offset = Self::Output::BITS_PER_UNIT - offset;

            if offset == 0 {
                for i in shift..self.num_arr {
                    new.bits[i-shift] = self.bits[i];
                }
            } else {
                for i in shift..self.num_arr-1 {
                    new.bits[i-shift] = (self.bits[i + 1] << sub_offset)
                         | (self.bits[i] >> offset);
                }
                new.bits[self.num_arr-shift-1] = self.bits[self.num_arr-1] >> offset;
            }
            new.bits[self.num_arr-(shift.max(1))..self.num_arr-1].fill(0);
        }
        return new;
    }
}
impl ShrAssign<usize> for BitArray {
    fn shr_assign(&mut self, rhs: usize) {
        let new = (&*self) >> rhs;
        self.bits = new.bits;
    }
}

impl <const N:usize> From<&[u8; N]> for BitArray {
    fn from(bits: &[u8; N]) -> Self {
        return Self::from_u8slice(bits);
    }
}
impl From<&[u8]> for BitArray {
    fn from(bits: &[u8]) -> Self {
        return Self::from_u8slice(bits);
    }
}

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc203/tasks/abc203_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut mpp = BTreeMap::<usize,BTreeSet<usize>>::new();
    for _ in 0..m {
        let x = scan.token::<usize>();
        let y = scan.token::<usize>();
        let p = mpp.entry(x).or_default();
        p.insert(y);
    }
    let mut now = BTreeSet::new();
    now.insert(n);
    for (_,ys) in mpp.iter() {
        logln!("ys:{:?}", ys);
        let mut add = vec![];
        for y in ys {
            if now.contains(&y.wrapping_sub(1)) || now.contains(&(y+1)){
                add.push(*y);
            }
        }
        for y in ys {
            now.remove(y);
        }
        for y in add {
            now.insert(y);
        }
        logln!("{:?}", now);
    }
    writeln!(out, "{}", now.len()).ok();
}

// https://atcoder.jp/contests/abc203/tasks/abc203_e
fn _solve_ba(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut mpp = BTreeMap::<usize,BitArray>::new();
    for _ in 0..m {
        let x = scan.token::<usize>();
        let y = scan.token::<usize>();
        let p = mpp.entry(x).or_insert(BitArray::new(2*n));
        p.set_bit_at(y);
    }
    let mut now = BitArray::new(2*n);
    now.set_bit_at(n);
    for (_,ys) in mpp.iter() {
        let add1 = &(&now >> 1) & ys;
        let add2 = &(&now << 1) & ys;
        now = &now ^ ys;
        now = &now | &add1;
        now = &now | &add2;
        logln!("{:?}", now.to_string());
    }
    let mut ans = 0;
    for i in 0..2*n {
        if now.test_bit(i) { ans += 1; }
    }
    writeln!(out, "{}a", ans).ok();
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
}

#[cfg(test)]
mod abc203e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
2 4
1 1
1 2
2 0
4 2
";
        let expected = "\
3
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test2() {
        let input: &[u8] = b"\
1 1
1 1
";
        let expected = "\
0
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
