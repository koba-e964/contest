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

fn main() {
    input! {
        n: usize, d: usize,
        a: [usize; n],
    }
    if d == 0 {
        let mut a = a;
        a.sort(); a.dedup();
        println!("{}", n - a.len());
        return;
    }
    let mut occ = vec![vec![]; d];
    for x in a {
        occ[x % d].push(x / d);
    }
    let mut ans = 0;
    for i in 0..d {
        occ[i].sort();
        if occ[i].is_empty() {
            continue;
        }
        let l = occ[i][occ[i].len() - 1];
        let mut dp = vec![0; l + 2];
        let mut f = vec![0; l + 1];
        for &v in &occ[i] {
            f[v] += 1;
        }
        for i in 1..=l + 1 {
            let mut mi = dp[i - 1] + f[i - 1];
            if i >= 2 {
                mi = mi.min(dp[i - 2] + f[i - 1]);
            }
            dp[i] = mi;
        }
        ans += dp[l + 1].min(dp[l]);
    }
    println!("{ans}");
}
