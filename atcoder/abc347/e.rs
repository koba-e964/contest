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
        n: usize, q: usize,
        x: [usize; q],
    }
    let mut a = vec![0; n];
    let mut on = vec![false; n];
    let mut card = 0i64;
    let mut acc = vec![0; q + 1];
    let mut occ = vec![vec![]; n];
    for i in 0..q {
        let x = x[i] - 1;
        occ[x].push(i);
        if on[x] {
            on[x] = false;
            card -= 1;
        } else {
            on[x] = true;
            card += 1;
        }
        acc[i + 1] = acc[i] + card;
    }
    for i in 0..n {
        for j in 0..(occ[i].len() + 1) / 2 {
            let lo = occ[i][j * 2];
            let hi = if 2 * j + 1 >= occ[i].len() { q } else { occ[i][2 * j + 1] };
            a[i] += acc[hi] - acc[lo];
        }
    }
    putvec!(a);
}
