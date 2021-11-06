use std::cmp::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

const INF: i64 = 1 << 50;

// (max, -inf)
struct Dyn2DSegTree {
    n: i64,
    root: Node,
    e: i64,
}

struct Node {
    val: i64,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

impl Dyn2DSegTree {
    fn new(n: i64) -> Self {
        assert_eq!(n & -n, n);
        Dyn2DSegTree {
            n: n,
            root: Node::new(-INF),
            e: -INF,
        }
    }
    // O(log^2 n)
    fn query(&self, x: std::ops::Range<i64>, y: std::ops::Range<i64>) -> i64 {
        self.query_sub(Some(&self.root), x.start, x.end, 0, self.n, y.start, y.end, 0, self.n)
    }
    fn query_sub(&self, node: Option<&Node>,
                 lx: i64, rx: i64, ax: i64, bx: i64,
                 ly: i64, ry: i64, ay: i64, by: i64) -> i64 {
        let node = if let Some(node) = node {
            node
        } else {
            return self.e;
        };
        if rx <= ax || bx <= lx || ry <= ay || by <= ly {
            return self.e;
        }
        if lx <= ax && bx <= rx && ly <= ay && by <= ry {
            return node.val;
        }
        if bx - ax >= by - ay {
            // bisect on x
            assert!(bx - ax >= 2);
            let mid = (bx + ax) / 2;
            let p = self.query_sub(node.l.as_deref(), lx, rx, ax, mid, ly, ry, ay, by);
            let q = self.query_sub(node.r.as_deref(), lx, rx, mid, bx, ly, ry, ay, by);
            return std::cmp::max(p, q);
        }
        // bisect on y
        assert!(by - ay >= 2);
        let mid = (by + ay) / 2;
        let p = self.query_sub(node.l.as_deref(), lx, rx, ax, bx, ly, ry, ay, mid);
        let q = self.query_sub(node.r.as_deref(), lx, rx, ax, bx, ly, ry, mid, by);
        std::cmp::max(p, q)
    }
    fn update(&mut self, x: i64, y: i64, val: i64) {
        let n = self.n;
        Self::update_sub(&mut self.root, x, y, val, self.e, 0, n, 0, n);
    }
    fn update_sub(node: &mut Node, x: i64, y: i64, val: i64, e: i64,
                  ax: i64, bx: i64, ay: i64, by: i64) {
        if x < ax || bx <= x || y < ay || by <= y {
            unreachable!();
        }
        if (ax, bx) == (x, x + 1) && (ay, by) == (y, y + 1) {
            node.val = val;
            return;
        }
        if bx - ax >= by - ay {
            // bisect on x
            assert!(bx - ax >= 2);
            let mid = (bx + ax) / 2;
            if x >= mid {
                if node.r.is_none() {
                    node.r = Some(Box::new(Node::new(e)));
                }
                Self::update_sub(node.r.as_mut().unwrap(), x, y, val, e,
                                 mid, bx, ay, by);
            } else {
                if node.l.is_none() {
                    node.l = Some(Box::new(Node::new(e)));
                }
                Self::update_sub(node.l.as_mut().unwrap(), x, y, val, e,
                                 ax, mid, ay, by);
            }
        } else {
            // bisect on y
            assert!(by - ay >= 2);
            let mid = (by + ay) / 2;
            if y >= mid {
                if node.r.is_none() {
                    node.r = Some(Box::new(Node::new(e)));
                }
                Self::update_sub(node.r.as_mut().unwrap(), x, y, val, e,
                                 ax, bx, mid, by);
            } else {
                if node.l.is_none() {
                    node.l = Some(Box::new(Node::new(e)));
                }
                Self::update_sub(node.l.as_mut().unwrap(), x, y, val, e,
                                 ax, bx, ay, mid);
            }
        }
        let p = node.l.as_ref().map(|v| v.val).unwrap_or(e);
        let q = node.r.as_ref().map(|v| v.val).unwrap_or(e);
        node.val = std::cmp::max(p, q);
    }
}

impl Node {
    fn new(e: i64) -> Self {
        Node {
            val: e,
            l: None,
            r: None,
        }
    }
}

fn parse(x: [i64; 6]) -> (i64, i64, i64) {
    let l = min(min(x[0], x[2]), x[4]);
    let r = max(max(x[0], x[2]), x[4]);
    let a = x[2] - x[0];
    let b = x[3] - x[1];
    let c = x[4] - x[0];
    let d = x[5] - x[1];
    (l, r, (a * d - b * c).abs())
}

// The author read the editorial before implementing this.
// Tags: 2d-segment-trees, dynamic-segment-trees, coordinate-compression
fn main() {
    let n: usize = get();
    let q: usize = get();
    let mut coo = vec![];
    let mut orig = vec![];
    for _ in 0..n {
        let mut x = [0i64; 6];
        for i in 0..6 {
            x[i] = get();
        }
        let (l, r, a) = parse(x);
        coo.push(l);
        coo.push(r);
        orig.push((l, r, a));
    }
    let mut qs = vec![];
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let mut x = [0i64; 6];
            for i in 0..6 {
                x[i] = get();
            }
            let (l, r, a) = parse(x);
            coo.push(l);
            coo.push(r);
            qs.push(Err((l, r, a)));
        } else {
            let l: i64 = get();
            let r: i64 = get();
            coo.push(l);
            coo.push(r + 1);
            qs.push(Ok((l, r + 1)));
        }
    }
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut sz = 1;
    while m > sz {
        sz *= 2;
    }
    let mut st = Dyn2DSegTree::new(sz as i64);
    for (l, r, a) in orig {
        let l = coo.binary_search(&l).unwrap() as i64;
        let r = coo.binary_search(&r).unwrap() as i64;
        let old = st.query(l..l + 1, r..r + 1);
        if old < a {
            st.update(l, r, a);
        }
    }
    for q in qs {
        match q {
            Err((l, r, a)) => {
                let l = coo.binary_search(&l).unwrap() as i64;
                let r = coo.binary_search(&r).unwrap() as i64;
                let old = st.query(l..l + 1, r..r + 1);
                if old < a {
                    st.update(l, r, a);
                }
            }
            Ok((l, r)) => {
                let l = coo.binary_search(&l).unwrap() as i64;
                let r = coo.binary_search(&r).unwrap() as i64;
                println!("{}", max(st.query(l..r, l..r), -1));
            }
        }
    }
}
