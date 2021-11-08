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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [[i64; m]; m],
        xy: [(usize1, usize1); n],
    }
    let mut acc = vec![vec![0; m + 1]; m + 1];
    for i in 0..m {
        for j in 0..m {
            acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j]
                + a[i][j];
        }
    }
    for &(x, y) in &xy {
        let mut tot = 0;
        for i in 0..x + 1 {
            for j in x + 1..m + 1 {
                let mut hm = HashMap::new();
                for k in 0..y + 1 {
                    *hm.entry(acc[j][k] - acc[i][k]).or_insert(0) += 1;
                }
                for k in y + 1..m + 1 {
                    tot += *hm.get(&(acc[j][k] - acc[i][k])).unwrap_or(&0);
                }
            }
        }
        println!("{}", tot);
    }
}
