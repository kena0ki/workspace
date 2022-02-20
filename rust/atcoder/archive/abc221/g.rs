
use std::io::{BufRead, BufWriter, Write};
use rustrithm::scanner;

use std::collections::HashSet;
use rustrithm::bitarray::BitArray;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc221/tasks/abc221_g
//
//    (x,y)  -> (x+y,x-y) -> ((x+y,x-y)+1)/2
// --------------------------------------------
// R: (1,0)  -> (1,1)     -> (1,1)
// L: (-1,0) -> (-1,-1)   -> (0,0)
// U: (0,1)  -> (1,-1)    -> (1,0)
// D: (0,-1) -> (-1,1)    -> (0,1)
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n: usize = scan.token();
    let a: i64 = scan.token();
    let b: i64 = scan.token();
    let mut darr = Vec::<usize>::with_capacity(n);
    for _ in 0..n {
        darr.push(scan.token());
    }
    let darr = &darr[..];
    let dsum = darr.iter().sum::<usize>();
    let x_goal = f_goal(a+b,dsum);
    let y_goal = f_goal(a-b,dsum);
    if x_goal.is_err() || y_goal.is_err() {
        writeln!(out, "No").ok();
        return ;
    }
    let x_goal=x_goal.unwrap();
    let y_goal=y_goal.unwrap();
    let dp = _f_dp_bit(n,darr);
    let x=_f_path_bit(n,&dp,darr,x_goal);
    let y=_f_path_bit(n,&dp,darr,y_goal);
    if x.is_err() || y.is_err() {
        writeln!(out, "No").ok();
        return ;
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
    writeln!(out, "Yes").ok();
    writeln!(out, "{}", ans).ok();
    return ;
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
        //let mut next_set = dp[i].clone();
        let mut next_set = HashSet::<usize>::with_capacity(dp[i].len()*2);
        for &s in &dp[i] {
            next_set.insert(s);
        }
        //next_set.reserve(next_set.len()*2);
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
    //dp.push(BitArray::from_u8slice_with_size(&[1], 1_000_000));
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn abc221g_1() {
        let input: &[u8] = b"\
3 2 -2
1
2
3
";
        let expected = b"\
Yes
LDR
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, &output[..]);
    }

    #[test]
    fn abc221g_2() {
        let input: &[u8] = b"\
2 1 0
1
6
";
        let expected = b"\
No
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, &output[..]);
    }

    #[test]
    fn abc221g_3() {
        let input: &[u8] = b"\
5 6 7
1
3
5
7
9
";
        let expected = b"\
Yes
LRLUR
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, &output[..]);
    }

    use std::time::Instant;

    #[test]
    fn abc221g_bench() {
        let n = 200;
        let darr = (1..=200).collect::<Vec<_>>();
        let start = Instant::now();
        let _dp = _f_dp_set(n,&*darr);
        println!("dp.len(): {}", _dp[n-1].len());
        let duration = start.elapsed();

        println!("Time elapsed in _f_dp_bit() is: {:?}", duration);
    }
}

