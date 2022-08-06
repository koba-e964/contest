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

struct V {
    head: BinaryHeap<Reverse<i64>>,
    tail: BinaryHeap<i64>,
    tot: i64,
    lim: usize,
}

impl V {
    fn new() -> Self {
        V {
            head: BinaryHeap::new(),
            tail: BinaryHeap::new(),
            tot: 0,
            lim: 0,
        }
    }
    // O(|lim - self.lim| log n)
    fn set_lim(&mut self, lim: usize) {
        self.lim = lim;
        self.adjust();
    }
    fn adjust(&mut self) {
        while self.tail.len() < self.lim && !self.head.is_empty() {
            let v = self.head.pop().unwrap().0;
            self.tail.push(v);
            self.tot -= v;
        }
        while self.tail.len() > self.lim {
            let v = self.tail.pop().unwrap();
            self.head.push(Reverse(v));
            self.tot += v;
        }
    }
    fn push(&mut self, val: i64) {
        if val >= 0 {
            self.tot += val;
            return;
        }
        if self.lim == 0 {
            self.head.push(Reverse(val));
            self.tot += val;
            return;
        }
        self.tail.push(val);
        self.adjust();
    }
}

fn main() {
    input! {
        n: usize, k: usize,
        ty: [(i32, i64); n],
    }
    let mut ma = -1i64 << 50;
    let mut ign = 0;
    let mut que = V::new();
    for i in (0..n).rev() {
        let (t, y) = ty[i];
        if t == 1 {
            if ign <= k {
                que.set_lim(k - ign);
                ma = max(ma, y + que.tot);
            }
            ign += 1;
        } else {
            que.push(y);
        }
    }
    if ign <= k {
        que.set_lim(k - ign);
        ma = max(ma, que.tot);
    }
    println!("{}", ma);
}
