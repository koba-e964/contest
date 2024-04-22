use std::collections::*;
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

const INF: i64 = 1 << 58;

fn rec(ca: &[(i64, Vec<usize>)], i: usize, v: Vec<usize>, p: usize, memo: &mut HashMap<(usize, Vec<usize>), i64>) -> i64 {
    if i == ca.len() {
        for &v in &v {
            if v < p {
                return INF;
            }
        }
        return 0;
    }
    if let Some(&mi) = memo.get(&(i, v.clone())) {
        return mi;
    }
    let mut mi = rec(ca, i + 1, v.clone(), p, memo);
    let (ci, ref ai) = ca[i];
    let mut nv = v.clone();
    for j in 0..v.len() {
        nv[j] = std::cmp::min(v[j] + ai[j], p);
    }
    mi = std::cmp::min(mi, rec(ca, i + 1, nv, p, memo) + ci);
    memo.insert((i, v), mi);
    mi
}

fn main() {
    input! {
        n: usize, k: usize, p: usize,
        ca: [(i64, [usize; k]); n],
    }
    let mut memo = HashMap::new();
    let ans = rec(&ca, 0, vec![0; k], p, &mut memo);
    println!("{}", if ans >= INF { -1 } else { ans });
}
