use std::io::{BufRead, BufWriter, Write};
use rustrithm::scanner;

use rustrithm::range_query::static_arq::StaticArq;
use rustrithm::range_query::specs::ArqSpec;
use rustrithm::util;
use rustrithm::math::modulo::ModU64;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

const MOD: u64 = 998244353;

struct ArqImpl;
impl ArqSpec for ArqImpl {
    type S = ModU64<MOD>;
    type F = ModU64<MOD>;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        return a+b;
    }
    fn identity() -> Self::S {
        return ModU64::<MOD>::new(0);
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
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n:usize = scan.token();
    let mut a = vec![0usize;n];
    for i in 0..n {
        a[i]=scan.token();
    }
    let (arr,m) = util::coord_cmp(&mut a);
    let f = ModU64::<MOD>::new(0);
    let v = vec![f; m];
    let mut seg = StaticArq::<ArqImpl>::new(&v);
    let mut ans = f;
    let mut m2 = f.sibling(2);
    let mut m2_inv = m2.inv();
    for i in 1..n {
        seg.update(arr[i-1], arr[i-1], &m2_inv);
        let sum = seg.query(0, arr[i]) * m2;
        ans = ans + sum;
        m2*=2;
        m2_inv/=2;
    }
    writeln!(out, "{}", ans).ok();
}

#[cfg(test)]
mod abc221e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
10
198495780 28463047 859606611 212983738 946249513 789612890 782044670 700201033 367981604 302538501
";
        let expected = "\
830
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}

