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
        n: usize, k: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
    }
    let mut a = a;
    let mut b = b;
    let mut c = c;
    a.sort_unstable(); a.reverse();
    b.sort_unstable(); b.reverse();
    c.sort_unstable(); c.reverse();
    let mut que = BinaryHeap::new();
    let mut seen = HashSet::new();
    que.push((a[0] * b[0] + b[0] * c[0] + a[0] * c[0], 0, 0, 0));
    let mut rem = k as i32;
    while rem > 0 {
        let (score, i, j, k) = que.pop().unwrap();
        if !seen.insert((i, j, k)) {
            continue;
        }
        for &(di, dj, dk) in &[(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            let ni = i + di;
            let nj = j + dj;
            let nk = k + dk;
            if ni < n && nj < n && nk < n {
                que.push((a[ni] * b[nj] + b[nj] * c[nk] + a[ni] * c[nk], ni, nj, nk));
            }
        }
        rem -= 1;
        if rem == 0 {
            println!("{score}");
        }
    }
}
