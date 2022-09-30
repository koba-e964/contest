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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        res: [(usize1, usize1, i32, i32); n * (n - 1) / 2],
    }
    let mut pts = vec![0; n];
    for (a, b, c, d) in res {
        match c.cmp(&d) {
            Ordering::Greater => pts[a] += 3,
            Ordering::Less => pts[b] += 3,
            Ordering::Equal => {
                pts[a] += 1;
                pts[b] += 1;
            }
        }
    }
    let mut rnk = vec![];
    for i in 0..n {
        rnk.push((pts[i], i));
    }
    rnk.sort(); rnk.reverse();
    let mut ans = vec![0; n];
    let mut cur = -1;
    for i in 0..n {
        if cur != rnk[i].0 {
            cur = rnk[i].0;
            ans[rnk[i].1] = i + 1;
        } else {
            ans[rnk[i].1] = ans[rnk[i - 1].1];
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
