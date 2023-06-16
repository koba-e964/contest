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

fn dist(g: &[Vec<(usize, i64)>], a: usize) -> Vec<i32> {
    const INF: i32 = 1 << 28;
    let mut dist = vec![INF; g.len()];
    let mut que = VecDeque::new();
    que.push_back((0, a));
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        for &(w, _) in &g[v] {
            que.push_back((d + 1, w));
        }
    }
    dist
}

// https://yukicoder.me/problems/no/1833 (3.5)
// 二分探索をすると、重みが　 x を超えている辺があるパスに含まれているか、そのパスを選べば条件を満たせるかを判定する問題になる。
// あるパスに含まれるかどうかは、対象の辺に含まれる頂点だけで直径を求めればわかる。
fn main() {
    input! {
        n: usize,
        abc: [(usize1, usize1, i64); n - 1],
    }
    let mut g = vec![vec![]; n];
    let mut cmax = 0;
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        g[b].push((a, c));
        cmax = max(cmax, c);
    }
    let mut pass = cmax;
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mut on = vec![false; n];
        let mut some = n;
        let mut ec = 0;
        for &(a, b, c) in &abc {
            if c > mid {
                on[a] = true;
                on[b] = true;
                some = a;
                ec += 1;
            }
        }
        let d1 = dist(&g, some);
        let mut x = (0, 0);
        for i in 0..n {
            if on[i] {
                x = max(x, (d1[i], i));
            }
        }
        let d2 = dist(&g, x.1);
        let mut y = (0, 0);
        for i in 0..n {
            if on[i] {
                y = max(y, (d2[i], i));
            }
        }
        let mut mi = cmax;
        let mut rem = d2[y.1];
        let mut cur = y.1;
        let mut ec_on_path = 0;
        while rem > 0 {
            let mut nxt = n;
            for &(w, c) in &g[cur] {
                if d2[w] == rem - 1 {
                    if c > mid {
                        ec_on_path += 1;
                    }
                    mi = min(mi, c);
                    nxt = w;
                    break;
                }
            }
            cur = nxt;
            rem -= 1;
        }
        if ec_on_path == ec && cmax - mi <= mid {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
