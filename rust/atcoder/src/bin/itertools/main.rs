use itertools::Itertools;

fn main() {
    let nums = 0..25;
    let len = nums.len();
    // let perms = nums.permutations(len);
    let combi = nums.combinations(len/2);
    //itertools::assert_equal(perms, vec![
    //    vec![5, 6],
    //    vec![5, 7],
    //    vec![6, 5],
    //    vec![6, 7],
    //    vec![7, 5],
    //    vec![7, 6],
    //]);
    let mut cnt = 0;
    for _ in combi {
        cnt+=1;
    }
    println!("{}",cnt);
}
