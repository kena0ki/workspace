use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, ShrAssign, Shr};


#[derive(Debug,Clone)]
pub struct BitArray {
    bits: Vec<u128>,
    num_bits: usize,
    num_arr: usize,
}

impl BitArray {
    pub const BITS_PER_UNIT:usize = u128::BITS as usize;

    pub fn new(size: usize) -> Self {
        let num_arr = size/Self::BITS_PER_UNIT + 1;
        return Self{
            bits: vec![0;num_arr],
            num_bits: size,
            num_arr,
        };
    }

    pub fn len(&self) -> usize {
        return self.num_bits;
    }

    /// Sets the specified bit to true. Index is zero-based.
    pub fn set_bit(&mut self, at: usize) {
        self.panic_if_out_of_range(at);
        self.bits[at/Self::BITS_PER_UNIT] |= 1<<(at%Self::BITS_PER_UNIT);
    }

    /// Sets the specified bit to false. Index is zero-based.
    pub fn clear_bit(&mut self, at: usize) {
        self.panic_if_out_of_range(at);
        self.bits[at/Self::BITS_PER_UNIT] &= 0<<(at%Self::BITS_PER_UNIT);
    }

    pub fn set_bits_by_num(&mut self, num: u128, offset: usize) {
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

    pub fn test_bit(&mut self, at: usize) -> bool {
        self.panic_if_out_of_range(at);
        return self.bits[at/Self::BITS_PER_UNIT] & (1<<(at%Self::BITS_PER_UNIT)) > 0;
    }

    fn panic_if_out_of_range(&self, at:usize) {
        if at > self.num_bits {
            panic!("Index {} out of range: {}.", at, self.num_bits);
        }
    }

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

impl BitAnd for BitArray {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.bitand_assign(rhs);
        return self;
    }
}
impl BitAndAssign for BitArray {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..self.num_arr {
            self.bits[i] &= rhs.bits[i];
        }
    }
}
impl BitOr for BitArray {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.bitor_assign(rhs);
        return self;
    }
}
impl BitOrAssign for BitArray {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..self.num_arr {
            self.bits[i] |= rhs.bits[i];
        }
    }
}
impl BitXor for BitArray {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self.bitxor_assign(rhs);
        return self;
    }
}
impl BitXorAssign for BitArray {
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..self.num_arr {
            self.bits[i] ^= rhs.bits[i];
        }
    }
}

impl Shl<usize> for BitArray {
    type Output = Self;
    fn shl(mut self, rhs: usize) -> Self::Output {
        self.shl_assign(rhs);
        return self;
    }
}
impl ShlAssign<usize> for BitArray {
    fn shl_assign(&mut self, rhs: usize) {
        //let mut new = Self::new(self.num_bits);
        if rhs != 0 {
            let shift = rhs / Self::BITS_PER_UNIT;
            let offset = rhs % Self::BITS_PER_UNIT;
            let sub_offset = Self::BITS_PER_UNIT - offset;

            if offset == 0 {
                for i in (shift..self.num_arr).rev() {
                    self.bits[i] = self.bits[i - shift];
                }
            } else {
                for i in (shift+1..self.num_arr).rev() {
                    self.bits[i] = (self.bits[i - shift] << offset)
                         | (self.bits[i - shift - 1] >> sub_offset);
                }
                self.bits[shift] = self.bits[0] << offset;
            }
            self.bits[0..shift].fill(0);
            let unused_range = Self::BITS_PER_UNIT - self.num_bits%Self::BITS_PER_UNIT;
            self.bits[self.num_arr-1] &= !0 >> unused_range;
        }
    }
}

impl Shr<usize> for BitArray {
    type Output = Self;
    fn shr(mut self, rhs: usize) -> Self::Output {
        self.shr_assign(rhs);
        return self;
    }
}
impl ShrAssign<usize> for BitArray {
    fn shr_assign(&mut self, rhs: usize) {
        if rhs != 0 {
            let shift = rhs / Self::BITS_PER_UNIT;
            let offset = rhs % Self::BITS_PER_UNIT;
            let sub_offset = Self::BITS_PER_UNIT - offset;

            if offset == 0 {
                for i in shift..self.num_arr {
                    self.bits[i-shift] = self.bits[i];
                }
            } else {
                for i in shift..self.num_arr-1 {
                    self.bits[i-shift] = (self.bits[i + 1] << sub_offset)
                         | (self.bits[i] >> offset);
                }
                self.bits[self.num_arr-shift-1] = self.bits[self.num_arr-1] >> offset;
            }
            self.bits[self.num_arr-(shift.max(1))..self.num_arr-1].fill(0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn barr_bitor() {
        let mut left = BitArray::new(200);
        left.set_bits_by_num(!0 - (1<<2) - (1<<80), 0);
        let mut right = BitArray::new(200);
        right.set_bits_by_num(!0 - (1<<2) - (1<<80), 60);
        let barr = left | right;
        let expected = "00000000000011111111111111111111111111111111111111111111111011111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111011";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_bitand() {
        let mut left = BitArray::new(200);
        left.set_bits_by_num(!0 - (1<<2) - (1<<80), 30);
        let mut right = BitArray::new(200);
        right.set_bits_by_num(!0 - (1<<2) - (1<<80), 60);
        let barr = left & right;
        let expected = "00000000000000000000000000000000000000000011111111111111111011111111111111111111111111111011111111111111111111111111111111111111111111111011000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_bitxor() {
        let mut left = BitArray::new(200);
        left.set_bits_by_num(!0 - (1<<2) - (1<<80), 30);
        let mut right = BitArray::new(200);
        right.set_bits_by_num(!0 - (1<<2) - (1<<80), 60);
        let barr = left ^ right;
        let expected = "00000000000011111111111111111111111111111100000000000000000100000000000000000000000000000100000000000000000000000000000000000000000000000100111111111111111111111111111011000000000000000000000000000000";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_shift_left() {
        let mut barr = BitArray::new(200);
        barr.set_bits_by_num(!0 - (1<<2) - (1<<80), 10);
        barr = barr << 100;
        let expected = "11111111101111111111111111111111111111111111111111111111111111111111111111111111111111101100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_shift_left_assign() {
        let mut barr = BitArray::new(200);
        barr.set_bits_by_num(!0 - (1<<2) - (1<<80), 10);
        barr <<= 50;
        let expected = "00000000000011111111111111111111111111111111111111111111111011111111111111111111111111111111111111111111111111111111111111111111111111111011000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_shift_right() {
        let mut barr = BitArray::new(200);
        barr.set_bits_by_num(!0 - (1<<2) - (1<<80), 72);
        barr = barr >> 100;
        let expected = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001111111111111111111111111111111111111111111111101111111111111111111111111111111111111111111111111111";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_shift_right_assign() {
        let mut barr = BitArray::new(200);
        barr.set_bits_by_num(!0 - (1<<2) - (1<<80), 72);
        barr >>= 50;
        let expected = "00000000000000000000000000000000000000000000000000111111111111111111111111111111111111111111111110111111111111111111111111111111111111111111111111111111111111111111111111111110110000000000000000000000";
        assert_eq!(expected, barr.to_string());
    }
}

