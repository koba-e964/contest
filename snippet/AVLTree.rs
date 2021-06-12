// http://www.prefield.com/algorithm/container/avl_tree.html
// https://atcoder.jp/contests/ttpc2019/tasks/ttpc2019_h
// https://atcoder.jp/contests/ttpc2019/submissions/23366439
#[derive(Clone)]
struct Node {
    p: i64,
    x: i64,
    xsum: i64,
    pxsum: i64,
    size: usize,
    height: i32,
    ch: [Option<Box<Node>>; 2],
}

impl Node {
    fn new(p: i64, x: i64) -> Self {
        Node {
            p: p,
            x: x,
            xsum: x,
            pxsum: p * x,
            size: 1,
            height: 1,
            ch: [None, None],
        }
    }
    // Insert a single element represented as a leaf node.
    fn insert(l: Option<Box<Node>>, x: Node) -> Option<Box<Node>> {
        let mut t = match l {
            None => return Some(Box::new(x)),
            Some(l) => l
        };
        if t.p <= x.p {
            t.ch[0] = Node::insert(t.ch[0].take(), x);
        } else {
            t.ch[1] = Node::insert(t.ch[1].take(), x);
        }
        Node::cons(&mut t);
        Some(Node::balance(t))
    }
    fn len(t: &Option<Box<Node>>) -> usize {
        if let &Some(ref x) = t {
            x.size
        } else {
            0
        }
    }
    fn ht(t: &Option<Box<Node>>) -> i32 {
        if let &Some(ref x) = t {
            x.height
        } else {
            0
        }
    }
    fn xsum(t: &Option<Box<Node>>) -> i64 {
        if let &Some(ref x) = t {
            x.xsum
        } else {
            0
        }
    }
    fn pxsum(t: &Option<Box<Node>>) -> i64 {
        if let &Some(ref x) = t {
            x.pxsum
        } else {
            0
        }
    }
    // Make t consistent.
    fn cons(t: &mut Box<Self>) {
        t.xsum = t.x;
        t.pxsum = t.p * t.x;
        for i in 0..2 {
            if let &Some(ref c) = &t.ch[i] {
                t.xsum += c.xsum;
                t.pxsum = t.pxsum.saturating_add(c.pxsum);
                assert!(if i == 0 { t.p <= c.p } else { t.p >= c.p }, "i = {}. t.p = {}, c.p = {}", i, t.p, c.p);
            }
        }
        t.height = std::cmp::max(Node::ht(&t.ch[0]), Node::ht(&t.ch[1])) + 1;
        t.size = 1 + Node::len(&t.ch[0]) + Node::len(&t.ch[1]);
    }
    fn rotate(mut t: Box<Self>, l: usize, r: usize) -> Box<Self> {
        let mut s = t.ch[r].take().unwrap();
        t.ch[r] = s.ch[l].take();
        Node::cons(&mut t);
        s.ch[l] = Some(Self::balance(t));
        Node::cons(&mut s);
        Self::balance(s)
    }
    fn balance(mut t: Box<Self>) -> Box<Self> {
        for i in 0..2 {
            if Self::ht(&t.ch[1 - i]) - Self::ht(&t.ch[i]) < -1 {
                let tmp = t.ch[i].as_mut().unwrap();
                if Self::ht(&tmp.ch[1 - i]) - Self::ht(&tmp.ch[i]) > 0 {
                    *tmp = 
                        Self::rotate(std::mem::replace(tmp, Box::new(Node::new(0, 0))), i, 1 - i);
                }
                return Self::rotate(t, 1 - i , i);
            }
        }
        Node::cons(&mut t);
        t
    }
    fn each<F: FnMut((i64, i64))>(t: Option<Box<Self>>, f: &mut F) {
        let mut t = match t {
            None => return,
            Some(t) => t,
        };
        Node::each(t.ch[0].take(), f);
        f((t.p, t.x));
        Node::each(t.ch[1].take(), f);
    }
    fn profit(t: &Option<Box<Self>>, mut s: i64) -> i64 {
        if t.is_none() {
            return 0;
        }
        if Node::xsum(&t) <= s {
            return Node::pxsum(&t);
        }
        let t = t.as_ref().unwrap();
        let mut ans = 0;
        if Node::xsum(&t.ch[0]) >= s {
            return Node::profit(&t.ch[0], s);
        }
        ans += Node::pxsum(&t.ch[0]);
        s -= Node::xsum(&t.ch[0]);
        let r = std::cmp::min(s, t.x);
        ans += t.p * r;
        s -= r;
        ans += Node::profit(&t.ch[1], s);
        ans
    }
}

#[derive(Clone)]
struct AVLTree {
    root: Option<Box<Node>>,
}

impl AVLTree {
    fn new() -> Self {
        AVLTree { root: None }
    }
    fn len(&self) -> usize {
        Node::len(&self.root)
    }
    fn insert(&mut self, p: i64, x: i64) {
        self.root = Node::insert(self.root.take(), Node::new(p, x));
    }
    // The order is not specified.
    fn each<F: FnMut((i64, i64))>(self, mut f: F) {
        Node::each(self.root, &mut f)
    }
    fn profit(&self, s: i64) -> i64 {
        Node::profit(&self.root, s)
    }
}
