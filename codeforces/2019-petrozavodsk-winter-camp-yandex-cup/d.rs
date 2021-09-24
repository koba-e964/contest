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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[derive(Clone, Default)]
struct BinaryMat {
    basis: Vec<i64>,
}

impl BinaryMat {
    // O(1)
    fn new() -> Self {
        Default::default()
    }
    // O(rank)
    fn sift(&self, mut x: i64) -> i64 {
        for &b in &self.basis {
            x = std::cmp::min(x, x ^ b);
            if x == 0 {
                return 0;
            }
        }
        x
    }
    // O(rank)
    fn add(&mut self, x: i64) {
        let x = self.sift(x);
        if x != 0 {
            self.basis.push(x);
        }
    }
    // O(1)
    #[allow(unused)]
    fn rank(&self) -> usize {
        self.basis.len()
    }
    // O(rank)
    #[allow(unused)]
    fn is_indep(&self, x: i64) -> bool {
        self.sift(x) != 0
    }
}

// O(r^3 \sum k_i), presumably O(r^3 + r^1.5 \sum k_i)
// a must be linearly independent
fn find_augmenting_path(
    a: &[i64], b: &[i64], c: &[usize],
    m: usize,
    sidx: &[usize],
) -> Vec<usize> {
    let sk = b.len(); // \sum k_i
    let mut ins = vec![false; sk];
    let mut s = vec![];
    for &v in sidx {
        s.push(b[v]);
        ins[v] = true;
    }
    let mut whole = BinaryMat::new();
    let mut minus1 = vec![BinaryMat::new(); sidx.len()];
    // O(r^3)
    for i in 0..sidx.len() {
        for j in 0..sidx.len() {
            if i == j { continue; }
            minus1[i].add(s[j]);
        }
        for &a in a {
            minus1[i].add(a);
        }
    }
    for &s in &s {
        whole.add(s);
    }
    for &a in a {
        whole.add(a);
    }
    let mut compidx = vec![];
    for i in 0..sk {
        if !ins[i] {
            compidx.push(i);
        }
    }
    let mut g = vec![vec![]; sk];
    let mut is_y1 = vec![false; sk];
    // Y1, O(r \sum k_i)
    for &i in &compidx {
        if whole.is_indep(b[i]) {
            is_y1[i] = true;
        }
    }
    let mut y2 = vec![];
    // Y2, O(\sum k_i)
    let mut f = vec![vec![]; m];
    for &i in sidx {
        assert!(f[c[i]].is_empty());
        f[c[i]].push(i);
    }
    for &i in &compidx {
        if f[c[i]].is_empty() {
            y2.push(i);
        }
    }
    // O(\sum k_i)
    // Exchange edge in M2; from S to E - S
    for &j in &compidx {
        if !f[c[j]].is_empty() {
            let i = f[c[j]][0];
            g[i].push(j);
        }
    }
    // Find a shortest path from Y2 to Y1 using bfs, O(r \sum k_i) without computing edges
    const INF: i32 = 1 << 28;
    let mut dist = vec![(INF, 0); sk];
    let mut que = std::collections::VecDeque::new();
    let mut vis = vec![false; sk];
    for &v in &y2 {
        que.push_back((0, v, v));
        vis[v] = true;
    }
    let mut mi = (INF, 0);
    while let Some((d, v, pre)) = que.pop_front() {
        if dist[v].0 <= d {
            continue;
        }
        dist[v] = (d, pre);
        if is_y1[v] {
            mi = (d, v);
            break;
        }
        if ins[v] {
            for &w in &g[v] {
                que.push_back((d + 1, w, v));
            }
        } else {
            if whole.is_indep(b[v]) { continue; }
            for i in 0..sidx.len() {
                let w = sidx[i];
                if vis[w] {
                    continue;
                }
                if minus1[i].is_indep(b[v]) {
                    // Exchange edge in M1; from E - S to S
                    // presumably O(r^1.5 \sum k_i) in total
                    que.push_back((d + 1, w, v));
                    vis[w] = true;
                }
            }
        }
    }
    if mi.0 >= INF {
        return vec![];
    }
    // Recover the path from dist
    let mut cur = mi.1;
    let mut path = vec![];
    loop {
        path.push(cur);
        let pre = dist[cur].1;
        if pre == cur {
            break;
        }
        cur = pre;
    }
    path.reverse();
    path
}

fn matroid_intersection(
    a: &[i64], b: &[i64], c: &[usize],
    m: usize,
) -> Vec<usize> {
    let mut sidx = vec![];
    for _ in 0..m {
        let aug = find_augmenting_path(a, b, c, m, &sidx);
        if aug.is_empty() {
            break;
        }
        let l = aug.len() / 2;
        assert_eq!(aug.len(), 2 * l + 1);
        for i in 0..l {
            sidx.retain(|&v| v != aug[2 * i + 1]);
        }
        for i in 0..l + 1 {
            sidx.push(aug[2 * i]);
        }
    }
    sidx
}

// Solved with hints
// Tags: matroid-intersection
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        m: usize,
        b: [[i64]; m],
    }
    let mut mat = BinaryMat::new();
    for &a in &a {
        if !mat.is_indep(a) {
            println!("-1");
            return;
        }
        mat.add(a);
    }
    let mut flat = vec![];
    let mut c = vec![];
    for i in 0..m {
        for &w in &b[i] {
            flat.push(w);
            c.push(i);
        }
    }
    let mut ans = matroid_intersection(&a, &flat, &c, m);
    if ans.len() == m {
        ans.sort();
        for i in 0..m {
            assert_eq!(c[ans[i]], i);
            println!("{}", flat[ans[i]]);
        }
    } else {
        println!("-1");
    }
}
