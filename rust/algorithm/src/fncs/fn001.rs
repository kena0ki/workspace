
// https://www.itmedia.co.jp/enterprise/articles/1003/06/news002.html
pub fn fn001(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let mut s = Summarizer::default();
    let state = State::default();
    s.max_width=args[2].parse::<usize>().unwrap();
    s.max_height=args[3].parse::<usize>().unwrap();
    s.debug_memo=vec![vec![0; s.max_height+1]; s.max_width+1];
    s.memo=vec![vec![0; s.max_height+1]; s.max_width+1];
    s.memoized_recursion(state);

    // println!("{:?}", s);
    println!("{:?}", s);
    println!("{:?}", s.summary);

  return Ok(());
}

#[derive(Clone, Copy, Default, Debug)]
struct State {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Default, Debug)]
struct Summarizer {
    pub max_height: usize,
    pub max_width: usize,
    pub summary: usize,
    pub debug_memo: Vec<Vec<usize>>,
    pub memo: Vec<Vec<usize>>,
}

impl Summarizer {
    // a(n,m) = a(n-1,m) + a(n,m-1)
    pub fn memoized_recursion(self: &mut Self, state: State) -> usize{
        let mut cnt=0;
        if state.width == self.max_width && state.height == self.max_height{
            self.memo[state.width][state.height]=1;
            self.summary+=1;
            return self.memo[state.width][state.height];
        }
        if state.height < self.max_height {
            let next_val = self.memo[state.width][state.height+1];
            if 0 == next_val {
                cnt+=self.memoized_recursion(State {
                    height: state.height+1,
                    ..state
                });
            } else {
                cnt+=next_val;
            }
        }
        if state.width < self.max_width {
            let next_val = self.memo[state.width+1][state.height];
            if 0 == next_val {
                cnt+=self.memoized_recursion(State {
                    width: state.width+1,
                    ..state
                });
            } else {
                cnt+=next_val;
            }
        }
        self.debug_memo[state.width][state.height]+=1;
        self.memo[state.width][state.height]=cnt;
        return self.memo[state.width][state.height];
    }
}

