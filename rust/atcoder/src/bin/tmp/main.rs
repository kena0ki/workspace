
use std::{io::{BufRead, BufWriter, Write}, hash::Hash};
#[allow(unused)]
use std::{{collections::*, mem::swap},cmp::Reverse};

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

trait MapX<K,V:Default> {
    fn update_with<F: FnOnce(&mut V) -> V>(&mut self, key:K, get_new_value:F);
}
macro_rules! update_with { () => {
    fn update_with<F: FnOnce(&mut V) -> V>(&mut self, key:K, get_new_value:F) {
        let v = self.entry(key).or_default();
        let nv = get_new_value(v);
        *v = nv;
    }
} }
impl <K:Ord,V:Default> MapX<K,V> for BTreeMap<K,V> {
    update_with!();
}
impl <K:Hash+Eq,V:Default> MapX<K,V> for HashMap<K,V> {
    update_with!();
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
}

#[cfg(test)]
mod abc999x {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, | $input:expr,) => {
            #[test]
            fn $name() {
                let vi = $input.split("\n").collect::<Vec<_>>();
                let vi = &vi[1..vi.len()-1];
                let mut inp = String::new();
                let mut cnt=0;
                for &line in vi {
                    if line.starts_with("出力例") { break; }
                    cnt+=1;
                    if line.starts_with("入力例") || line.starts_with("Copy") { continue; }
                    inp+=line; inp+="\n";
                }
                let mut exp = String::new();
                for &line in &vi[cnt..] {
                    if line.starts_with("出力例") || line.starts_with("Copy") { continue; }
                    exp+=line; exp+="\n";
                }
                let output = &mut Vec::new();
                let scan = &mut Scanner::new(inp.as_bytes());
                solve(scan, output);
                assert_eq!(exp, std::str::from_utf8(output).unwrap());
            }
        };
        ($name:ident, $($n:ident),*, | $input:expr, $($i:expr),*,) => {
            test_macro!($name, | $input,);
            test_macro!($($n),*, | $($i),*,);
        };
    }

    test_macro!(
tst0,
test1,
test2,
tst3,
tst4,
tst5,
tst6,
tst7,
|
"\
入力例 0 
",
"\
入力例 1 
Copy
4 1
1 2
2 3
3 4
出力例 1 
Copy
6
",
"\
入力例 2 
Copy
8 3
1 2
4 6
6 7
3 2
2 4
4 5
8 6
出力例 2 
Copy
9
",
"\
入力例 3 
",
"\
入力例 4 
",
"\
入力例 5 
",
"\
入力例 6 
",
"\
入力例 7 
",
);

}

fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut vva = vec![Vec::new();n];
    for _ in 0..n-1 {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        vva[u].push(v);
        vva[v].push(u);
    }

    let (dp0,dp1) = f(&vva,0,n,k);
    let ans = dp0[k]+dp1[k];
    writeln!(out, "{}", ans).ok();

    fn f(vva: &Vec<Vec<usize>>, u: usize, p:usize, k:usize) -> (Vec<usize>,Vec<usize>) {
        let mut dp0 = vec![0;k+1];
        dp0[0]=1;
        let mut vtmp0 = Vec::<Vec<usize>>::new();
        let mut vtmp1 = Vec::<Vec<usize>>::new();
        for &v in &vva[u] {
            if v == p { continue; }
            let (dpv0,dpv1) = f(vva,v,u,k);
            let mut tmp0 = dp0.clone();
            let mut tmp1 = dp0.clone();
            for i in (0..k+1).rev() { for j in (0..k+1).rev() {
                let nk = i+j;
                if nk > k || nk==0 { continue; }
                let (v0,v1) = (dpv0[j],dpv1[j]);
                dp0[nk] += dp0[i]*(v0+v1);
                tmp0[nk] += tmp0[i]*(v0);
                tmp1[nk] += tmp1[i]*(v1);
                for t in 0..vtmp0.len() {
                    vtmp0[t][nk] += vtmp0[t][i]*(v0+v1);
                }
                for t in 0..vtmp0.len() {
                    vtmp1[t][nk] += vtmp1[t][i]*(v0+v1);
                }
            }}
            vtmp0.push(tmp0);
            vtmp1.push(tmp1);
        }
        let mut dp1 = vec![0;k+1];
        for i in 0..vtmp0.len() {
            for j in 0..k+1 {
                if j>= 1 {
                    dp1[j] = vtmp0[i][j-1] + vtmp1[i][j];
                } else {
                    dp1[j] = vtmp1[i][j];
                }
            }
        }
        logln!("{},{:?},{:?},{:?},{:?}",u,dp0,dp1,vtmp0,vtmp1);


        return (dp0,dp1);
    }
}

