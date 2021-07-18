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

// This function returns a Vec consisting of the distances from vertex source.
// This function is especially useful under low-memory constraints
// where storing all edges is not feasible.
fn solve_dijkstra<F: Fn(usize) -> Vec<(usize, i32)>>(
    n: usize, source: usize, inf: i32, f: F) -> Vec<i32> {
    let mut d = vec![inf; n];
    // que holds (-distance, vertex), so that que.pop() returns the nearest element.
    let mut que = std::collections::BinaryHeap::new();
    que.push((0, source));
    while let Some((cost, pos)) = que.pop() {
        let cost = -cost;
        if d[pos] <= cost {
            continue;
        }
        d[pos] = cost;
        for &(w, c) in &f(pos) {
            let w = w as usize;
            let newcost = cost + c;
            if d[w] > newcost {
                d[w] = newcost + 1;
                que.push((-newcost, w));
            }
        }
    }
    return d;
}

// Tags: low-memory, extended-dijkstra, graph-without-explicit-edges
fn main() {
    input! {
        n: usize, m: usize, x: usize,
        t: [i32; n],
        abc: [(usize1, usize1, usize); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let f = |v: usize| {
        let mut ans = vec![];
        let a = v / (2 * x + 1);
        let i = v % (2 * x + 1);
        for &(b, c) in &g[a] {
            if i < 2 * x {
                let kind = i / x;
                let i = i % x;
                let to = std::cmp::min(x, i + c);
                if to == x {
                    let dest = b * (2 * x + 1) + if t[b] == 1 {
                        2 * x
                    } else if t[b] == 0 {
                        0
                    } else {
                        x
                    };
                    if t[a] != 2 && kind == 0 {
                        ans.push((dest, c as _));
                    }
                    if t[a] != 0 && kind == 1 {
                        ans.push((dest, c as _));
                    }
                } else {
                    if t[b] != 2 && t[a] != 2 && kind == 0 {
                        let dest = b * (2 * x + 1) + if t[b] == 1 {
                            to
                        } else {
                            0
                        };
                        ans.push((dest, c as _));
                    }
                    if t[b] != 0 && t[a] != 0 && kind == 1 {
                        let dest = b * (2 * x + 1) + if t[b] == 1 {
                            to + x
                        } else {
                            x
                        };
                        ans.push((dest, c as _));
                    }
                }
            }
            if i == 2 * x && t[a] == 1 {
                let dest = b * (2 * x + 1) + if t[b] == 1 {
                    2 * x
                } else if t[b] == 0 {
                    0
                } else {
                    x
                };
                ans.push((dest, c as _));
            }
        }
        ans
    };
    let sol = solve_dijkstra(n * (2 * x + 1), 0, 1 << 30, f);
    println!("{}", sol[(n - 1) * (2 * x + 1)..].iter().min().unwrap());
}
