#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
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

// Tags: low-memory
fn main() {
    input! {
        n: usize, m: i32,
        a: [[usize1]; 3],
    }
    let mut p3 = vec![0; n + 1];
    p3[0] = 1;
    for i in 1..n + 1 {
        p3[i] = p3[i - 1] * 3;
    }
    let mut init = 0;
    for i in 0..3 {
        for &w in &a[i] {
            init += p3[w] * i;
        }
    }
    let nxt = |i: usize| {
        let mut ans = vec![];
        let mut ma = [0; 3];
        let mut v = i;
        for j in 0..n {
            let r = v % 3;
            ma[r].chmax(j + 1);
            v /= 3;
        }
        for j in 0..2 {
            if ma[j] > ma[j + 1] {
                ans.push((i + p3[ma[j] - 1]) as i32);
            }
            if ma[j] < ma[j + 1] {
                ans.push((i - p3[ma[j + 1] - 1]) as i32);
            }
        }
        ans
    };
    let mut que = VecDeque::new();
    que.push_back((0, init as i32));
    let mut dist = vec![1 << 30; p3[n]];
    while let Some((d, v)) = que.pop_front() {
        let v = v as usize;
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        for &w in &nxt(v) {
            que.push_back((d + 1, w));
        }
    }
    let mi = min(dist[0], dist[p3[n] - 1]);
    println!("{}", if mi > m { -1 } else { mi });
}
