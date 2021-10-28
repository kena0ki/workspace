
use text_io::read;

// https://www.itmedia.co.jp/enterprise/articles/1003/06/news002.html
pub fn fn003(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let num: String = read!();
    let num_sel: usize = read!();

    let state = State::default();
    let mut calc = Calc::default();
    calc.chars=num.chars().collect();
    calc.num_sel=num_sel;
    calc.num_rest=calc.chars.len()-calc.num_sel;
    calc.debug_memo=vec![vec![0; calc.num_sel+1]; calc.num_rest+1];
    calc.memo=vec![vec![vec![]; calc.num_sel+1]; calc.num_rest+1];
    calc.memoized_recursion(state);

    // println!("{:?}", s);
    println!("{:?}", calc);

  return Ok(());
}

#[derive(Clone, Copy, Default, Debug)]
struct State {
    pub x: usize,
    pub y: usize,
}

type Combinations = Vec<String>;

#[derive(Clone, Default, Debug)]
struct Calc {
    pub chars: Vec<char>,
    pub num_sel: usize,
    pub num_rest: usize,
    pub debug_memo: Vec<Vec<usize>>,
    pub memo: Vec<Vec<Combinations>>,
}

impl Calc {
    // a(n,m) = a(n-1,m) + a(n,m-1)
    pub fn memoized_recursion(self: &mut Self, state: State) -> Combinations{
        let mut chrs=vec![];
        if state.x == self.num_rest && state.y == self.num_sel{
            // let chr = self.chars[(state.x+state.y)-1];
            // self.memo[state.x][state.y].push(chr.into());
            // return self.memo[state.x][state.y];
            return vec![];
        }
        if state.y < self.num_sel {
            let mut next_val = self.memo[state.x][state.y+1].to_vec();
            if 0 == next_val.len() {
                next_val = self.memoized_recursion(State {
                    y: state.y+1,
                    ..state
                });
            }
            if 0 == next_val.len() {
                let chr = self.chars[(state.x+state.y)];
                chrs.push(chr.to_string());
            } else {
                for st in next_val {
                    let chr = self.chars[(state.x+state.y)];
                    chrs.push(st + chr.to_string().as_str());
                }
            }
        }
        if state.x < self.num_rest {
            let mut next_val = self.memo[state.x+1][state.y].to_vec();
            if 0 == next_val.len() {
                next_val = self.memoized_recursion(State {
                    x: state.x+1,
                    ..state
                });
            }
            for st in next_val {
                chrs.push(st);
            }
        }
        self.debug_memo[state.x][state.y]+=1;
        self.memo[state.x][state.y]=chrs;
        return self.memo[state.x][state.y].to_vec();
    }
}

