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

fn rec(d: &[usize], a: &[i64], pos: usize, ord: &[usize],
       memo: &mut HashMap<(usize, Vec<usize>), i64>) -> i64 {
    if pos >= d.len() {
        return 0;
    }
    let key = (pos, ord.to_vec());
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let mut ma = 0;
    for i in 0..ord.len() + 1 {
        let mut tmp = 0;
        let mut neword = vec![];
        for j in 0..i {
            if pos - ord[j] <= d[ord[j]] {
                tmp += a[ord[j]];
            }
            if pos + 1 - ord[j] <= 7 {
                neword.push(ord[j]);
            }
        }
        neword.push(pos);
        for j in i..ord.len() {
            if pos - ord[j] <= d[pos] {
                tmp += a[pos];
            }
            if pos + 1 - ord[j] <= 7 {
                neword.push(ord[j]);
            }
        }
        ma.chmax(tmp + rec(d, a, pos + 1, &neword, memo));
    }
    memo.insert(key, ma);
    ma
}

// Tags: dp, memoization, permutation-indexed-dp
fn solve() {
    input! {
        n: usize,
        d: [usize; n],
        a: [i64; n],
    }
    let mut memo = HashMap::new();
    println!("{}", rec(&d, &a, 0, &[], &mut memo));
}
