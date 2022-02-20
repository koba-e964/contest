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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Registry.
#[derive(Clone, Debug)]
struct Reg<T> {
    a: Vec<T>,
    inv: std::collections::HashMap<T, usize>,
}

impl<T: std::hash::Hash + Eq + Clone> Reg<T> {
    fn get(&mut self, t: &T) -> usize {
        if !self.inv.contains_key(t) {
            let idx = self.a.len();
            self.a.push(t.clone());
            self.inv.insert(t.clone(), idx);
        }
        self.inv[t]
    }
    // init must have distinct elements.
    fn init<F>(&mut self, init: &[T], f: F) -> Vec<Vec<i64>>
    where F: Fn(T) -> Vec<T> {
        let mut que = std::collections::VecDeque::new();
        for t in init {
            let idx = self.get(t);
            que.push_back(idx);
        }
        let mut n = self.a.len();
        let mut vis = vec![false; n];
        let mut to = vec![vec![]; n];
        while let Some(v) = que.pop_front() {
            if vis[v] { continue; }
            let ans = f(self.a[v].clone());
            let mut entries = vec![];
            for elem in ans {
                let idx = self.get(&elem);
                entries.push(idx);
                if n <= idx {
                    // A newly created entry.
                    n = self.a.len();
                    vis.resize(n, false);
                    to.resize(n, vec![]);
                    que.push_back(idx);
                }
            }
            vis[v] = true;
            to[v] = entries;
        }
        let mut ans = vec![vec![0; n]; n];
        for i in 0..n {
            for &e in &to[i] {
                ans[i][e] += 1;
            }
        }
        ans
    }
    pub fn find_index(&self, t: &T) -> usize {
        self.inv[t]
    }
    // init must have distinct elements.
    pub fn new<F>(init: &[T], f: F) -> (Self, Vec<Vec<i64>>)
    where F: Fn(T) -> Vec<T> {
        let mut me = Reg { a: vec![], inv: std::collections::HashMap::default() };
        let res = me.init(init, f);
        (me, res)
    }
}

fn squmul(a: &[Vec<i64>], b: &[Vec<i64>], mo: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][k] += a[i][j] * b[j][k];
                ret[i][k] %= mo;
            }
        }
    }
    ret
}

fn squpow(a: &[Vec<i64>], mut e: i64, mo: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut sum = vec![vec![0; n]; n];
    for i in 0..n { sum[i][i] = 1; }
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur, mo);
        }
        cur = squmul(&cur, &cur, mo);
        e /= 2;
    }
    sum
}

const MOD: i64 = 1_000_000_007;

// Tags: matrix-exponentiation
fn main() {
    input! {
        l: i64, n: usize, m: usize,
        k: [usize; m],
    }
    let mut pop = vec![false; n];
    for k in k {
        pop[k - 1] = true;
    }
    let init = (-1, -1);
    let next = |(x, y)| -> Vec<(i32, i32)> {
        if (x, y) == (-2, -2) {
            return vec![(-2, -2); n];
        }
        let mut v = vec![];
        for i in 0..n as i32 {
            if x == i {
                v.push((x, min(n as i32, y + 1)));
            } else if x >= 0 && x == y && pop[x as usize] {
                v.push((-2, -2));
            } else {
                v.push((i, 0));
            }
        }
        v
    };
    let (reg, mat) = Reg::new(&[init, (-2, -2)], next);
    let pw = squpow(&mat, l, MOD);
    let ii = reg.find_index(&init);
    let mut tot = pw[ii][reg.find_index(&(-2, -2))];
    for i in 0..n as i32 {
        if pop[i as usize] {
            tot += pw[ii][reg.find_index(&(i, i))];
            tot %= MOD;
        }
    }
    println!("{}", tot);
}
