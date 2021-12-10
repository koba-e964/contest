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

/// Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
/// T is a commutative monoid. Indices are 1 .. n.
/// Verified by yukicoder No.404 (http://yukicoder.me/submissions/155373)
struct BIT<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> BIT<T> {
    fn new(n: usize, e: T) -> Self {
        let n = n.next_power_of_two();
        BIT { n: n, ary: vec![e.clone(); n + 1], e: e }
    }
    /**
     * gets the sum in [1 .. idx]
     * @param idx
     * @return sum
     */
    fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx].clone();
            idx &= idx - 1;
        }
        sum
    }
    /**
     * performs data[idx] += val;
     */
    fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where T: std::ops::AddAssign<U> {
        assert!(idx > 0);
        let n = self.n;
        while idx <= n {
            self.ary[idx] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
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

fn dfs(v: usize, ch: &[Vec<usize>], bit: &mut BIT<i64>) -> i64 {
    let mut ans = bit.accum(v);
    bit.add(v + 1, 1);
    for &w in &ch[v] {
        ans += dfs(w, ch, bit);
    }
    bit.add(v + 1, -1);
    ans
}

// https://yukicoder.me/problems/no/778 (2.5)
// セグメント木や BIT を持ちながら DFS。セグメント木や BIT は先祖の頻度を管理することにする。
fn solve() {
    input! {
        n: usize,
        a: [usize; n - 1],
    }
    let mut ch = vec![vec![]; n];
    for i in 0..n - 1 {
        ch[a[i]].push(i + 1);
    }
    let mut bit = BIT::new(n, 0);
    println!("{}", dfs(0, &ch, &mut bit));
}
