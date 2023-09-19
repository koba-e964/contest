use std::cmp::*;
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

fn divide_into_segment_ranges<F: FnMut(usize)>(n: usize, rng: std::ops::Range<usize>, mut f: F) {
    assert!(n.is_power_of_two());
    let (mut a, mut b) = (rng.start, rng.end);
    a += n - 1;
    b += n - 1;
    while a < b {
        if (a & 1) == 0 {
            f(a);
        }
        if (b & 1) == 0 {
            f(b - 1);
        }
        a = a / 2;
        b = (b - 1) / 2;
    }
}

// 範囲に辺を張りたいので、重み 0 の辺を使って ->->-> という流れを作ってそれの上に辺を張ることにする。
// -> それだと (a, b) に対して複数の c がある場合にうまくいかないので、代わりにセグメント木を使う。
// Tags: manual-segment-trees
fn main() {
    input! {
        n: usize, m: usize,
        uvw: [(usize1, usize1, i64); m],
        k: usize,
        abc: [(usize1, usize1, usize1); k],
    }
    let mut g = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        g[u].push((v, w));
    }
    let mut offset = vec![0; n + 1];
    for i in 0..n {
        g[i].sort();
        offset[i + 1] = offset[i] + g[i].len();
    }
    let mut map = vec![HashMap::new(); n];
    for &(a, b, c) in &abc {
        if let Ok(idx) = g[b].binary_search_by_key(&c, |x| x.0) {
            map[a].entry(b).or_insert(vec![]).push(idx);
        }
    }
    for i in 0..n {
        for e in map[i].values_mut() {
            e.sort();
        }
    }
    let m2 = m.next_power_of_two();
    let mut sup = vec![vec![]; 2 * m2]; // segment tree
    for i in 0..m2 - 1 {
        sup[i].push((2 * i + 1, 0));
        sup[i].push((2 * i + 2, 0));
    }
    for a in 0..n {
        for i in 0..g[a].len() {
            let (b, w) = g[a][i];
            let idx = offset[a] + i;
            if let Some(v) = map[a].get(&b) {
                if !v.is_empty() {
                    for i in 0..v.len() - 1 {
                        divide_into_segment_ranges(m2, offset[b] + v[i] + 1..offset[b] + v[i + 1], |x| sup[m2 + idx - 1].push((x, w)));
                    }
                    divide_into_segment_ranges(m2, offset[b]..offset[b] + v[0], |x| sup[m2 + idx - 1].push((x, w)));
                    divide_into_segment_ranges(m2, offset[b] + v[v.len() - 1] + 1..offset[b + 1], |x| sup[m2 + idx - 1].push((x, w)));
                }
            } else {
                if b == n - 1 {
                    sup[m2 - 1 + idx].push((2 * m2 - 1, w));
                } else {
                    divide_into_segment_ranges(m2, offset[b]..offset[b + 1], |x| sup[m2 + idx - 1].push((x, w)));
                }
            }
        }
    }
    let mut que = BinaryHeap::new();
    for i in 0..g[0].len() {
        que.push((Reverse(0), m2 - 1 + i));
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; 2 * m2];
    while let Some((Reverse(d), v)) = que.pop() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        for &(w, c) in &sup[v] {
            que.push((Reverse(d + c), w));
        }
    }
    let ans = dist[2 * m2 - 1];
    println!("{}", if ans >= INF { -1 } else { ans });
}
