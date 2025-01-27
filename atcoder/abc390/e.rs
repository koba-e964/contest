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

fn calc(x: usize, kind: &[(i64, usize)]) -> Vec<i64> {
    let mut dp = vec![0; x + 1];
    for &(a, c) in kind {
        for j in (c..x + 1).rev() {
            dp[j] = dp[j].max(dp[j - c] + a);
        }
    }
    dp
}

fn main() {
    input! {
        n: usize, x: usize,
        vac: [(usize1, i64, usize); n],
    }
    let mut kind = vec![vec![]; 3];
    for (v, a, c) in vac {
        kind[v].push((a, c));
    }
    let mut tbl = vec![vec![]; 3];
    for i in 0..3 {
        tbl[i] = calc(x, &kind[i]);
    }
    let mut cur = tbl[0].clone();
    for i in 1..3 {
        let mut next = vec![0; x + 1];
        for j in 0..x + 1 {
            for k in 0..j + 1 {
                next[j] = next[j].max(cur[k].min(tbl[i][j - k]));
            }
        }
        cur = next;
    }
    println!("{}", cur[x]);
}
