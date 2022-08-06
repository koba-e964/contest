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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; m],
        d: [i64; m],
    }
    let mut ab = vec![];
    let mut cd = vec![];
    for i in 0..n {
        ab.push((a[i], b[i]));
    }
    for i in 0..m {
        cd.push((c[i], d[i]));
    }
    ab.sort();
    cd.sort();
    let mut set = BTreeSet::new();
    let mut pos = m;
    for (a, b) in ab.into_iter().rev() {
        while pos > 0 && cd[pos - 1].0 >= a {
            set.insert((cd[pos - 1].1, pos));
            pos -= 1;
        }
        if let Some(&v) = set.range((b, 0)..).next() {
            set.remove(&v);
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
