use std::collections::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(a: &[i64], m: i64, x: i64, memo: &mut HashMap<(Vec<i64>, i64), bool>) -> bool {
    if let Some(&val) = memo.get(&(a.to_vec(), x)) {
        return val;
    }
    let n = a.len();
    let mut win = false;
    for i in 0..n {
        for j in 1..a[i] + 1 {
            if j % m <= x {
                let mut acp = a.to_vec();
                acp[i] -= j;
                let sub = dfs(&acp, m, j % m, memo);
                if !sub {
                    win = true;
                }
            }
            let mut acp = a.to_vec();
            acp[i] -= j;
            dfs(&acp, m, m - 1, memo);
        }
    }
    if x == m - 1 && !win {
        eprintln!("a = {:?}, x = {}, win = {}", a, x, win);
    }
    memo.insert((a.to_vec(), x), win);
    win
}

fn solve() {
    input! {
        n: usize, m: i64,
        a: [i64; n],
    }
    let mut memo = HashMap::new();
    dfs(&a, m, m - 1, &mut memo);
}
