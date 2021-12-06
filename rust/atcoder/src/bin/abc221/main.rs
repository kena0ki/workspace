
fn main() {}

#[cfg(test)]
mod e {
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
    fn _main(){
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
}


#[cfg(test)]
mod g_set {
    use std::collections::HashSet;
    use atcoder::util::bitarray::BitArray;

    #[test]
    fn test_g1() {
        let (yn, ans) = main(3,2,-2, &[1,2,3]);
        println!("{}", yn);
        println!("{}", ans);
        assert_eq!("Yes", yn);
        assert_eq!("LDR", ans);
    }

    #[test]
    fn test_g2() {
        let (yn, ans) = main(2,1,0, &[1,6]);
        println!("{}", yn);
        println!("{}", ans);
        assert_eq!("No", yn);
        assert_eq!("", ans);
    }

    #[test]
    fn test_g3() {
        let (yn, ans) = main(5,6,7, &[1,3,5,7,9]);
        println!("{}", yn);
        println!("{}", ans);
        assert_eq!("Yes", yn);
        assert_eq!("LRLUR", ans);
    }

    //    (x,y)  -> (x+y,x-y) -> ((x+y,x-y)+1)/2
    // --------------------------------------------
    // R: (1,0)  -> (1,1)     -> (1,1)
    // L: (-1,0) -> (-1,-1)   -> (0,0)
    // U: (0,1)  -> (1,-1)    -> (1,0)
    // D: (0,-1) -> (-1,1)    -> (0,1)
    fn main(n:usize, a:i64, b:i64, darr: &[usize]) -> (String, String) {
        let dsum = darr.iter().sum::<usize>();
        let x_goal = f_goal(a+b,dsum);
        let y_goal = f_goal(a-b,dsum);
        if x_goal.is_err() || y_goal.is_err() {
            return ("No".into(), "".into());
        }
        let x_goal=x_goal.unwrap();
        let y_goal=y_goal.unwrap();
        let dp = _f_dp_bit(n,darr);
        let x=_f_path_bit(n,&dp,darr,x_goal);
        let y=_f_path_bit(n,&dp,darr,y_goal);
        if x.is_err() || y.is_err() {
            return ("No".into(), "".into());
        }
        let x_path=x.unwrap();
        let y_path=y.unwrap();
        let mut ans = vec![0;n];
        let restore = [b'L', b'U', b'D', b'R'];
        for i in 0..n {
            let bits = y_path[i]<<1 | x_path[i];
            ans[i] = restore[bits as usize];
        }
        // println!("{:?}", x_path);
        // println!("{:?}", y_path);

        let ans = unsafe { std::str::from_utf8_unchecked(&*ans) };
        return ("Yes".into(), ans.into());
    }
    fn f_goal(ab:i64, dsum: usize) -> Result<usize, ()>{
        let tmp=ab+dsum as i64;
        if tmp < 0 || tmp & 1 == 1 {
            return Err(());
        }
        let goal = tmp as usize /2;
        // println!("goal: {:?}", goal);
        return Ok(goal);
    }
    fn _f_dp_set(n:usize, darr: &[usize]) -> Vec<HashSet<usize>>{
        let mut dp=Vec::with_capacity(n+1);
        let mut set = HashSet::<usize>::new();
        set.insert(0);
        dp.push(set);
        for i in 0..n {
            let mut next_set = dp[i].clone();
            next_set.reserve(next_set.len()*2);
            for s in dp[i].iter() {
                next_set.insert(s+darr[i]);
            }
            dp.push(next_set);
        }
        // println!("{:?}", dp);
        return dp;
    }
    fn _f_path_set(n:usize, dp: &Vec<HashSet<usize>>, darr: &[usize],goal:usize) -> Result<Vec<u8>,()>{
        if ! dp[n].contains(&goal) {
            return Err(());
        }
        let mut curr=goal as i64;
        let mut path = vec![0;n];
        for i in (0..n).rev() {
            if curr < 0 {
                break;
            }
            let prev = curr-darr[i] as i64;
            // println!("curr: {:?}", curr);
            if prev >= 0 && dp[i].contains(&(prev as usize)) {
                path[i] = 1;
                curr=prev;
            }
        }
        return Ok(path);
    }
    fn _f_dp_bit(n: usize, darr: &[usize]) -> Vec<BitArray>{
        let mut dp = Vec::with_capacity(n+1);
        dp.push(BitArray::from_u8slice_with_size(&[1], 3600001));
        for i in 0..n {
            let next = &(&dp[i] << darr[i]) | &dp[i];
            dp.push(next);
        }
        return dp;
    }
    fn _f_path_bit(n:usize, dp: &Vec<BitArray>, darr: &[usize],goal:usize) -> Result<Vec<u8>,()>{
        if ! dp[n].test_bit(goal) {
            return Err(());
        }
        let mut curr=goal as i64;
        let mut path = vec![0;n];
        for i in (0..n).rev() {
            if curr < 0 {
                break;
            }
            let prev = curr-darr[i] as i64;
            if prev >= 0 && dp[i].test_bit(prev as usize) {
                path[i] = 1;
                curr=prev;
            }
        }
        return Ok(path);
    }
}

