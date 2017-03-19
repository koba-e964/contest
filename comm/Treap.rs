/// Treap (balanced binary search tree)
/// Reference: https://www.slideshare.net/iwiwi/2-12188757
/// Verified by: ARC061-D (http://arc061.contest.atcoder.jp/submissions/1172709)
/// 2150ms for n = 9*10^5, maybe a bit slow for an O(n * log(n))-time algorithm...
#[derive(Clone, Debug)]
enum Treap<T> {
    Bin(
        usize, // size
        i64, // priority
        T, // value
        Box<Treap<T>>, // left
        Box<Treap<T>>, // right
    ),
    Tip,
}

impl<T: Ord> Treap<T> {
    pub fn new() -> Self { Treap::Tip }
    pub fn singleton(v: T, pri: i64) -> Self {
        use Treap::*;
        Bin(1, pri, v, Box::new(Tip), Box::new(Tip))
    }
    pub fn size(&self) -> usize {
        use Treap::*;
        match *self {
            Tip => 0,
            Bin(t, _,  _, _, _) => t,
        }
    }
    // Merges two BST. Their ownership is taken.
    pub fn merge(left: Self, right: Self) -> Self {
        use Treap::*;
        match (left, right) {
            (Tip, Tip) => Tip,
            (Tip, x) => x,
            (x, Tip) => x,
            (Bin(lsize, lpri, lelem, lleft, lright),
             Bin(rsize, rpri, relem, rleft, rright)) => {
                if lpri > rpri {
                    let right = Bin(rsize, rpri, relem, rleft, rright);
                    let mut ret = Bin(lsize, lpri, lelem, lleft,
                                  Box::new(Self::merge(*lright, right)));
                    ret.update();
                    ret
                } else {
                    let left = Bin(lsize, lpri, lelem, lleft, lright);
                    let mut ret = Bin(rsize, rpri, relem,
                                      Box::new(Self::merge(left, *rleft)),
                                      rright);
                    ret.update();
                    ret
                }
            }
        }
    }
    pub fn split(self, k: usize) -> (Self, Self) {
        use Treap::*;
        match self {
            Tip => (Tip, Tip),
            Bin(size, pri, elem, left, right) => {
                if k <= left.size() {
                    let (sl, sr) = Self::split(*left, k);
                    let mut ret = Bin(size, pri, elem, Box::new(sr), right);
                    ret.update();
                    (sl, ret)
                } else {
                    let (sl, sr) = Self::split(*right, k - left.size() - 1);
                    let mut ret = Bin(size, pri, elem, left, Box::new(sl));
                    ret.update();
                    (ret, sr)
                }
            }
        }
    }
    fn update(&mut self) {
        use Treap::*;
        match *self {
            Tip => (),
            Bin(ref mut lsize, ref _pri, ref _elem, ref left, ref right) => {
                *lsize = left.size() + right.size() + 1;
            },
        }
    }
    fn insert_at(self, v: T, pri: i64, k: usize) -> Self {
        use Treap::*;
        // Speed up: compare the priority
        match self {
            Tip => Self::singleton(v, pri),
            Bin(size, spri, elem, left, right) => {
                let lsize = left.size();
                if spri <= pri {
                    let cself = Bin(size, spri, elem, left, right);
                    let (left, right) = cself.split(k);
                    return Bin(size + 1, pri, v,
                               Box::new(left), Box::new(right));
                }
                if k < lsize {
                    return Bin(size + 1, spri, elem,
                               Box::new((*left).insert_at(v, pri, k)),
                               right);
                }
                if k >= lsize + 1 {
                    return Bin(size + 1, spri, elem,
                               left,
                               Box::new((*right)
                                        .insert_at(v, pri, k - lsize - 1)));
                }
                let cself = Bin(size, spri, elem, left, right);
                let sing = Self::singleton(v, pri);
                let (left, right) = cself.split(k);
                let tmp = Self::merge(left, sing);
                Self::merge(tmp, right)
            }
        }
    }
    fn erase_at(self, k: usize) -> Self {
        use Treap::*;
        match self {
            Tip => Tip,
            Bin(size, pri, elem, left, right) => {
                if k < left.size() {
                    return Bin(size - 1, pri, elem,
                               Box::new((*left).erase_at(k)), right);
                }
                if k == left.size() {
                    return Self::merge(*left, *right); // hit
                }
                let lsize = left.size();
                return Bin(size - 1, pri, elem,
                           left,
                           Box::new((*right).erase_at(k - lsize - 1)));
            }
        }
    }
    fn find_index(&self, t: &T) -> (usize, bool) {
        use Treap::*;
        use std::cmp::Ordering;
        let mut offset = 0;
        let mut tap = self;
        loop {
            match *tap {
                Tip => return (offset, false),
                Bin(_, _, ref elem, ref left, ref right) => {
                    match elem.cmp(t) {
                        Ordering::Equal => return (offset + left.size(), true),
                        Ordering::Greater =>
                            tap = left,
                        Ordering::Less => {
                            offset += left.size() + 1;
                            tap = right;
                        },
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    fn insert(self, v: T, pri: i64) -> Self {
        let (idx, found) = self.find_index(&v);
        if found {
            self
        } else {
            self.insert_at(v, pri, idx)
        }
    }
    #[allow(dead_code)]
    fn erase(self, v: &T) -> Self {
        let (idx, found) = self.find_index(v);
        if found {
            self.erase_at(idx)
        } else {
            self
        }
    }
    fn into_vec_sub(self, vec: &mut Vec<T>) {
        use Treap::*;
        match self {
            Tip => (),
            Bin(_, _, elem, left, right) => {
                left.into_vec_sub(vec);
                vec.push(elem);
                right.into_vec_sub(vec);
            }
        }
    }
    #[allow(dead_code)]
    pub fn into_vec(self) -> Vec<T> {
        let mut ret = Vec::new();
        self.into_vec_sub(&mut ret);
        ret
    }
}
