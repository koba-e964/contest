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

// Solved with hints
// https://yukicoder.me/problems/no/2321 (4)
// 取る区間の集合 X に対する利得を f(X) とすると、劣モジュラー性 f(X) + f(Y) >= f(X \/ Y) + f(X /\ Y) が成り立たない。
// 例えば X = {1,2}, Y = {1,3} として区間 1, 2, 3 すべてが同じものを指しているときが反例である。
// -> ヒントを見た。最短路で解ける。
// 問題を全て表の状態からの差分 (裏返しに必要なコスト + 裏になってしまったものの合計) の最小化に読み替え、
// さらに i 番目が裏になることを区間 [i, i+1) をコスト C で裏返すことと同一視する。
// 表になっているかどうかの mod 2 の配列の差分配列 d を考えることにすると、区間 [l, r) を裏返すことは d[l] += 1, d[r] += 1 と同じ。最終的な目標は d[0] = d[n] = 1, 他の要素がすべて 0 であることである。
// このような問題はグラフの最短路問題に帰着できる。(mod 2 での条件下のコスト最小化はおそらくこのくらいでしか線形計画問題に帰着できないように思える。)
fn main() {
    input! {
        n: usize, m: usize, c: i64,
        a: [i64; n],
        lr: [(usize1, usize); m],
    }
    let mut g = vec![vec![]; n + 1];
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
        g[i].push((i + 1, a[i]));
        g[i + 1].push((i, a[i]));
    }
    for &(l, r) in &lr {
        g[l].push((r, c));
        g[r].push((l, c));
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; n + 1];
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0));
    while let Some((Reverse(d), v)) = que.pop() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        for &(w, c) in &g[v] {
            que.push((Reverse(d + c), w));
        }
    }
    println!("{}", sum - dist[n]);
}
