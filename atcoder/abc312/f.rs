use std::cmp::*;
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

#[derive(Debug)]
pub struct KSmallest {
    hi: std::collections::BTreeSet<(i64, usize)>,
    lo: std::collections::BTreeSet<(i64, usize)>,
    k: usize,
    losum: i64,
}

impl KSmallest {
    pub fn new(k: usize) -> Self {
        KSmallest {
            hi: std::collections::BTreeSet::new(),
            lo: std::collections::BTreeSet::new(),
            k: k,
            losum: 0,
        }
    }
    // x.1 should be unique.
    pub fn add(&mut self, x: (i64, usize)) {
        self.lo.insert(x);
        self.losum += x.0;
        self.adjust();
    }
    pub fn remove(&mut self, x: (i64, usize)) {
        if self.lo.remove(&x) {
            self.losum -= x.0;
            self.adjust();
            return;
        }
        self.hi.remove(&x);
    }
    fn adjust(&mut self) {
        if self.lo.len() < self.k && !self.hi.is_empty() {
            let &mi = self.hi.iter().next().unwrap();
            self.hi.remove(&mi);
            self.lo.insert(mi);
            self.losum += mi.0;
        }
        if self.lo.len() > self.k {
            let &ma = self.lo.iter().rev().next().unwrap();
            self.lo.remove(&ma);
            self.losum -= ma.0;
            self.hi.insert(ma);
        }
    }
    pub fn losum(&self) -> i64 {
        self.losum
    }
    pub fn set_k(&mut self, k: usize) {
        while self.k < k {
            self.k += 1;
            self.adjust();
        }
        while self.k > k {
            self.k -= 1;
            self.adjust();
        }
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        tx: [(i32, i64); n],
    }
    let mut cutter = vec![];
    let mut dat = KSmallest::new(m);
    let mut p = vec![];
    for i in 0..n {
        let (t, x) = tx[i];
        if t == 2 {
            cutter.push(x);
        }
        if t == 0 {
            dat.add((-x, i));
        }
        if t == 1 {
            p.push((-x, i));
        }
    }
    cutter.sort(); cutter.reverse();
    p.sort();
    let mut ans = -dat.losum();
    let mut cut = 0;
    let mut cur = 0;
    for i in 0..min(m, cutter.len()) {
        dat.set_k(m - i - 1);
        cut += cutter[i] as usize;
        while cur < cut && cur < p.len() {
            dat.add(p[cur]);
            cur += 1;
        }
        ans = max(ans, -dat.losum());
    }
    println!("{}", ans);
}
