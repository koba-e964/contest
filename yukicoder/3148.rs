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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

#[derive(Eq, Clone, Copy, Debug)]
struct AB {
    a: i64,
    b: i64,
}
impl std::cmp::PartialEq for AB {
    fn eq(&self, &Self { a: oa, b: ob }: &Self) -> bool {
        self.a * ob == oa * self.b
    }
}
impl std::cmp::PartialOrd for AB {
    fn partial_cmp(&self, &Self { a: oa, b: ob }: &Self) -> Option<std::cmp::Ordering> {
        (self.a * ob).partial_cmp(&(oa * self.b))
    }
}
impl std::cmp::Ord for AB {
    fn cmp(&self, &Self { a: oa, b: ob }: &Self) -> std::cmp::Ordering {
        (self.a * ob).cmp(&(oa * self.b))
    }
}

fn dfs1(s: &[char], bank: &mut usize, p: &mut [usize]) -> (usize, usize) {
    assert_eq!(s[0], '(');
    let me = *bank;
    *bank += 1;
    let mut cur = 1;
    while cur < s.len() && s[cur] == '(' {
        let (ch, pos) = dfs1(&s[cur..], bank, p);
        p[ch] = me;
        cur += pos;
    }
    assert_eq!(s[cur], ')');
    (me, cur + 1)
}

fn topsort_opt_tree(mut ch: Vec<BTreeSet<usize>>, mut p: Vec<usize>, a: &[i64]) -> i64 {
    let n = ch.len();
    let mut que = BinaryHeap::new();
    let mut abs = vec![AB { a: 0, b: 1 }; n];
    for i in 1..n {
        abs[i] = AB { a: a[i - 1], b: 1 };
        que.push((abs[i], i));
    }
    let mut cost = 0;
    let mut nvert = n;
    while let Some((alleged, idx)) = que.pop() {
        if abs[idx].a != alleged.a || abs[idx].b != alleged.b {
            continue;
        }
        let par = p[idx];

        let merged = AB {
            a: abs[par].a + abs[idx].a,
            b: abs[par].b + abs[idx].b,
        };
        cost += abs[par].b * abs[idx].a;

        let (newidx, other);
        if ch[par].len() > ch[idx].len() {
            (newidx, other) = (par, idx);
            // up
            // -1 <-> 0 => -1 <-> 1
            ch[par].remove(&idx);
            // 0 <-> 1 => -1 <-> 1
            let tmp = std::mem::take(&mut ch[idx]);
            for v in tmp {
                ch[par].insert(v);
                p[v] = par;
            }
        } else {
            (newidx, other) = (idx, par);
            // down
            // -2 <-> -1 => -2 <-> 0
            let parpar = p[par];
            if parpar != n {
                ch[parpar].insert(idx);
                ch[parpar].remove(&par);
            }
            p[idx] = parpar;
            // -1 <-> 0 => 0 <-> 0
            let tmp = std::mem::take(&mut ch[par]);
            for v in tmp {
                if v == idx {
                    continue;
                }
                ch[idx].insert(v);
                p[v] = idx;
            }
        }
        p[other] = n;
        abs[newidx] = merged;
        abs[other] = AB { a: 0, b: 0 };
        if p[newidx] != n {
            que.push((merged, newidx));
        }
        assert!(ch[other].is_empty());
        nvert -= 1;
        if nvert == 1 {
            break;
        }
    }
    cost
}

// https://yukicoder.me/problems/no/3148 (3.5)
// 逆向きに考えると以下の問題になる。
// 根付き木のトポロジカルソート (根から子へ) を行い、最終的に得られた配列を B とすると 1 * B[0] + ... + N * B[N-1] がコストである。コストを最小化せよ。
// トポロジカルソート、ただし候補の中で最大のものから順番に見る、でできそうに思えるが、例えば A = (1, 10, 10, 1, 100), p = (None, 0, 1, 0, 3) のときに 100 を最後まで待つ必要があり困りそう。
// 部分木の解をマージする時に O(st)-time かけてよいのであれば、全体では O(N^2)-time で解ける。
// O(st)-time のマージは単に DP するだけでよい。
// (60) と (1, 100) をマージする時は 60 は左にあった方がよく、(10) と (1, 100) のときは 10 は右にあった方が良い。
// これは 10 * 2 < 1 + 100 < 60 * 2 であるため。
// ただ、平均値だけで見るのも間違いで、例えば (10) と (1, 100, 1) は (1, 100, 10, 1) になる。
// ヒントを見た。https://atcoder.jp/contests/agc023/tasks/agc023_f の類題で、同じようなやり方で解ける。
// 以下のような問題になる: 木の頂点ごとに A, B という値があり、頂点のトポロジカルソートをする。(A1, B1) と (A2, B2) をこの順番にするとコストが B1 * A2 かかるときコストを最小化せよ。
// AGC023-F のように、 A/B が最大の頂点について親とマージしていくことになる。これはマージテクと優先度キューを使えば O(N log N) になる。
// -> AC。 AGC023-F の解説にあったように、マージテクではなく UnionFind を使った方が楽だった。
// solved with hints
// Similar problems: https://atcoder.jp/contests/agc023/tasks/agc023_f
// Tags: topological-sort-on-trees, weighted-union-heuristic
fn solve() {
    input! {
        n: usize,
        s: chars,
        a: [i64; n],
    }
    let mut s = s;
    s.insert(0, '(');
    s.push(')');
    let mut p = vec![0; n + 1];
    p[0] = n + 1;
    dfs1(&s, &mut 0, &mut p);
    let mut ch = vec![BTreeSet::new(); n + 1];
    for i in 1..=n {
        ch[p[i]].insert(i);
    }

    println!("{}", topsort_opt_tree(ch, p, &a));
}
