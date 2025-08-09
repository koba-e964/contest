use std::io::{Write, BufWriter};
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

const INF: i64 = 1 << 60;

fn squmul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut ret = vec![vec![-INF; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][k] = ret[i][k].max(a[i][j] + b[j][k]);
            }
        }
    }
    ret
}

fn squpow(a: &[Vec<i64>], mut e: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut sum = vec![vec![-INF; n]; n];
    for i in 0..n { sum[i][i] = 0; }
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur);
        }
        cur = squmul(&cur, &cur);
        e /= 2;
    }
    sum
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: i64,
        t: [i64; n],
        a: [[i64; n]; n],
    }
    let apw = squpow(&a, m);
    let mut ans = vec![-INF; n];
    for i in 0..n {
        for j in 0..n {
            ans[j] = ans[j].max(apw[i][j] + t[i]);
        }
    }
    putvec!(ans);
}
