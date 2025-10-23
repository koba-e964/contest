fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

type Q = (usize, usize, usize, usize);

fn add(nodes: &mut [Vec<Q>], a: usize, b: usize, l: usize, r: usize, idx: usize, q: Q) {
    if r - l == 1 {
        panic!();
    }
    let mid = (l + r) / 2;
    if a <= mid && b >= mid {
        nodes[idx].push(q);
        return;
    }
    if b < mid {
        add(nodes, a, b, l, mid, 2 * idx + 1, q);
    } else {
        add(nodes, a, b, mid, r, 2 * idx + 2, q);
    }
}

fn rec(nodes: &[Vec<Q>], l: usize, r: usize, idx: usize, wv: &[(usize, i64)], ans: &mut [i64]) {
    if r - l == 1 {
        return;
    }
    let mid = (l + r) / 2;
    let half = r - mid;
    const T: usize = 501;
    let mut lft = vec![vec![0; T]; half + 1];
    let mut rgt = vec![vec![0; T]; half + 1];
    for i in 0..half {
        let (w, v) = wv[mid - i - 1];
        for j in 0..T - w {
            lft[i + 1][j + w] = lft[i + 1][j + w].max(lft[i][j] + v);
        }
        for j in 0..T {
            lft[i + 1][j] = lft[i + 1][j].max(lft[i][j]);
        }
    }
    for i in 0..half {
        let (w, v) = wv[mid + i];
        for j in 0..T - w {
            rgt[i + 1][j + w] = rgt[i + 1][j + w].max(rgt[i][j] + v);
        }
        for j in 0..T {
            rgt[i + 1][j] = rgt[i + 1][j].max(rgt[i][j]);
        }
    }
    for &(a, b, c, qidx) in &nodes[idx] {
        let mut me = 0;
        for i in 0..=c {
            me = me.max(lft[mid - a][i] + rgt[b - mid][c - i]);
        }
        ans[qidx] = me;
    }
    rec(nodes, l, mid, 2 * idx + 1, wv, ans);
    rec(nodes, mid, r, 2 * idx + 2, wv, ans);
}

// The author read the editorial before implementing this.
// Tags: divide-and-conquer-rsq, range-static-query, manual-segment-trees
fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut wv = vec![];
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        wv.push((ints[0] as usize, ints[1]));
    }
    const W: usize = 1 << 15;
    while wv.len() < W {
        wv.push((0, 0));
    }
    let q = getline().trim().parse::<usize>().unwrap();
    let mut nodes = vec![vec![]; 2 * W - 1];
    for i in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        add(&mut nodes, ints[0] - 1, ints[1], 0, W, 0, (ints[0] - 1, ints[1], ints[2], i));
    }
    if q <= 10 {
        for i in 0..nodes.len() {
            if !nodes[i].is_empty() {
                eprintln!("{i} => {:?}", nodes[i]);
            }
        }
    }
    let mut ans = vec![-1; q];
    rec(&nodes, 0, W, 0, &wv, &mut ans);
    for i in 0..q {
        println!("{}", ans[i]);
    }
}
