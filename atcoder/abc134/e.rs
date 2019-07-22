#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

/// 2-3 Tree
/// Reference: https://en.wikipedia.org/wiki/2%E2%80%933_tree
/// https://www.slideshare.net/sandpoonia/23-tree
/// Verified by: ARC061-D (http://arc061.contest.atcoder.jp/submissions/1246386)
/// Codeforces 404E (http://codeforces.com/contest/785/submission/26747752), TLE
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
    pub fn size(&self) -> usize {
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
    pub fn insert(self, x: T) -> Self {
        match self.insert_sub(x) {
            Ok(t) => t,
            Err((t1, t2, v)) =>
                Self::node_two(v, Box::new(t1), Box::new(t2)),
        }
    }
    fn get_min(&mut self) -> &mut T {
        use TwoThreeTree::*;
        match *self {
            Tip => panic!(),
            Two(_, ref mut val, ref mut left, _) => {
                if let &Tip = &left as &Self {
                    val
                } else {
                    left.get_min()
                }
            }
            Three(_, ref mut val, _, ref mut left, _, _) => {
                if let &Tip = &left as &Self {
                    val
                } else {
                    left.get_min()
                }
            }
        }
    }
    fn delete_sub(self, x: &T, mut leftist: bool)
                  -> (Result<Self, Box<Self>>, Option<T>) {
        use TwoThreeTree::*;
        match self {
            Tip => (Ok(Tip), None),
            Two(_, mut val, left, mut right) => {
                let mut cmp_result = x.cmp(&val);
                if leftist {
                    if let &Tip = &left as &Self {
                        cmp_result = std::cmp::Ordering::Equal;
                    } else {
                        cmp_result = std::cmp::Ordering::Less;
                    }
                }
                if cmp_result == std::cmp::Ordering::Equal {
                    if let Tip = *right {
                        return (Err(Box::new(Tip)), Some(val));
                    }
                    let inord_succ = right.get_min();
                    std::mem::swap(&mut val, inord_succ);
                    cmp_result = std::cmp::Ordering::Greater;
                    leftist = true;
                }
                match cmp_result {
                    std::cmp::Ordering::Equal => panic!(),
                    std::cmp::Ordering::Less => {
                        match left.delete_sub(x, leftist) {
                            (Ok(t), ret) =>
                                (Ok(Self::node_two(val, Box::new(t), right)), ret),
                            (Err(box_t), ret) => {
                                let right = *right; // Workaround: http://stackoverflow.com/questions/28466809/collaterally-moved-error-on-deconstructing-box-of-pairs
                                match right {
                                    Tip => panic!(),
                                    Two(_, rv, rleft, rright) => {
                                        let newtree =
                                            Self::node_three(val, rv, box_t, rleft, rright);
                                        (Err(Box::new(newtree)), ret)
                                    },
                                    Three(_, rv1, rv2, rleft, rmiddle, rright) => {
                                        // rotation
                                        let left = Self::node_two(val, box_t, rleft);
                                        let right = Self::node_two(rv2, rmiddle, rright);
                                        (Ok(Self::node_two(rv1, Box::new(left), Box::new(right))), ret)
                                    },
                                }
                            },
                        }
                    },
                    std::cmp::Ordering::Greater => {
                        match right.delete_sub(x, leftist) {
                            (Ok(t), ret) =>
                                (Ok(Self::node_two(val, left, Box::new(t))), ret),
                            (Err(box_t), ret) => {
                                let left = *left; // Workaround
                                match left {
                                    Tip => panic!(),
                                    Two(_, lv, lleft, lright) => {
                                        let newtree =
                                            Self::node_three(lv, val, lleft, lright, box_t);
                                        (Err(Box::new(newtree)), ret)
                                    },
                                    Three(_, lv1, lv2, lleft, lmiddle, lright) => {
                                        // rotation
                                        let left = Self::node_two(lv1, lleft, lmiddle);
                                        let right = Self::node_two(val, lright, box_t);
                                        (Ok(Self::node_two(lv2, Box::new(left), Box::new(right))), ret)
                                    },
                                }
                            },
                        }
                    },
                }
            }
            Three(_, mut val1, mut val2, left, mut middle, mut right) => {
                let mut cmp_result;
                if leftist {
                    if let &Tip = &left as &Self {
                        cmp_result = 1;
                    } else {
                        cmp_result = 0;
                    }
                } else if x < &val1 {
                    cmp_result = 0;
                } else if x == &val1 {
                    cmp_result = 1;
                } else if x < &val2 {
                    cmp_result = 2;
                } else if x == &val2 {
                    cmp_result = 3;
                } else {
                    cmp_result = 4;
                }
                if cmp_result == 1 {
                    if let Tip = *right {
                        return (Ok(Self::node_two(
                            val2, Box::new(Tip), Box::new(Tip))),
                                   Some(val1));
                    }
                    let inord_succ = middle.get_min();
                    std::mem::swap(&mut val1, inord_succ);
                    cmp_result = 2;
                    leftist = true;
                }
                if cmp_result == 3 {
                    if let Tip = *right {
                        return (Ok(Self::node_two(
                            val1, Box::new(Tip), Box::new(Tip))),
                                   Some(val2));
                    }
                    let inord_succ = right.get_min();
                    std::mem::swap(&mut val2, inord_succ);
                    cmp_result = 4;
                    leftist = true;
                }
                macro_rules! three_two {
                    ($ll:expr, $lv:expr, $lr:expr,
                     $val1:expr,
                     $ml:expr, $mv:expr, $mr:expr,
                     $val2:expr,
                     $rl:expr, $rv:expr, $rr:expr) => {
                        Self::node_three(
                            $val1,
                            $val2,
                            Box::new(Self::node_two($lv, $ll, $lr)),
                            Box::new(Self::node_two($mv, $ml, $mr)),
                            Box::new(Self::node_two($rv, $rl, $rr)))
                    }
                }
                match cmp_result {
                    0 => {
                        match left.delete_sub(x, leftist) {
                            (Ok(t), ret) =>
                                (Ok(Self::node_three(
                                    val1, val2,
                                    Box::new(t), middle, right)),
                                 ret),
                            (Err(t), ret) => {
                                match (*middle, *right) {
                                    (Three(_, mv1, mv2, mleft, mmiddle, mright),
                                     right) =>
                                        (Ok(Self::node_three(
                                            mv1, val2,
                                            Box::new(Self::node_two(val1, t, mleft)),
                                            Box::new(Self::node_two(mv2, mmiddle, mright)),
                                            Box::new(right))), ret),
                                    (Two(_, mv, mleft, mright),
                                     Three(_, rv1, rv2, rleft, rmiddle, rright)) =>
                                        (Ok(three_two!(
                                            t, val1, mleft,
                                            mv,
                                            mright, val2, rleft,
                                            rv1,
                                            rmiddle, rv2, rright)), ret),
                                    (Two(_, mv, mleft, mright),
                                     right) =>
                                        (Ok(Self::node_two(
                                            val2,
                                            Box::new(Self::node_three(
                                                val1, mv, t, mleft, mright)),
                                            Box::new(right))), ret),
                                    _ => panic!("Unexpected Tip"),
                                }
                            }
                        }
                    },
                    2 => {
                        match middle.delete_sub(x, leftist) {
                            (Ok(t), ret) =>
                                (Ok(Self::node_three(
                                    val1, val2,
                                    left, Box::new(t), right)),
                                 ret),
                            (Err(t), ret) => {
                                match (*left, *right) {
                                    (Three(_, lv1, lv2, lleft, lmiddle, lright),
                                     right) =>
                                        (Ok(Self::node_three(
                                            lv2, val2,
                                            Box::new(Self::node_two(lv1, lleft, lmiddle)),
                                            Box::new(Self::node_two(val1, lright, t)),
                                            Box::new(right))), ret),
                                    (Two(_, lv, lleft, lright),
                                     Three(_, rv1, rv2, rleft, rmiddle, rright)) =>
                                        (Ok(three_two!(
                                            lleft, lv, lright,
                                            val1,
                                            t, val2, rleft,
                                            rv1,
                                            rmiddle, rv2, rright)), ret),
                                    (Two(_, lv, lleft, lright),
                                     right) =>
                                        (Ok(Self::node_two(
                                            val2,
                                            Box::new(Self::node_three(
                                                lv, val1, lleft, lright, t)),
                                            Box::new(right))), ret),
                                    _ => panic!("Unexpected Tip"),
                                }
                            }
                        }
                    },
                    4 => {
                        match right.delete_sub(x, leftist) {
                            (Ok(t), ret) =>
                                (Ok(Self::node_three(
                                    val1, val2,
                                    left, middle, Box::new(t))),
                                 ret),
                            (Err(t), ret) => {
                                match (*left, *middle) {
                                    (left,
                                     Three(_, mv1, mv2, mleft, mmiddle, mright)) =>
                                        (Ok(Self::node_three(
                                            val1, mv2,
                                            Box::new(left),
                                            Box::new(Self::node_two(mv1, mleft, mmiddle)),
                                            Box::new(Self::node_two(val2, mright, t)))), ret),
                                    (Three(_, lv1, lv2, lleft, lmiddle, lright),
                                     Two(_, mv, mleft, mright)) =>
                                        (Ok(three_two!(
                                            lleft, lv1, lmiddle,
                                            lv2,
                                            lright, val1, mleft,
                                            mv,
                                            mright, val2, t)), ret),
                                    (left,
                                     Two(_, mv, mleft, mright)) =>
                                        (Ok(Self::node_two(
                                            val1,
                                            Box::new(left),
                                            Box::new(Self::node_three(
                                                mv, val2, mleft, mright, t)))), ret),
                                    _ => panic!("Unexpected Tip"),
                                }
                            }
                        }
                    },
                    _ => panic!("Unexpected case"),
                }
            },
        }
    }
    pub fn delete(self, x: &T) -> (Self, Option<T>) {
        let (t, ret) = self.delete_sub(x, false);
        match t {
            Ok(t) => (t, ret),
            Err(t) => (*t, ret),
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
    // Verified by https://atcoder.jp/contests/abc128/submissions/5654911
    pub fn at(&self, nth: usize) -> Option<&T> {
        use TwoThreeTree::*;
        match *self {
            Tip => None,
            Two(_size, ref v, ref left, ref right) => {
                if nth < left.size() {
                    left.at(nth)
                } else if nth == left.size() {
                    Some(v)
                } else {
                    right.at(nth - left.size() - 1)
                }
            },
            Three(_size, ref v1, ref v2, ref left, ref middle, ref right) => {
                if nth < left.size() {
                    left.at(nth)
                } else if nth == left.size() {
                    Some(v1)
                } else if nth < left.size() + 1 + middle.size() {
                    middle.at(nth - left.size() - 1)
                } else if nth == left.size() + 1 + middle.size() {
                    Some(v2)
                } else {
                    right.at(nth - left.size() - 1 - middle.size() - 1)
                }
            }
        }
    }
    pub fn into_vec(self) -> Vec<T> {
        let mut ret = Vec::with_capacity(self.size());
        self.into_vec_sub(&mut ret);
        ret
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input!(a: [i64]);
    let mut tree = TwoThreeTree::new();
    let mut cnt = 1;
    for &a in &a {
        let (idx, _) = tree.find_index(&(a, 0));
        if idx > 0 {
            let &elem = tree.at(idx - 1).unwrap();
            tree = tree.delete(&elem).0;
        }
        tree = tree.insert((a, cnt));
        cnt += 1;
    }
    puts!("{}\n", tree.size());
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
