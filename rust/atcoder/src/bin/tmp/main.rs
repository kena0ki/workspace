use std::{io::{BufRead, BufWriter, Write}, hash::Hash};
#[allow(unused)]
use std::{{collections::*, mem::swap},cmp::Reverse};

use std::collections::HashMap;

/// Represents a union of disjoint sets. Each set's elements are arranged in a
/// tree, whose root is the set's representative.
#[derive(Debug,Default,Clone)]
pub struct DisjointSets {
    parent: Vec<usize>,
    num_nodes: HashMap<usize,usize>,
}

impl DisjointSets {
    /// Initializes disjoint sets containing one element each.
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            num_nodes: HashMap::<_,_>::with_capacity(size),
        }
    }

    /// Finds the set's representative. Do path compression along the way to make
    /// future queries faster.
    pub fn find(&mut self, u: usize) -> usize {
        let pu = self.parent[u];
        if pu != u {
            self.parent[u] = self.find(pu);
        }
        self.parent[u]
    }

    /// Merges the sets containing u and v into a single set containing their
    /// union. Returns true if u and v were previously in different sets.
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let (pu, cu) = self.find_and_count(u);
        let (pv, cv) = self.find_and_count(v);
        let diff = pu != pv;
        if diff {
            self.num_nodes.remove(&pu);
            self.num_nodes.insert(pv, cv+cu);
        }
        self.parent[pu] = pv;
        diff
    }

    /// Returns the set's representative with the number of nodes in the set.
    pub fn find_and_count(&mut self, v:usize) -> (usize, usize) {
        let p = self.find(v);
        if let Some(&num) = self.num_nodes.get(&p) {
            return (p, num);
        }
        return (v, 1);
    }

    /// Returns the number of nodes in the set.
    pub fn count(&mut self, v:usize) -> usize {
        return self.find_and_count(v).1;
    }
}

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
test0,
test1,
test2,
test3,
tst4,
tst5,
tst6,
tst7,
|
"\
入力例 0 
Copy
7
5 3 2 4 7 1 6
5
1 5
5 6
1 2
2 3
1 7
",
"\
入力例 1 
Copy
6
5 3 2 4 6 1
4
1 5
5 6
1 2
2 3
出力例 1 
Copy
3
4 2 1
",
"\
入力例 2 
Copy
5
3 4 1 2 5
2
1 3
2 5
出力例 2 
Copy
-1
",
"\
入力例 3 
Copy
4
1 2 3 4
6
1 2
1 3
1 4
2 3
2 4
3 4
出力例 3 
Copy
0
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
    let mut vp = vec![0;n];
    for i in 0..n {
        let p = scan.token::<usize>()-1;
        vp[i]=p;
    }
    let m = scan.token::<usize>();
    let mut vva = vec![Vec::new();n];
    let mut ds = DisjointSets::new(n);
    for i in 0..m {
        let a = scan.token::<usize>()-1;
        let b = scan.token::<usize>()-1;
        if ds.find(a)==ds.find(b) {
            continue;
        }
        vva[a].push((i+1,b));
        vva[b].push((i+1,a));
        ds.merge(a,b);
    }
    let mut vis=vec![false;n];
    let mut ans = Vec::new();
    for i in 0..n {
        if vis[i] { continue; }
        if !f1(&vva,i,n,&mut vis, &mut vp, &mut ans) {
            writeln!(out, "{}", -1).ok();
            return;
        }
    }

    let m=ans.len();
    writeln!(out, "{}", m).ok();
    for (i,&a) in ans.iter().enumerate() {
        write!(out, "{}{}", a,b" \n"[(i+1)/m] as char).ok();
    }
}
fn f1(vva:&Vec<Vec<(usize,usize)>>, u:usize, p:usize, vis:&mut Vec<bool>, vp: &mut Vec<usize>,ans: &mut Vec<usize>) -> bool {
    vis[u]=true;
    for &(_,v) in &vva[u] {
        if v==p { continue; }
        if !f1(vva,v,u,vis,vp,ans) {
            return false;
        }
    }
    return f2(vva,u,usize::max_value(),u,vp,ans);
}
fn f2(vva:&Vec<Vec<(usize,usize)>>, u:usize, p:usize, s:usize, vp:&mut Vec<usize>,ans: &mut Vec<usize>) -> bool {
    if vp[u]==s { return true; }
    for &(e,v) in &vva[u] {
        if v==p { continue; }
        if f2(vva,v,u,s,vp,ans) {
            vp.swap(u,v);
            ans.push(e);
            logln!("{:?}",vp);
            return true;
        }
    }
    return false;
}

