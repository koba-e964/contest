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
}

// The author solved this after reading the editorial.
// Tags: slope-trick
fn main() {
    input! {
        n: usize,
        tdx: [(i64, i32, i64); n],
    }
    let mut slt = SlopeTrick::new();
    for _ in 0..n {
        slt.add_plus(0);
        slt.add_minus(0);
    }
    let mut last = 0;
    for (t, d, x) in tdx {
        let dif = t - last;
        slt.sliding_min(-dif, dif);
        if d == 0 {
            slt.add_minus(x);
        } else {
            slt.add_plus(x);
        }
        last = t;
    }
    println!("{}", slt.min());
}
