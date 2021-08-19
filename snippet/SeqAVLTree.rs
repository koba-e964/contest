// http://www.prefield.com/algorithm/container/avl_tree.html
// https://atcoder.jp/contests/code-festival-2014-exhibition-open/tasks/code_festival_exhibition_b
// https://atcoder.jp/contests/code-festival-2014-exhibition-open/submissions/23724473
#[derive(Clone)]
struct Node<T, F> {
    val: T,
    sum: T,
    f: F,
    e: T,
    size: usize,
    height: i32,
    ch: [Option<Box<Self>>; 2],
}

impl<T: Default + Copy, F: Fn(T, T) -> T + Copy> Node<T, F> {
    fn new(val: T, f: F, e: T) -> Self {
        Node {
            val: val,
            sum: val,
            f: f,
            e: e,
            size: 1,
            height: 1,
            ch: [None, None],
        }
    }
    // Insert a single element represented as a leaf node.
    fn insert(l: Option<Box<Self>>, x: Self, pos: usize) -> Option<Box<Self>> {
        let mut t = match l {
            None => return Some(Box::new(x)),
            Some(l) => l
        };
        let llen = Self::len(&t.ch[0]);
        if pos <= llen {
            t.ch[0] = Node::insert(t.ch[0].take(), x, pos);
        } else {
            t.ch[1] = Node::insert(t.ch[1].take(), x, pos - 1 - llen);
        }
        Node::cons(&mut t);
        Some(Node::balance(t))
    }
    fn erase(t: Option<Box<Self>>, pos: usize) -> Option<Box<Self>> {
        let mut t = match t {
            None => return None,
            Some(l) => l
        };
        let llen = Self::len(&t.ch[0]);
        if pos < llen {
            t.ch[0] = Self::erase(t.ch[0].take(), pos);
        } else if pos == llen {
            return Self::move_down(t.ch[0].take(), t.ch[1].take());
        } else {
            t.ch[1] = Node::erase(t.ch[1].take(), pos - 1 - llen);
        }
        Node::cons(&mut t);
        Some(Node::balance(t))
        
    }
    fn move_down(l: Option<Box<Self>>, r: Option<Box<Self>>) -> Option<Box<Self>> {
        let mut t = match l {
            Some(t) => t,
            None => return r,
        };
        t.ch[1] = Self::move_down(t.ch[1].take(), r);
        Some(Self::balance(t))
    }
    fn len(t: &Option<Box<Self>>) -> usize {
        if let &Some(ref x) = t {
            x.size
        } else {
            0
        }
    }
    fn ht(t: &Option<Box<Self>>) -> i32 {
        if let &Some(ref x) = t {
            x.height
        } else {
            0
        }
    }
    // Make t consistent.
    fn cons(t: &mut Box<Self>) {
        t.sum = t.val;
        if let Some(ref ch) = t.ch[0] {
            t.sum = (t.f)(ch.sum, t.sum);
        }
        if let Some(ref ch) = t.ch[1] {
            t.sum = (t.f)(t.sum, ch.sum);
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
                        Self::rotate(std::mem::replace(tmp, Box::new(Node::new(T::default(), t.f, t.e))), i, 1 - i);
                }
                return Self::rotate(t, 1 - i , i);
            }
        }
        Node::cons(&mut t);
        t
    }
    fn each<F1: FnMut(T)>(t: Option<Box<Self>>, f: &mut F1) {
        let mut t = match t {
            None => return,
            Some(t) => t,
        };
        Node::each(t.ch[0].take(), f);
        f(t.val);
        Node::each(t.ch[1].take(), f);
    }
    fn range(t: &Box<Self>, l: usize, r: usize) -> T {
        if l == 0 && r >= t.size {
            return t.sum;
        }
        let llen = Self::len(&t.ch[0]);
        let mut left = t.e;
        let mut me = t.e;
        let mut right = t.e;
        if l < llen {
            left = Self::range(t.ch[0].as_ref().unwrap(), l, std::cmp::min(r, llen));
        }
        if l <= llen && r >= llen + 1 {
            me = t.val;
        }
        if r > llen + 1 {
            right = Self::range(t.ch[1].as_ref().unwrap(),
                                std::cmp::max(llen + 1, l) - llen - 1, r - llen - 1);
        }
        (t.f)((t.f)(left, me), right)
    }
    // Find min x s.t. p(t.range(0..x)), or |t| + 1 if there is no such x
    fn binary_search<M: Fn(T) -> bool>(t: &Option<Box<Self>>, cur: T, p: &M) -> usize {
        if p(cur) {
            return 0;
        }
        let t = if let Some(ref t) = t {
            t
        } else {
            return 1;
        };
        if !p((t.f)(cur, t.sum)) {
            return t.size + 1;
        }
        let sub = Node::binary_search(&t.ch[0], cur, p);
        let llen = Self::len(&t.ch[0]);
        if sub <= llen {
            return sub;
        }
        let lsum = if let Some(ref l) = t.ch[0] {
            l.sum
        } else {
            t.e
        };
        let tmp = (t.f)(lsum, t.val);
        if p(tmp) {
            return llen + 1;
        }
        llen + 1 + Self::binary_search(&t.ch[1], (t.f)(cur, tmp), p)
    }
}

#[derive(Clone)]
struct SeqAVLTree<T, F> {
    root: Option<Box<Node<T, F>>>,
    f: F,
    e: T,
}

impl<T: Default + Copy, F: Fn(T, T) -> T + Copy> SeqAVLTree<T, F> {
    fn new(f: F, e: T) -> Self {
        SeqAVLTree { root: None, f: f, e: e }
    }
    #[allow(unused)]
    fn len(&self) -> usize {
        Node::len(&self.root)
    }
    fn insert(&mut self, val: T, pos: usize) {
        self.root = Node::insert(self.root.take(), Node::new(val, self.f, self.e), pos);
    }
    #[allow(unused)]
    fn erase(&mut self, pos: usize) {
        self.root = Node::erase(self.root.take(), pos);
    }
    #[allow(unused)]
    fn each<F1: FnMut(T)>(self, mut f: F1) {
        Node::each(self.root, &mut f)
    }
    fn range(&self, rng: std::ops::Range<usize>) -> T {
        let l = rng.start;
        let r = rng.end;
        if let Some(ref x) = self.root {
            Node::range(x, l, r)
        } else {
            self.e
        }
    }
    // Find min x s.t. p(t.range(0..x)), or |t| + 1 if there is no such x
    fn binary_search<M: Fn(T) -> bool>(&self, f: M) -> usize {
        Node::binary_search(&self.root, self.e, &f)
    }
}
