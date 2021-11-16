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

// Tags: bit-dp
fn main() {
    input! {
        n: usize,
        abc: [[i64; 3]; n],
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![vec![[-INF; 3]; 1 << n]; n];
    let get = |idx: usize, x: usize| -> (i64, [i64; 2]) {
        let mut ans = [0; 2];
        for i in 1..3 {
            ans[i - 1] = abc[idx][(x + i) % 3];
        }
        ans.sort();
        (abc[idx][x], ans)
    };
    for i in 0..n {
        for j in 0..3 {
            let (w, _v) = get(i, j);
            dp[i][1 << i][j] = w;
        }
    }
    for bits in 1usize..1 << n {
        if bits.count_ones() <= 1 { continue; }
        for i in 0..n {
            if (bits & 1 << i) == 0 { continue; }
            for j in 0..n {
                if i == j || (bits & 1 << i) == 0 { continue; }
                for k in 0..3 {
                    let val = dp[j][bits ^ 1 << i][k];
                    let (_, w1) = get(j, k);
                    for l in 0..3 {
                        let (v2, w2) = get(i, l);
                        if w1[0] <= w2[0] && w1[1] <= w2[1] {
                            dp[i][bits][l].chmax(val + v2);
                        }
                    }
                }
            }
        }
    }
    let mut ans = -INF;
    for i in 0..n {
        for bits in 0..1 << n {
            for k in 0..3 {
                ans.chmax(dp[i][bits][k]);
            }
        }
    }
    println!("{}", ans);
}
