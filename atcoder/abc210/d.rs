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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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

const INF: i64 = 1 << 50;

fn calc(a: &[Vec<i64>], c: i64) -> i64 {
    let h = a.len();
    let w = a[0].len();
    let mut dp = vec![vec![INF; w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            let mut me = a[i][j] + c * (i + j) as i64;
            if i + 1 < h {
                me.chmin(dp[i + 1][j]);
            }
            if j + 1 < w {
                me.chmin(dp[i][j + 1]);
            }
            dp[i][j] = me;
        }
    }
    let mut ans = INF;
    for i in 0..h {
        for j in 0..w {
            let me = a[i][j] - c * (i + j) as i64;
            if i + 1 < h {
                ans.chmin(me + dp[i + 1][j]);
            }
            if j + 1 < w {
                ans.chmin(me + dp[i][j + 1]);
            }
        }
    }
    ans
}

fn solve() {
    input! {
        h: usize, w: usize, c: i64,
        a: [[i64; w]; h],
    }
    let mut a = a;
    let mut mi = calc(&a, c);
    for i in 0..h {
        a[i].reverse();
    }
    mi.chmin(calc(&a, c));
    a.reverse();
    mi.chmin(calc(&a, c));
    for i in 0..h {
        a[i].reverse();
    }
    mi.chmin(calc(&a, c));
    println!("{}", mi);
}
