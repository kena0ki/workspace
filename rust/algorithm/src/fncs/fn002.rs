
pub fn fn002(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let number = &args[2];
    let perm_len = &args[3].parse::<usize>().unwrap();
    let p = Params {
        rest: number.as_str().as_bytes().into(),
        perm: vec![],
    };
    let mut o = Obj {
        accum: vec![],
        rest_len: number.len() - perm_len,
    };
    o.rec(p);
    println!("{:?}", o.accum);
    println!("{:?}", o.accum.len());
    return Ok(());
}

#[derive(Clone, Default, Debug)]
struct Params {
    pub rest: Vec<u8>,
    pub perm: Vec<u8>,
}

#[derive(Clone, Default, Debug)]
struct Obj {
    pub accum: Vec<String>,
    pub rest_len: usize,
}
impl Obj {
    fn rec(self: &mut Self, p: Params) {
        if p.rest.len() == self.rest_len || p.rest.len() == 0 {
            self.accum.push(std::str::from_utf8(&p.perm).unwrap().into());
            return;
        }
        for i in 0..p.rest.len() {
            let mut clone = p.rest.to_vec();
            let item = clone.remove(i);
            let mut new_perm = p.perm.to_vec();
            new_perm.push(item);
            let next = Params {
                rest: clone,
                perm: new_perm,
            };
            self.rec(next);
        }
    }
}
