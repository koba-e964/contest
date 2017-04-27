/// 2-3 Tree
/// Reference: https://en.wikipedia.org/wiki/2%E2%80%933_tree
/// https://www.slideshare.net/sandpoonia/23-tree
/// Verified by: ARC061-D (http://arc061.contest.atcoder.jp/submissions/1246386)
#[derive(Clone, Debug)]
enum TwoThreeTree<T> {
    Tip,
    Two(
        usize, // size
        T, // value
        Box<TwoThreeTree<T>>, // left
        Box<TwoThreeTree<T>>, // right
    ),
    Three(
        usize, // size
        T, // val1
        T, // val2
        Box<TwoThreeTree<T>>, // left
        Box<TwoThreeTree<T>>, // middle
        Box<TwoThreeTree<T>>, // right
    ),
}

impl<T: Ord> TwoThreeTree<T> {
    pub fn new() -> Self {
        TwoThreeTree::Tip
    }
    fn size(&self) -> usize {
        use TwoThreeTree::*;
        match *self {
            Tip => 0,
            Two(sz, _, _, _) => sz,
            Three(sz, _, _, _, _, _) => sz,
        }
    }
    fn node_two(x: T, left: Box<Self>, right: Box<Self>) -> Self {
        TwoThreeTree::Two(left.size() + right.size() + 1, x,
                          left, right)
    }
    fn node_three(x: T, y: T, left: Box<Self>, middle: Box<Self>, right: Box<Self>)
                  -> Self {
        TwoThreeTree::Three(left.size() + middle.size() + right.size() + 2,
                            x, y,
                            left, middle, right)
    }
    fn divide_four(t1: Box<Self>, v1: T, t2: Box<Self>, v2: T,
                       t3: Box<Self>, v3: T, t4: Box<Self>) -> (Self, Self, T) {
        (Self::node_two(v1, t1, t2), Self::node_two(v3, t3, t4), v2)
    }
    // Ok(x) -> ordinary tree
    // Err((t1, t2, v)) -> propagating v, whilst dividing the tree into t1, t2
    fn insert_sub(self, x: T) -> Result<Self, (Self, Self, T)> {
        use TwoThreeTree::*;
        match self {
            Tip => Err((Tip, Tip, x)),
            Two(size, val, left, right) => {
                match x.cmp(&val) {
                    std::cmp::Ordering::Equal => 
                        Ok(Two(size, val, left, right)),
                    std::cmp::Ordering::Less => {
                        match left.insert_sub(x) {
                            Ok(t) =>
                                Ok(Self::node_two(val, Box::new(t), right)),
                            Err((t1, t2, sub_up)) =>
                                Ok(Self::node_three(
                                    sub_up, val,
                                    Box::new(t1), Box::new(t2), right)),
                        }
                    },
                    std::cmp::Ordering::Greater => {
                        match right.insert_sub(x) {
                            Ok(t) =>
                                Ok(Self::node_two(val, left, Box::new(t))),
                            Err((t1, t2, sub_up)) =>
                                Ok(Self::node_three(
                                    val, sub_up,
                                    left, Box::new(t1), Box::new(t2))),
                        }
                    },
                }
                        
            }
            Three(size, val1, val2, left, middle, right) => {
                if x == val1 || x == val2 {
                    return Ok(Three(size, val1, val2, left, middle, right));
                }
                if x < val1 {
                    match left.insert_sub(x) {
                        Ok(sub_tr) =>
                            Ok(Self::node_three(
                                val1, val2,
                                Box::new(sub_tr), middle, right)),
                        Err((t1, t2, sub_up)) => {
                            let (t1, t2, v) = Self::divide_four(
                                Box::new(t1), sub_up, Box::new(t2), val1,
                                middle, val2, right);
                            Err((t1, t2, v))
                        },
                    }
                } else if x < val2 {
                    match middle.insert_sub(x) {
                        Ok(sub_tr) =>
                            Ok(Self::node_three(
                                val1, val2,
                                left, Box::new(sub_tr), right)),
                        Err((t1, t2, sub_up)) => {
                            let (t1, t2, v) = Self::divide_four(
                                left, val1, Box::new(t1), sub_up,
                                Box::new(t2), val2, right);
                            Err((t1, t2, v))
                        },
                    }
                } else {
                    match right.insert_sub(x) {
                        Ok(sub_tr) =>
                            Ok(Self::node_three(
                                val1, val2,
                                left, middle, Box::new(sub_tr))),
                        Err((t1, t2, sub_up)) => {
                            let (t1, t2, v) = Self::divide_four(
                                left, val1, middle, val2,
                                Box::new(t1), sub_up, Box::new(t2));
                            Err((t1, t2, v))
                        },
                    }
                }
            }
        }
    }
    fn insert(self, x: T) -> Self {
        match self.insert_sub(x) {
            Ok(t) => t,
            Err((t1, t2, v)) =>
                Self::node_two(v, Box::new(t1), Box::new(t2)),
        }
    }
    fn into_vec_sub(self, ret: &mut Vec<T>) {
        use TwoThreeTree::*;
        match self {
            Tip => (),
            Two(_, val, left, right) => {
                left.into_vec_sub(ret);
                ret.push(val);
                right.into_vec_sub(ret);
            },
            Three(_, val1, val2, left, middle, right) => {
                left.into_vec_sub(ret);
                ret.push(val1);
                middle.into_vec_sub(ret);
                ret.push(val2);
                right.into_vec_sub(ret);
            },
        }
    }
    pub fn find_index(&self, x: &T) -> (usize, bool) {
        use TwoThreeTree::*;
        match *self {
            Tip => (0, false),
            Two(_size, ref v, ref left, ref right) => {
                match x.cmp(v) {
                    std::cmp::Ordering::Equal => (left.size(), true),
                    std::cmp::Ordering::Less => left.find_index(x),
                    std::cmp::Ordering::Greater => {
                        let (res, found) = right.find_index(x);
                        (res + left.size() + 1, found)
                    },
                }
            },
            Three(_size, ref v1, ref v2, ref left, ref middle, ref right) => {
                if x == v1 {
                    return (left.size(), true);
                }
                if x == v2 {
                    return (left.size() + middle.size() + 1, true);
                }
                if x < v1 {
                    return left.find_index(x);
                }
                if x < v2 {
                    let (res, found) = middle.find_index(x);
                    return (left.size() + 1 + res, found);
                }
                let (res, found) = right.find_index(x);
                return (left.size() + middle.size() + 2 + res, found);
            }
        }
    }
    pub fn into_vec(self) -> Vec<T> {
        let mut ret = Vec::with_capacity(self.size());
        self.into_vec_sub(&mut ret);
        ret
    }
}
