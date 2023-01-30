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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

type Coord = f64; // the type of coordinates
type P<C = Coord> = (C, C);

fn sub<C: std::ops::Sub<Output = C>>((ax, ay): P<C>, (bx, by): P<C>) -> P<C> {
    (ax - bx, ay - by)
}

fn norm((x, y): P) -> Coord {
    x * x + y * y
}

fn main() {
    input! {
        n: usize, m: usize,
        xy: [(f64, f64); n + m],
    }
    const INF: f64 = 1.0e15;
    let mut dp = vec![vec![INF; n + m]; 1 << (n + m)];
    dp[0][0] = 0.0;
    for i in 0..n + m {
        dp[1 << i][i] = norm(xy[i]).sqrt();
    }
    for bits in 1usize..1 << (n + m) {
        if bits.count_ones() <= 0 { continue; }
        let boost = 1 << (bits >> n).count_ones();
        for i in 0..n + m {
            if (bits & 1 << i) == 0 { continue; }
            for j in 0..n + m {
                if (bits & 1 << j) != 0 { continue; }
                let tmp = dp[bits][i] + norm(sub(xy[i], xy[j])).sqrt() / boost as f64;
                dp[bits | 1 << j][j].chmin(tmp);
            }
        }
    }
    let mut ans = INF;
    for i in 0usize..1 << m {
        let boost = 1 << i.count_ones();
        for j in 0..n + m {
            ans.chmin(dp[(1 << n) - 1 + (i << n)][j] + norm(xy[j]).sqrt() / boost as f64);
        }
    }
    println!("{}", ans);
}
