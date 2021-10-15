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
        n: usize, m: usize, x: i64,
        ab: [(i64, usize1); n],
        k: usize,
        c: [usize; k],
    }
    let mut occ = vec![vec![]; m];
    for &(a, b) in &ab {
        occ[b].push(a);
    }
    let mut v: Vec<i64> = vec![];
    for i in 0..m {
        if occ[i].is_empty() { continue; }
        occ[i].sort(); occ[i].reverse();
        occ[i][0] += x;
        v.extend(&occ[i]);
    }
    v.sort(); v.reverse();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + v[i];
    }
    let mut tot = 0;
    for c in c {
        tot += acc[c];
    }
    println!("{}", tot);
}
