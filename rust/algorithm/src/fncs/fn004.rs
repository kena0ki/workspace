
use text_io::read;

// permutation
pub fn fn004(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let string:String = read!();
    let perm_len:usize = read!();
    let mut c = Calc::new(string.chars().collect(), perm_len);
    c.next_perm(perm_len);
    return Ok(());
}

#[derive(Clone, Default, Debug)]
struct _Params {
    pub depth: usize,
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
    fn next_perm(self: &mut Self, depth: usize) -> usize {
        let mut cnt=0;

        return cnt;
    }
}
