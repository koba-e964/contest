// http://www.prefield.com/algorithm/container/avl_tree.html
// https://atcoder.jp/contests/past202112-open/submissions/28434154
#[derive(Clone)]
struct Node<T> {
    p: T,
    size: usize,
    height: i32,
    ch: [Option<Box<Self>>; 2],
}

impl<T: PartialOrd + std::fmt::Debug + Default> Node<T> {
    fn new(p: T) -> Self {
        Node {
            p: p,
            size: 1,
            height: 1,
            ch: [None, None],
        }
    }
    // Insert a single element represented as a leaf node.
    fn insert(l: Option<Box<Self>>, x: Self) -> Option<Box<Self>> {
        let mut t = match l {
            None => return Some(Box::new(x)),
            Some(l) => l,
        };
        if t.p >= x.p {
            t.ch[0] = Self::insert(t.ch[0].take(), x);
        } else {
            t.ch[1] = Self::insert(t.ch[1].take(), x);
        }
        Self::cons(&mut t);
        Some(Self::balance(t))
    }
    fn erase(l: Option<Box<Self>>, p: &T) -> Option<Box<Self>> {
        let mut t = match l {
            None => return None,
            Some(l) => l,
        };
        if t.p == *p {
            return Self::move_down(t.ch[0].take(), t.ch[1].take());
        }
        if t.p > *p {
            t.ch[0] = Self::erase(t.ch[0].take(), p);
        } else {
            t.ch[1] = Self::erase(t.ch[1].take(), p);
        }
        Self::cons(&mut t);
        Some(Self::balance(t))
    }
    fn move_down(l: Option<Box<Self>>, r: Option<Box<Self>>) -> Option<Box<Self>> {
        let mut t = match l {
            None => return r,
            Some(l) => l
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
    fn at(l: &Option<Box<Self>>, pos: usize) -> Option<&T> {
        let t = match l {
            None => return None,
            Some(l) => l,
        };
        if pos >= t.size {
            return None;
        }
        let ll = Self::len(&t.ch[0]);
        if pos < ll {
            return Self::at(&t.ch[0], pos);
        }
        if pos == ll {
            return Some(&t.p);
        }
        Self::at(&t.ch[1], pos - ll - 1)
    }
    // Make t consistent.
    fn cons(t: &mut Box<Self>) {
        for i in 0..2 {
            if let &Some(ref c) = &t.ch[i] {
                assert!(if i == 0 { t.p >= c.p } else { t.p <= c.p }, "i = {}. t.p = {:?}, c.p = {:?}", i, t.p, c.p);
            }
        }
        t.height = std::cmp::max(Self::ht(&t.ch[0]), Self::ht(&t.ch[1])) + 1;
        t.size = 1 + Self::len(&t.ch[0]) + Self::len(&t.ch[1]);
    }
    fn rotate(mut t: Box<Self>, l: usize, r: usize) -> Box<Self> {
        let mut s = t.ch[r].take().unwrap();
        t.ch[r] = s.ch[l].take();
        Self::cons(&mut t);
        s.ch[l] = Some(Self::balance(t));
        Self::cons(&mut s);
        Self::balance(s)
    }
    fn balance(mut t: Box<Self>) -> Box<Self> {
        for i in 0..2 {
            if Self::ht(&t.ch[1 - i]) - Self::ht(&t.ch[i]) < -1 {
                let tmp = t.ch[i].as_mut().unwrap();
                if Self::ht(&tmp.ch[1 - i]) - Self::ht(&tmp.ch[i]) > 0 {
                    let dummy = Self::new(T::default());
                    *tmp = 
                        Self::rotate(std::mem::replace(tmp, Box::new(dummy)), i, 1 - i);
                }
                return Self::rotate(t, 1 - i , i);
            }
        }
        Self::cons(&mut t);
        t
    }
    fn each<'a, F: FnMut(&'a T)>(t: &'a Option<Box<Self>>, f: &mut F) {
        let t = match t {
            None => return,
            Some(t) => t,
        };
        Self::each(&t.ch[0], f);
        f(&t.p);
        Self::each(&t.ch[1], f);
    }
}

#[derive(Clone)]
struct AVLTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + std::fmt::Debug + Default> AVLTree<T> {
    fn new() -> Self {
        AVLTree { root: None }
    }
    #[allow(unused)]
    fn len(&self) -> usize {
        Node::len(&self.root)
    }
    fn insert(&mut self, p: T) {
        self.root = Node::insert(self.root.take(), Node::new(p));
    }
    fn at(&self, pos: usize) -> Option<&T> {
        Node::at(&self.root, pos)
    }
    fn erase(&mut self, p: &T) {
        self.root = Node::erase(self.root.take(), p);
    }
    // The order is not specified.
    #[allow(unused)]
    fn each<'a, F: FnMut(&'a T)>(&'a self, mut f: F) {
        Node::each(&self.root, &mut f)
    }
}
