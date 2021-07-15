use std::cmp::*;
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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn newvis(k: usize, x: i32, y: i32, nx: i32, ny: i32, vis: &[(i32, i32)])
          -> Vec<(i32, i32)> {
    let mut ans = vec![];
    for &(x, y) in vis {
        if max(0, nx - x) + max(0, ny - y) <= k as i32 {
            ans.push((x, y));
        }
    }
    ans.push((x, y));
    ans.sort_unstable();
    ans.dedup();
    ans
}

const INF: i32 = 1 << 26;

fn rec(k: usize, h: i32, w: i32, s: &[Vec<char>],
       rem: usize, x: i32, y: i32, vis: &[(i32, i32)],
       memo: &mut HashMap<(usize, i32, i32, Vec<(i32, i32)>), i32>) -> i32 {
    if x == h - 1 && y == w - 1 && rem == 0 {
        return 0;
    }
    if s[x as usize][y as usize] == '#' {
        return -INF;
    }
    let key = (rem, x, y, vis.to_vec());
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let visited = vis.binary_search(&(x, y)).is_ok();
    let mut ma = -INF;
    if x < h - 1 {
        let newvis = newvis(rem, x, y, x + 1, y, &vis);
        ma.chmax(rec(k, h, w, s,
                     rem, x + 1, y, &newvis,
                     memo));
    }
    if y < w - 1 {
        let newvis = newvis(rem, x, y, x, y + 1, &vis);
        ma.chmax(rec(k, h, w, s,
                     rem, x, y + 1, &newvis,
                     memo));
    }
    if rem > 0 {
        if x > 0 {
            let newvis = newvis(rem - 1, x, y, x - 1, y, &vis);
            ma.chmax(rec(k, h, w, s,
                         rem - 1, x - 1, y, &newvis,
                         memo));
        }
        if y > 0 {
            let newvis = newvis(rem - 1, x, y, x, y - 1, &vis);
            ma.chmax(rec(k, h, w, s,
                         rem - 1, x, y - 1, &newvis,
                         memo));
        }
    }
    if !visited && s[x as usize][y as usize] != '.' {
        ma += (s[x as usize][y as usize] as u8 - b'0') as i32;
    }
    memo.insert(key, ma);
    ma
}

// The author read the editorial and the model solution
// before implementing this.
fn solve() {
    input! {
        h: usize, w: usize, k: usize,
        s: [chars; h],
    }
    let mut memo = HashMap::new();
    let ans = rec(k, h as i32, w as i32, &s,
                  k, 0, 0, &[], &mut memo);
    println!("{}", ans);
}
