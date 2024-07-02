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

// https://yukicoder.me/problems/no/2718 (3.5)
// solved with hints
// A_i が同じ楽器の調和度は簡単に計算できるので、A_i は相異なるとして良い。
// A_i の昇順に以下をやる。
// (i) 各 x について、max(dp[A_i * x], floor(B_i/x)) を得られた調和度の max に反映する。
// (ii) 各 x について、dp[A_i * x].chmax(floor(B_i/x)) をやる。
// -> 解説を見た。先述の方法だと lcm(A_i, A_j) >= 2 * 10^5 であるペアをうまく拾えない。gcd を見る必要がある。
// g | gcd(A_i, A_j) なるペアおよび g に対して min(floor(gB_i/A_j), floor(gB_j/A_i)) をとると、g を動かした時の最大値が i と j の調和度である。
// g を固定した時のこの値の最大値を求めるために、min の左側の値を全部試す必要がある。 
// B_iA_i でソートしたあと、左側を (A_i, B_i), 右側を (A_j, B_j) として 2 個の値を組み合わせると、
// floor(gB_i/A_j) の方が小さい。したがって、各 j に対して max_{j < i}(B_i) をあらかじめ計算しておけば良い。
fn main() {
    input! {
        n: usize,
        ab: [(usize, i64); n],
    }
    const W: usize = 200_100;
    let mut ans = 0;
    let mut occ = vec![vec![]; W];
    for (a, b) in ab {
        occ[a].push(b);
    }
    for i in 1..W {
        occ[i].sort();
        occ[i].reverse();
        if occ[i].len() >= 2 {
            ans = ans.max(occ[i][1]);
        }
    }
    for g in 1..W {
        let mut val = vec![];
        for j in 1..(W - 1) / g + 1 {
            if occ[j * g].len() >= 1 {
                val.push((j, occ[j * g][0]));
            }
        }
        val.sort_by(|x, y| (x.1 * x.0 as i64).cmp(&(y.0 as i64 * y.1)));
        if val.len() >= 2 {
            let mut ma = vec![0; val.len()];
            ma[0] = val[0].1;
            for i in 1..val.len() {
                ma[i] = ma[i - 1].max(val[i].1);
            }
            for i in 1..val.len() {
                let (a, _) = val[i];
                ans = ans.max(ma[i - 1] / a as i64);
            }
        }
    }
    println!("{}", ans);
}
