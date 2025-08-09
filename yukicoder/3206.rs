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

// Copied from: <https://atcoder.jp/contests/abc160/submissions/11638664>
// Authored by: [vain0](https://atcoder.jp/users/vain0)
// Ref: <https://qiita.com/keymoon/items/2a52f1b0fb7ef67fb89e>
fn rerooting<
    T: Clone,
    E: IntoIterator<Item = (usize, usize)>,
    F: FnMut(T, T) -> T,
    G: FnMut(T, usize) -> T,
>(
    node_count: usize,
    edges: E,
    identity: T,
    mut operate: F,
    mut operate_node: G,
) -> Vec<T> {
    const NO_PARENT: usize = std::usize::MAX;

    let mut adjacents = vec![vec![]; node_count];
    let mut index_for_adjacents = vec![vec![]; node_count];

    for (u, v) in edges {
        index_for_adjacents[u].push(adjacents[v].len());
        index_for_adjacents[v].push(adjacents[u].len());
        adjacents[u].push(v);
        adjacents[v].push(u);
    }

    if node_count == 0 {
        return vec![];
    }

    if node_count == 1 {
        return vec![operate_node(identity, 0)];
    }

    let mut parents = vec![0; node_count];
    let mut order = vec![0; node_count];

    // initialize ordered tree
    {
        let mut index = 0;
        let mut stack = vec![0];
        parents[0] = NO_PARENT;

        while let Some(node) = stack.pop() {
            order[index] = node;
            index += 1;

            for i in 0..adjacents[node].len() {
                let adjacent = adjacents[node][i];
                if adjacent == parents[node] {
                    continue;
                }

                stack.push(adjacent);
                parents[adjacent] = node;
            }
        }
    }

    let mut dp = (0..node_count)
        .map(|i| vec![identity.clone(); adjacents[i].len()])
        .collect::<Vec<_>>();

    // from leaf
    for i in (1..node_count).rev() {
        let node = order[i];
        let parent = parents[node];

        let mut accum = identity.clone();
        let mut parent_index = NO_PARENT;

        for j in 0..adjacents[node].len() {
            if adjacents[node][j] == parent {
                parent_index = j;
                continue;
            }

            accum = operate(accum, dp[node][j].clone());
        }

        dp[parent][index_for_adjacents[node][parent_index]] = operate_node(accum, node);
    }

    let mut res = vec![identity.clone(); node_count];
    let mut accums_from_tail = vec![];

    // to leaf
    for i in 0..node_count {
        let node = order[i];
        let deg = adjacents[node].len();
        let mut accum = identity.clone();

        accums_from_tail.clear();
        accums_from_tail.extend(std::iter::repeat(identity.clone()).take(deg));

        for j in (1..deg).rev() {
            accums_from_tail[j - 1] = operate(accums_from_tail[j].clone(), dp[node][j].clone());
        }

        for j in 0..deg {
            dp[adjacents[node][j]][index_for_adjacents[node][j]] =
                operate_node(operate(accum.clone(), accums_from_tail[j].clone()), node);
            accum = operate(accum, dp[node][j].clone());
        }

        res[node] = operate_node(accum, node);
    }

    res
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// https://yukicoder.me/problems/no/3206 (3.5)
// 全方位木 DP で、それぞれの部分根付き木に対して一番深い葉の深さを計算する。
// そのあとは O(\sum deg[i]) = O(N) 時間で計算できる。
fn solve() {
    input! {
        n: usize,
        uv: [(usize1, usize1); n - 1],
    }
    let res = rerooting(
        n,
        uv,
        // TODO: clarify
        vec![-1],
        |mut a, b| {
            a.push(b.iter().max().cloned().unwrap_or(-1));
            a
        },
        |mut a, _| {
            for v in a.iter_mut() {
                *v += 1;
            }
            a
        },
    );
    let mut ans = 1;
    for i in 0..n {
        let mut v = res[i][1..].to_vec();
        v.sort_unstable(); v.reverse();
        for i in 0..v.len() {
            ans = ans.max(v[i] * (i + 1) as i64 + 1);
        }
    }
    println!("{ans}");
}
