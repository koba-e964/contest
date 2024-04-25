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

fn main() {
    input! {
        n: usize, k: usize,
        cv: [(usize1, i64); n],
    }
    let vsum: i64 = cv.iter().map(|&(_, v)| v).sum();
    const INF: i64 = 1 << 60;
    let mut dp = vec![[(n, INF); 2]; k + 1];
    dp[0][0] = (n, 0);
    for (c, v) in cv {
        let mut ep = vec![[(n, INF); 2]; k + 1];
        for i in 0..=k {
            let mut tmp = [(n, INF); 4];
            for j in 0..2 {
                let (c0, v0) = dp[i][j];
                if c0 != c {
                    tmp[j] = (c, v0);
                }
                if i > 0 {
                    let (c1, v1) = dp[i - 1][j];
                    tmp[2 + j] = (c1, v1 + v);
                }
            }
            tmp.sort_unstable_by_key(|x| x.1);
            ep[i][0] = tmp[0];
            let mut idx = 1;
            while idx < 4 {
                if tmp[idx].0 != tmp[0].0 {
                    break;
                }
                idx += 1;
            }
            if idx < 4 {
                ep[i][1] = tmp[idx];
            }
        }
        dp = ep;
    }
    let ans = dp[k][0].1;
    println!("{}", if ans == INF { -1 } else { vsum - ans });
}
