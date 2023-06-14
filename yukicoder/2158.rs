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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn rec(
    k: usize,
    pst: &[(usize, usize, usize)],
    rem: Vec<usize>,
    memo: &mut HashMap<Vec<usize>, (i64, f64)>,
) -> (i64, f64) {
    if let Some(&val) = memo.get(&rem) {
        return val;
    }
    let n = pst.len();
    let mut ma = 0;
    let mut ex = 1.0;
    'outer:
    for bits in 0..1 << n {
        let mut prob = 1.0;
        let mut time = 0;
        let mut nxtrem = rem.clone();
        for i in 0..n {
            let (p, s, t) = pst[i];
            if (bits & 1 << i) != 0 {
                if rem[i] == 0 {
                    continue 'outer;
                }
                nxtrem[i] -= 1;
                prob *= rem[i] as f64 / (p - k) as f64;
                time += s;
            } else {
                if rem[i] == p - k {
                    continue 'outer;
                }
                prob *= (p - k - rem[i]) as f64 / (p - k) as f64;
                time += t;
            }
        }
        let (subma, subex) = if time <= 60 {
            (0, 0.0)
        } else {
            rec(k, &pst, nxtrem, memo)
        };
        ma = max(ma, subma + 1);
        ex += prob * subex;
    }
    memo.insert(rem, (ma, ex));
    (ma, ex)
}

// https://yukicoder.me/problems/no/2158 (4)
// \sum s_i <= 60 のとき必ず初日で全完できる。
// \sum t_i > 60 のとき、そもそも全完は不可能である。
// 最小値は K+2 である。
// 状態数は 5^6 程度、なおかつ全部既出でない限りまだ見ていない問題は 1 問ずつ減るため、最大でも 30 日くらいで全部既出になる。
fn solve() {
    input! {
        n: usize, k: usize,
        pst: [(usize, usize, usize); n],
    }
    let mut init = vec![0; n];
    let mut ssum = 0;
    let mut tsum = 0;
    for i in 0..n {
        let (p, s, t) = pst[i];
        init[i] = p - k;
        ssum += s;
        tsum += t;
    }
    if ssum <= 60 {
        println!("1 1 1");
        return;
    }
    if tsum > 60 {
        println!("-1");
        return;
    }
    let mut memo = HashMap::new();
    let (ma, ex) = rec(k, &pst, init, &mut memo);
    println!("{} {} {}", k + 2, ma + k as i64, ex + k as f64);
}
