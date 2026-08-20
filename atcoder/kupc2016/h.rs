fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
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
    #[allow(unused)]
    fn min(&self) -> i64 {
        self.mi
    }
    #[allow(unused)]
    fn at(self, x: i64) -> i64 {
        let mut ans = self.mi;
        for p in self.l {
            ans += 0.max(p + self.ladd - x);
        }
        for p in self.r {
            ans += 0.max(x - p.0 - self.radd);
        }
        ans
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
    // self <- min(self(y) where x-b <= y)
    fn sliding_min_right(&mut self, b: i64) {
        self.l.clear();
        self.radd += b;
    }
}

// Solved with hints
// https://atcoder.jp/contests/kupc2016/tasks/kupc2016_h?lang=ja
// 先頭から見て、 右に送る量 -> 今まで払ったコスト という関数を管理することにする。
// - 現在地で使う量を y とすると、新しく送る量は A[i] + x - y であり、y >= B[i] である必要がある。
// - また、追加コストは |x| である。
// - g(x) := min { f(y) + |y| | y >= x + B[i] - A[i] } である。
// Slope trick で実装するのであれば、 f |-> x |-> f(x) + |x| と 左側全カット (sliding_window_min) を実装する必要がある。
// -> https://maspypy.com/slope-trick-2-%e5%95%8f%e9%a1%8c%e7%b7%a8#toc4を読んだ。
// 「壁」を作る必要はなく、十分な傾きをつけておけば良い。Nで十分。これはコストNで範囲外から調達することを許すことに相当し、それよりは範囲内から調達した方が常に安いため。
// - (Input)
// - SlopeTrick を貼る
// - SlopeTrick に機能追加する
//   - sliding_min_right を作る
//   - f(a) を計算できるようにする
// - 初期化する
//   - newを呼ぶ
//   - 左右にN回0を追加する
// - add_minus, add_plus を呼ぶ
// - sliding_min_right を呼ぶ
// - f(0)を println! する
fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let b = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut slt = SlopeTrick::new();
    for i in 0..n {
        slt.add_plus(0);
        slt.add_minus(0);
    }
    for i in 0..n {
        slt.add_plus(0);
        slt.add_minus(0);
        slt.sliding_min_right(a[i] - b[i]);
    }
    println!("{}", slt.at(0));
}
