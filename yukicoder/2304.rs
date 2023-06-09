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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// https://maspypy.com/slope-trick-1-%e8%a7%a3%e8%aa%ac%e7%b7%a8
#[derive(Clone, Debug, Default)]
struct SlopeTrick {
    l: std::collections::BinaryHeap<i64>,
    r: std::collections::BinaryHeap<std::cmp::Reverse<i64>>,
    ladd: i64,
    radd: i64,
    mi: i64,
}

impl SlopeTrick {
    fn new() -> Self {
        Self::default()
    }
    fn min(&self) -> i64 {
        self.mi
    }
    #[allow(unused)]
    fn add_const(&mut self, a: i64) {
        self.mi += a;
    }
    // self += max(0, x - a)
    fn add_plus(&mut self, a: i64) {
        self.l.push(a - self.ladd);
        let x = self.l.pop().unwrap() + self.ladd;
        self.r.push(std::cmp::Reverse(x - self.radd));
        self.mi += std::cmp::max(0, x - a);
    }
    // self += max(0, a - x)
    fn add_minus(&mut self, a: i64) {
        self.r.push(std::cmp::Reverse(a - self.radd));
        let x = self.r.pop().unwrap().0 + self.radd;
        self.l.push(x - self.ladd);
        self.mi += std::cmp::max(0, a - x);
    }
    // self <- min(self(y) where x-b <= y <= x-a)
    fn sliding_min(&mut self, a: i64, b: i64) {
        self.ladd += a;
        self.radd += b;
    }
    // self <- min(self(y) where y <= x-a)
    fn sliding_min_left(&mut self, a: i64) {
        self.ladd += a;
        self.r.clear();
    }
}

// https://yukicoder.me/problems/no/2304 (4)
// a はソートされていると、また最終的な配列もソートされていると仮定して良い。
// i 番目の要素を x にするときの最小コストを f(x) とすると、i+1 番目の要素を x にするときの最小コストは
// min_{u < x} f(u) + |a[i+1] - x| である。これは slope trick (左からの累積 min と右に 1 シフト) でできる。
// Tags: slope-trick
fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut a = a;
    a.sort();
    let mut st = SlopeTrick::new();
    for &a in &a {
        st.sliding_min_left(1);
        st.add_plus(a);
        st.add_minus(a);
    }
    println!("{}", st.min());
}
