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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Verified by: https://atcoder.jp/contests/abc160/submissions/26509495
trait LeaveOne<Me = (), App = ()>: Default + Clone {
    type T: Default + Clone;
    type Me: Clone;
    type App;
    fn build(me: Self::Me, vals: &[Self::T], app: &Self::App) -> Self;
    fn leave_one(&self, excl: Self::T) -> Self::T;
    fn exchange_one(&self, excl: Self::T, incl: Self::T) -> Self::T;
    fn add_one(&self, incl: Self::T) -> Self::T;
    fn as_is(&self) -> Self::T;
}

struct Reroot<LOO: LeaveOne> {
    #[allow(unused)]
    pub dp1: Vec<LOO::T>,
    #[allow(unused)]
    pub dp2: Vec<Vec<LOO::T>>,
    #[allow(unused)]
    pub dp_loo: Vec<LOO>,
}

impl<LOO: LeaveOne> Reroot<LOO> {
    pub fn new(g: &[Vec<usize>], me: &[LOO::Me], app: &LOO::App) -> Self {
        let n = g.len();
        let mut dp1 = vec![LOO::T::default(); n];
        let mut dp2 = vec![vec![]; n];
        let mut dp_loo = vec![LOO::default(); n];
        Self::dfs1(0, n, &g, &mut dp_loo, &mut dp2, me, app);
        Self::dfs2(0, n, &g, &mut dp1, &dp_loo, &mut dp2, &app, LOO::T::default());
        Reroot {
            dp1: dp1,
            dp2: dp2,
            dp_loo: dp_loo,
        }
    }
    fn dfs1(
        v: usize, par: usize, g: &[Vec<usize>],
        dp_loo: &mut [LOO], dp2: &mut [Vec<LOO::T>], me: &[LOO::Me], app: &LOO::App,
    ) {
        let mut mydp2 = vec![LOO::T::default(); g[v].len()];
        let mut chval = vec![];
        for i in 0..g[v].len() {
            let w = g[v][i];
            if w == par { continue; }
            Self::dfs1(w, v, g, dp_loo, dp2, me, app);
            mydp2[i] = dp_loo[w].as_is();
            chval.push(mydp2[i].clone());
        }
        dp_loo[v] = LOO::build(me[v].clone(), &chval, app);
        dp2[v] = mydp2;
    }
    fn dfs2(
        v: usize, par: usize, g: &[Vec<usize>],
        dp1: &mut [LOO::T],
        dp_loo: &[LOO],
        dp2: &mut [Vec<LOO::T>],
        app: &LOO::App,
        passed: LOO::T,
    ) {
        for i in 0..g[v].len() {
            let w = g[v][i];
            if w == par {
                dp2[v][i] = passed.clone();
                continue;
            }
            let inherited = if par >= g.len() {
                dp_loo[v].leave_one(dp2[v][i].clone())
            } else {
                dp_loo[v].exchange_one(dp2[v][i].clone(), passed.clone())
            };
            Self::dfs2(w, v, g, dp1, dp_loo, dp2, app, inherited);
        }
        dp1[v] = if par >= g.len() {
            dp_loo[v].as_is()
        } else {
            dp_loo[v].add_one(passed)
        };
    }
}

#[derive(Clone, Default)]
struct MaxDistLeaveOne([i64; 2]);

// Depends on: tree/Reroot.rs
impl LeaveOne for MaxDistLeaveOne {
    type T = i64;
    type Me = bool;
    type App = ();
    fn build(me: bool, vals: &[i64], _: &()) -> Self {
        const INF: i64 = 1 << 50;
        let mut res = [-INF; 2];
        if me {
            res = [0; 2];
        }
        for &v in vals {
            res[0] = std::cmp::max(res[0], v + 1);
            res.sort();
        }
        MaxDistLeaveOne(res)
    }
    fn leave_one(&self, excl: i64) -> i64 {
        if self.0[1] == excl + 1 {
            self.0[0]
        } else {
            self.0[1]
        }
    }
    fn exchange_one(&self, excl: i64, incl: i64) -> i64 {
        std::cmp::max(self.leave_one(excl), incl + 1)
    }
    fn add_one(&self, incl: i64) -> i64 {
        std::cmp::max(self.0[1], incl + 1)
    }
    fn as_is(&self) -> i64 {
        self.0[1]
    }
}

// (#edges used, do we have an objective?)
#[derive(Clone, Default)]
struct UsedLeaveOne(i64, i32);

// Depends on: tree/Reroot.rs
impl LeaveOne for UsedLeaveOne {
    type T = (i64, bool);
    type Me = bool;
    type App = ();
    fn build(me: bool, vals: &[(i64, bool)], _: &()) -> Self {
        let mut res = 0;
        let mut me = if me { 1 } else { 0 };
        for &(v, has) in vals {
            if has {
                res += v + 1;
                me += 1;
            }
        }
        UsedLeaveOne(res, me)
    }
    fn leave_one(&self, excl: (i64, bool)) -> (i64, bool) {
        let UsedLeaveOne(mut res, mut me) = *self;
        if excl.1 {
            res -= excl.0 + 1;
            me -= 1;
        }
        if me > 0 {
            (res, true)
        } else {
            (0, false)
        }
    }
    fn exchange_one(&self, excl: (i64, bool), incl: (i64, bool)) -> (i64, bool) {
        let (res, me) = self.leave_one(excl);
        (res + if incl.1 { incl.0 + 1 } else { 0 }, incl.1 || me)
    }
    fn add_one(&self, incl: (i64, bool)) -> (i64, bool) {
        (self.0 + if incl.1 { incl.0 + 1 } else { 0 }, incl.1 || self.1 > 0)
    }
    fn as_is(&self) -> (i64, bool) {
        (self.0, self.1 > 0)
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// ​​https://yukicoder.me/problems/no/1718 (3.5)
// 全方位木 DP。答えは (ドングリに到達するのに通る辺の本数) * 2 - max(ドングリまでの距離)。
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, k: usize,
        uv: [(usize1, usize1); n - 1],
        d: [usize1; k],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut is_d = vec![false; n];
    for &d in &d {
        is_d[d] = true;
    }
    let re1 = Reroot::<MaxDistLeaveOne>::new(&g, &is_d, &());
    let re2 = Reroot::<UsedLeaveOne>::new(&g, &is_d, &());
    for i in 0..n {
        puts!("{}\n", 2 * re2.dp1[i].0 - re1.dp1[i]);
    }
}
