use text_io::read;

// calculate combination using DP (see fn001)
pub fn fn005(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let n: usize = read!();
    let k: usize = read!();
    let mut dp = vec![vec![0usize;n-k+2]; k+2];
    dp[1][1]=1;
    let mut cnt=0;
    for i in 1..k+2 {
        for j in 1..n-k+2 {
            dp[i][j] += dp[i][j-1] + dp[i-1][j];
            cnt+=1;
        }
    }
    //println!("{:?}", dp);
    println!("cnt: {}", cnt);
    println!("{}", dp[k+1][n-k+1]);
    return Ok(());
}
