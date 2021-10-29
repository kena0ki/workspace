
use text_io::read;

// permutation
pub fn fn004(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let string:String = read!();
    let perm_len:usize = read!();
    let mut c = Calc::new(string.chars().collect(), perm_len);
    c.next_perm();
    return Ok(());
}

#[derive(Clone, Default, Debug)]
struct Params {
    pub rest: Vec<u8>,
    pub perm: Vec<u8>,
}

#[derive(Clone, Default, Debug)]
struct Calc<T: Clone> {
    pub chars: Vec<T>,
    pub work_chars: Vec<T>,
    pub k: usize,
}
impl <T: Clone + PartialEq> Calc<T> {
    fn new(chars: Vec<T>, k:usize) -> Self{
        let work_chars = chars.to_vec();
        return Calc {
            chars,
            work_chars,
            k,
        };
    }
    fn next_perm(self: &mut Self) -> usize {
        let mut cnt=0;
        for i in 0..self.chars.len() {
            for j in 0..i {
                let bk_chars = self.work_chars.to_vec();
                self.work_chars.swap(0,1);
                cnt+=1;
                for k in 0..j-1 {
                    for l in 1..k-1 {
                        self.work_chars.swap(l,l+1);
                        cnt+=1;
                    }
                    self.work_chars.swap(0,1);
                    cnt+=1;
                    if bk_chars == self.work_chars {
                        break;
                    }
                }
                self.work_chars.swap(0,j);
            }
        }
        return cnt;
    }
}
