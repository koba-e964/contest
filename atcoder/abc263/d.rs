use std::cmp::*;
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
        n: usize, l: i64, r: i64,
        a: [i64; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut aux1 = vec![0; n];
    let mut aux2 = vec![0; n];
    for i in 1..n {
        aux1[i] = min(aux1[i - 1], l * i as i64 - acc[i]);
    }
    for i in (0..n - 1).rev() {
        aux2[i] = min(aux2[i + 1], r * (n - i - 1) as i64 - acc[n] + acc[i + 1]);
    }
    let mut mi = min(l, r) * n as i64;
    for i in 0..n {
        mi = min(mi, acc[n] + aux1[i] + aux2[i]);
    }
    println!("{}", mi);
}
