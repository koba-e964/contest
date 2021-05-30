#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

type State = (usize, usize, usize, u8);

fn dfs(s: State, board: &[Vec<char>],
       memo: &mut HashMap<State, bool>, vis: &mut HashSet<State>)
       -> bool {
    let h = board.len();
    let w = board[0].len();
    let dir = [(0, 1), (0, w - 1), (1, 0), (h - 1, 0)];
    if let Some(&v) = memo.get(&s) {
        return v;
    }
    if vis.contains(&s) {
        memo.insert(s, false);
        return false;
    }
    vis.insert(s);
    let (i, j, mut d, mut mem) = s;
    match board[i][j] {
        '<' => d = 1,
        '>' => d = 0,
        '^' => d = 3,
        'v' => d = 2,
        '_' => if mem == 0 {
            d = 0
        } else {
            d = 1
        }
        '|' => if mem == 0 {
            d = 2
        } else {
            d = 3
        }
        '?' => {
            let mut res = false;
            for nd in 0..4 {
                let nx = (i + dir[nd].0) % h;
                let ny = (j + dir[nd].1) % w;
                if dfs((nx, ny, nd, mem), board, memo, vis) {
                    res = true;
                    break;
                }
            }
            memo.insert(s, res);
            return res;
        }
        '.' => {}
        '@' => {
            memo.insert(s, true);
            return true;
        }
        '+' => mem = (mem + 1) & 15,
        '-' => mem = (mem + 15) & 15,
        x => {
            let val = x as u8 - b'0';
            mem = val;
        }
    }
    let nx = (i + dir[d].0) % h;
    let ny = (j + dir[d].1) % w;
    dfs((nx, ny, d, mem), board, memo, vis)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: usize, _w: usize,
        s: [chars; h],
    }
    let mut vis = HashSet::new();
    let mut memo = HashMap::new();
    let init = (0, 0, 0, 0u8);
    let ans = dfs(init, &s, &mut memo, &mut vis);
    puts!("{}\n", if ans { "YES" } else { "NO" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
