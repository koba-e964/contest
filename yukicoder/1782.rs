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

// https://yukicoder.me/problems/no/1782 (3.5)
// 適当に W の要素である整数 x (W_i の min など) を取り、重みの和 mod x ごとにそれを実現する最小の重みを持てばできそう。(最小値が意味を持つために、x が実現できる正整数であることが必要。)
// dp[(i + W[i]) % x] <- dp[i] + W[i] という遷移を、遷移で値が変わらなくなるまでする必要があるので、この dp は対応する辺を張った有向グラフにおける最短路問題として解くことができる。
// 初期値は dp[0] = 0 ではなく dp[W[i] % x] = W[i] としなければならないことに注意。
// このグラフの頂点数は x、辺の本数は Nx である。
// -> Dijkstra で通ったがテストケースの弱さ頼み。dp の値として x での商をとることにすれば
// 01BFS で可能だった。
// Tags: dp
fn main() {
    input! {
        n: usize, l: i64,
        w: [i64; n],
    }
    let x = *w.iter().min().unwrap() as usize;
    let mut rem = vec![0; n];
    for i in 0..n {
        rem[i] = (w[i] % x as i64) as usize;
    }
    let mut que = BinaryHeap::new();
    for i in 0..n {
        que.push((-w[i], rem[i]));
    }
    let mut d = vec![l + 1; x];
    while let Some((cost, pos)) = que.pop() {
        let cost = -cost;
        if d[pos] <= cost {
            continue;
        }
        d[pos] = cost;
        for i in 0..n {
            let (w, c) = ((pos + rem[i]) % x, w[i]);
            let newcost = cost + c;
            if d[w] > newcost {
                d[w] = newcost + 1;
                que.push((-newcost, w));
            }
        }
    }
    let mut ans = 0;
    for i in 0..x {
        if d[i] > l { continue; }
        ans += (l - d[i]) / x as i64 + 1;
    }
    println!("{}", ans);
}
