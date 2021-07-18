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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: bit-dp
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize1; n],
    }
    let mut acc = vec![vec![0usize; n + 1]; m];
    for i in 0..n {
        for j in 0..m {
            acc[j][i + 1] = acc[j][i] + if a[i] == j { 1 } else { 0 };
        }
    }
    let mut dp = vec![0; 1 << m];
    for bits in 1..1 << m {
        let mut me = n;
        let mut sum = 0;
        for i in 0..m {
            if (bits & 1 << i) == 0 {
                continue;
            }
            sum += acc[i][n];
        }
        for i in 0..m {
            if (bits & 1 << i) == 0 {
                continue;
            }
            let pre = sum - acc[i][n];
            me = std::cmp::min(me, dp[bits ^ 1 << i] + (sum - pre) - (acc[i][sum] - acc[i][pre]));
        }
        dp[bits] = me;
    }
    println!("{}", dp[(1 << m) - 1]);
}
