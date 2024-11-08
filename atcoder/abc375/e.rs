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

const INF: i32 = 1 << 28;

fn main() {
    input! {
        n: usize,
        ab: [(i32, usize); n],
    }
    const W: usize = 501;
    let bsum = ab.iter().map(|&(_, b)| b).sum::<usize>();
    if bsum % 3 != 0 {
        println!("-1");
        return;
    }
    let mut dp = vec![vec![INF; W]; W];
    dp[0][0] = 0;
    for (a, b) in ab {
        let mut ep = vec![vec![INF; W]; W];
        for i in 0..W {
            for j in 0..W {
                ep[i][j] = ep[i][j].min(dp[i][j] + (a != 3) as i32);
                if i + b < W {
                    ep[i + b][j] = ep[i + b][j].min(dp[i][j] + (a != 1) as i32);
                }
                if j + b < W {
                    ep[i][j + b] = ep[i][j + b].min(dp[i][j] + (a != 2) as i32);
                }
            }
        }
        dp = ep;
    }
    let ans = dp[bsum / 3][bsum / 3];
    println!("{}", if ans == INF { -1 } else { ans });
}
