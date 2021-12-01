use text_io::read;
use atcoder::range_query::segtree::static_arq::StaticArq;
use atcoder::range_query::segtree::specs::ArqSpec;
use atcoder::util;
use atcoder::math::modulo::{ModUsize,ModUsizeFactory};

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
// segment tree
// fenwick
//
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
    let (arr,m) = util::coord_cmp(&mut a);
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

