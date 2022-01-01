use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

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

// Registry.
#[derive(Clone, Default, Debug)]
struct Reg<T> {
    a: Vec<T>,
    inv: std::collections::HashMap<T, usize>,
}

impl<T: Default> Reg<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: std::hash::Hash + Eq + Clone> Reg<T> {
    pub fn get(&mut self, t: &T) -> usize {
        if !self.inv.contains_key(t) {
            let idx = self.a.len();
            self.a.push(t.clone());
            self.inv.insert(t.clone(), idx);
        }
        self.inv[t]
    }
    // init must have distinct elements.
    pub fn init<F>(&mut self, init: &[T], f: F) -> Vec<Vec<i64>>
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
}

#[derive(Clone, Default, Debug, Hash, Eq, PartialEq)]
struct State {
    back: i32,
    me: i32,
    uf: Vec<usize>,
    conn: i32,
    one: i32,
}

// For each component, the root is always the maximum element.
fn uf_naive(uf: &mut [usize], x: usize, y: usize) {
    if uf[x] == uf[y] { return; }
    let n = uf.len();
    let r = std::cmp::max(uf[x], uf[y]);
    let other = r ^ uf[x] ^ uf[y];
    for i in 0..n {
        if uf[i] == other {
            uf[i] = r;
        }
    }
}

fn next(s: State) -> Vec<State> {
    let mut ans = vec![];
    for i in 0..16 {
        let back = i;
        for j in 0..8 {
            let me = j;
            // Find the degree of each vertex
            let mut pre = [0; 4];
            let mut now = [0; 4];
            for k in 0..3 {
                if (me & 1 << k) != 0 {
                    now[k] += 1;
                    now[k + 1] += 1;
                }
                if (s.me & 1 << k) != 0 {
                    pre[k] += 1;
                    pre[k + 1] += 1;
                }
            }
            for k in 0..4 {
                if (back & 1 << k) != 0 {
                    now[k] += 1;
                    pre[k] += 1;
                }
                if (s.back & 1 << k) != 0 {
                    pre[k] += 1;
                }
            }
            if pre.iter().any(|&x| x > 2) {
                continue;
            }
            // Connectedness
            let mut nuf = vec![0; 8];
            for k in 0..4 {
                nuf[k] = s.uf[k];
                nuf[k + 4] = k + 4;
            }
            for k in 0..4 {
                if (back & 1 << k) == 0 { continue; }
                // union-find
                uf_naive(&mut nuf, k, k + 4);
            }
            for k in 0..3 {
                if (me & 1 << k) == 0 { continue; }
                // union-find
                uf_naive(&mut nuf, k + 4, k + 5);
            }
            let mut uf = vec![0; 4];
            let mut conn = s.conn;
            let mut one = s.one;
            for k in 0..4 {
                if pre[k] > 0 && nuf[k] == k {
                    conn += 1;
                }
                if one == 0 && pre[k] != 1 {
                    one += 10;
                }
                if pre[k] == 1 {
                    if k != 0 {
                        one += 10;
                    }
                    one += 1;
                }
                uf[k] = nuf[k + 4] - 4;
            }
            if conn == 0 && one <= 2 {
                ans.push(State {
                    back,
                    me,
                    uf,
                    conn,
                    one,
                });
            }
        }
    }
    ans
}

// https://yukicoder.me/problems/no/569 (4)
// 面倒なことで知られる連結性 DP を、行列累乗に変換する。
// 状態は https://yukicoder.me/problems/no/541 とほとんど同じ。
// それに加えて、次数 1 の頂点が今までに何個あったか持てば良い。また、終点が (3, N) であるという条件は終状態に任せて、始点が (0, 0) であるという条件を満たすために、next の内部で
// 「次数 1 の頂点は頂点 0 のみ」という条件を要請すれば良い。
// -> 頂点 (0, ?) であって次数 1 であるものが (0, 0) より後に出現してしまう場合に対応できない。それを避けるために、next の内部で、次数 1 の頂点を見ていない状態で次数 2 以上の頂点 0 を見ると不正な状態とみなす処理を ad-hoc に追加した。
// 行列累乗に変換する部分はライブラリを書いた。この問題の場合は 158 次行列となる。
// Similar-problems: https://yukicoder.me/problems/no/541
// Tags: connectedness-dp, matrix-exponentiation
fn main() {
    const MOD: i64 = 1_000_000_007;
    let n: i64 = get();
    let mut reg = Reg::<State>::new();
    let mut uf = vec![0; 4];
    for i in 0..4 {
        uf[i] = i;
    }
    let init = State { back: 0, me: 0, uf: uf.clone(), conn: 0, one: 0 };
    let term = State { back: 8, me: 0, uf: uf.clone(), conn: 0, one: 1 };
    let mat = reg.init(&[init.clone()], next);
    let idx2 = reg.get(&term);
    let pw = squpow(&mat, n + 2, MOD);
    let mut tot = 0;
    for i in 0..reg.a.len() {
        let s = &reg.a[i];
        if s.back == 0 && s.conn == 0 && s.one == 0 {
            tot = (tot + pw[i][idx2]) % MOD;
        }
    }
    println!("{}", tot);
}
