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

fn parse(s: &str) -> usize {
    let s: Vec<_> = s.split(":").collect();
    assert_eq!(s.len(), 2);
    let m: usize = s[0].parse().unwrap();
    let s: usize = s[1].parse().unwrap();
    60 * m + s
}

fn calc(l: usize, s: &[usize]) -> f64 {
    let n = s.len();
    let mut dp = vec![vec![0.0; l]; n + 1];
    dp[0][0] = 1.0;
    for i in 0..n {
        let mut ep = dp.clone();
        for u in 0..n {
            for j in 0..l {
                let val = dp[u][j];
                if u + 1 < n && j + s[i] < l {
                    ep[u + 1][j + s[i]] += val;
                }
            }
        }
        dp = ep;
    }
    let mut comb = vec![vec![0.0; n + 1]; n + 1];
    comb[0][0] = 1.0;
    for i in 1..n + 1 {
        for j in 0..n + 1 {
            comb[i][j] = comb[i - 1][j];
            if j > 0 {
                comb[i][j] += comb[i - 1][j - 1];
            }
        }
    }
    let mut ans = 0.0;
    let mut tmp = vec![vec![0.0; l]; n];
    for i in 0..n {
        // modosu dp (find dp values without i-th element)
        for j in 0..n {
            for k in 0..l {
                tmp[j][k] = dp[j][k];
                if j > 0 && k >= s[i] {
                    tmp[j][k] -= tmp[j - 1][k - s[i]];
                }
            }
        }
        for j in 0..n {
            for k in 0..l {
                // (* i! * (n - i - 1)! / n!) = (/ n / comb[n - 1][i])
                ans += tmp[j][k] / comb[n - 1][j];
            }
        }
    }
    ans / n as f64
}

fn main() {
    input! {
        n: usize, l: usize,
        s: [String; n],
    }
    let l = 60 * l;
    let s: Vec<_> = s.iter().map(|x| parse(x)).collect();
    let ans = calc(l, &s);
    println!("{}", ans);
}
