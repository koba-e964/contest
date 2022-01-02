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

// Quick-Find data structure.
// Verified by: https://atcoder.jp/contests/cf17-tournament-round3-open/submissions/22928265
// Verified by: https://atcoder.jp/contests/ttpc2019/submissions/23384553 (polymorphic version)
struct QuickFind<T = ()> {
    root: Vec<usize>, mem: Vec<Vec<usize>>,
    dat: Vec<T>, default: T,
}

impl QuickFind<()> {
    #[allow(unused)]
    fn new(n: usize) -> Self {
        Self::with_dat(n, ())
    }
    #[allow(unused)]
    fn unite(&mut self, x: usize, y: usize) {
        self.unite_with_hooks(x, y, |&(), _| (), |(), ()| ());
    }
}
impl<T: Clone> QuickFind<T> {
    fn with_dat(n: usize, def: T) -> Self {
        let root = (0..n).collect();
        let mut mem = vec![vec![]; n];
        for i in 0..n {
            mem[i] = vec![i];
        }
        QuickFind { root: root, mem: mem, dat: vec![def.clone(); n], default: def }
    }
    fn root(&self, x: usize) -> usize {
        self.root[x]
    }
    #[allow(unused)]
    fn set(&mut self, idx: usize, val: T) {
        self.apply(idx, move |me| *me = val);
    }
    #[allow(unused)]
    fn get(&mut self, idx: usize) -> T {
        let mut ans = self.default.clone();
        self.apply(idx, |me| ans = me.clone());
        ans
    }
    fn apply<F: FnOnce(&mut T)>(&mut self, idx: usize, f: F) {
        let r = self.root[idx];
        f(&mut self.dat[r]);
    }
    // unite always merges y to x if |x| >= |y|.
    fn unite_with_hooks<F: FnMut(&T, i64), G: FnMut(T, T) -> T>(
        &mut self, x: usize, y: usize,
        mut hook: F, mut merge: G) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.mem[x].len() < self.mem[y].len() {
            std::mem::swap(&mut x, &mut y);
        }
        let memy = std::mem::replace(&mut self.mem[y], vec![]);
        for &v in &memy {
            self.root[v] = x;
        }
        self.mem[x].extend_from_slice(&memy);
        // hook
        hook(&self.dat[x], -1);
        hook(&self.dat[y], -1);
        self.dat[x] = merge(
            std::mem::replace(&mut self.dat[x], self.default.clone()),
            std::mem::replace(&mut self.dat[y], self.default.clone()),
        );
        hook(&self.dat[x], 1);
    }
    #[allow(unused)]
    fn is_same_set(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&self, x: usize) -> usize {
        let x = self.root(x);
        self.mem[x].len()
    }
}

// https://yukicoder.me/problems/no/828 (4.5)
// x が y を根とした場合に神童数 <=> x と y の間に x より大きい値がない
// よって、小さい数から順に繋いでいくと、それぞれの y について、y と y 以下の頂点をすべて繋いだときの部分木を考えたとき、x がその部分木に含まれていることと、x と y の間に x より大きい値がないこと (つまり、x を根にしたとき y が神童数であること) が同値。よって、つなぐ処理中で連結成分内の頂点全てに対して += 1 という操作ができれば良い。これは QuickFind などでできる。
// Tags: quick-find, query
fn main() {
    input! {
        n: usize,
        r: [i64; n],
        ab: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        if a < b {
            g[b].push(a);
        } else {
            g[a].push(b);
        }
    }
    let mut qf = QuickFind::with_dat(n, (vec![1], 0));
    for i in 0..n {
        qf.set(i, (vec![i], 0));
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        for &w in &g[i] {
            qf.unite_with_hooks(i, w,
                                |&(_, _), _coef| (),
                                |(mut t1, x1), (mut t2, x2)| {
                                    for &v in &t2 {
                                        ans[v] += x2 - x1;
                                    }
                                    t1.append(&mut t2);
                                    (t1, x1)
                                });
        }
        qf.apply(i, |elem| elem.1 += 1);
    }
    let bias = qf.get(0).1;
    let mut out = 1;
    for i in 0..n {
        let sol = ans[i] + bias;
        out = out * (sol + r[i]) % 1_000_000_007;
        if n < 10 {
            eprintln!("{}", sol);
        }
    }
    println!("{}", out);
}
