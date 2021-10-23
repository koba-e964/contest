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
        m: usize,
        ab: [(usize1, usize1); m],
        p: [usize1; 8],
    }
    let init: Vec<usize> = (0..9).collect();
    let mut que = VecDeque::new();
    que.push_back((0, init));
    let mut dist = HashMap::new();
    while let Some((d, v)) = que.pop_front() {
        if let Some(&oldd) = dist.get(&v) {
            if oldd <= d {
                continue;
            }
        }
        dist.insert(v.clone(), d);
        for &(a, b) in &ab {
            if v[a] != 8 && v[b] != 8 {
                continue;
            }
            let mut w = v.clone();
            w.swap(a, b);
            que.push_back((d + 1, w));
        }
    }
    let mut st = vec![8; 9];
    for i in 0..8 {
        st[p[i]] = i;
    }
    if let Some(&d) = dist.get(&st) {
        println!("{}", d);
    } else {
        println!("-1");
    }
}
