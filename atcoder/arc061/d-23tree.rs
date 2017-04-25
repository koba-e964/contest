#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
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


/// 2-3 Tree
/// Reference: https://en.wikipedia.org/wiki/2%E2%80%933_tree
/// https://www.slideshare.net/sandpoonia/23-tree
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
    fn leaf_one(x: T) -> Self {
        use TwoThreeTree::*;
        Two(1, x, Box::new(Tip), Box::new(Tip))
    }
    fn leaf_two(x: T, y: T) -> Self {
        use TwoThreeTree::*;
        Three(2, x, y, Box::new(Tip), Box::new(Tip), Box::new(Tip))
    }
    fn node_two(x: T, left: Self, right: Self) -> Self {
        TwoThreeTree::Two(left.size() + right.size() + 1, x,
                          Box::new(left), Box::new(right))
    }
    fn node_three(x: T, y: T, left: Self, middle: Self, right: Self) -> Self {
        TwoThreeTree::Three(left.size() + middle.size() + right.size() + 2,
                            x, y,
                            Box::new(left), Box::new(middle), Box::new(right))
    }
    fn divide_four(t1: Self, v1: T, t2: Self, v2: T,
                       t3: Self, v3: T, t4: Self) -> (Self, Self, T) {
        (Self::node_two(v1, t1, t2), Self::node_two(v3, t3, t4), v2)
    }
    // Ok(x) -> ordinary tree
    // Err((t1, t2, v)) -> propagating v, whilst dividing the tree into t1, t2
    fn insert_sub(self, x: T) -> Result<Self, (Self, Self, T)> {
        use TwoThreeTree::*;
        match self {
            Tip => Ok(Self::leaf_one(x)),
            Two(1, val, _, _) => // leaf
                Ok(match x.cmp(&val) {
                    Ordering::Equal => Self::leaf_one(val),
                    Ordering::Less => Self::leaf_two(x, val),
                    Ordering::Greater => Self::leaf_two(val, x),
                }),
            Three(2, val1, val2, _, _, _) => { // leaf
                // $a, $b, $c should be increasing in this order.
                macro_rules! err_3 {
                    ($a:expr, $b:expr, $c:expr) => {
                        Err((Self::leaf_one($a),
                             Self::leaf_one($c),
                             $b))
                    }
                }
                if val1 == x || val2 == x {
                    Ok(Self::leaf_two(val1, val2))
                } else if x < val1 {
                    err_3!(x, val1, val2)
                } else if x < val2 {
                    err_3!(val1, x, val2)
                } else {
                    err_3!(val1, val2, x)
                }
            }
            Two(size, val, left, right) => {
                match x.cmp(&val) {
                    Ordering::Equal => 
                        Ok(Two(size, val, left, right)),
                    Ordering::Less => {
                        match left.insert_sub(x) {
                            Ok(t) => Ok(Self::node_two(val, t, *right)),
                            Err((t1, t2, sub_up)) =>
                                Ok(Self::node_three(sub_up, val,
                                                    t1, t2, *right)),
                        }
                    },
                    Ordering::Greater => {
                        match right.insert_sub(x) {
                            Ok(t) => Ok(Self::node_two(val, *left, t)),
                            Err((t1, t2, sub_up)) =>
                                Ok(Self::node_three(val, sub_up,
                                                    *left, t1, t2)),
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
                            Ok(Self::node_three(val1, val2,
                                                sub_tr, *middle, *right)),
                        Err((t1, t2, sub_up)) => {
                            let (t1, t2, v) = Self::divide_four(
                                t1, sub_up, t2, val1,
                                *middle, val2, *right);
                            Err((t1, t2, v))
                        },
                    }
                } else if x < val2 {
                    match middle.insert_sub(x) {
                        Ok(sub_tr) =>
                            Ok(Self::node_three(val1, val2,
                                                *left, sub_tr, *right)),
                        Err((t1, t2, sub_up)) => {
                            let (t1, t2, v) = Self::divide_four(
                                *left, val1, t1, sub_up,
                                t2, val2, *right);
                            Err((t1, t2, v))
                        },
                    }
                } else {
                    match right.insert_sub(x) {
                        Ok(sub_tr) =>
                            Ok(Self::node_three(val1, val2,
                                                *left, *middle, sub_tr)),
                        Err((t1, t2, sub_up)) => {
                            let (t1, t2, v) = Self::divide_four(
                                *left, val1, *middle, val2,
                                t1, sub_up, t2);
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
                Self::node_two(v, t1, t2),
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
                    Ordering::Equal => (left.size(), true),
                    Ordering::Less => left.find_index(x),
                    Ordering::Greater => {
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


fn solve() {
    let h: i32 = get();
    let w: i32 = get();
    let n = get();
    let mut pts = TwoThreeTree::new();
    let mut nbr = TwoThreeTree::new();
    for _ in 0 .. n {
        let a: i32 = get();
        let b: i32 = get();
        pts = pts.insert((a, b));
        for dx in -1 .. 2 {
            for dy in -1 .. 2 {
                let nx = a + dx;
                let ny = b + dy;
                if nx >= 2 && nx <= h - 1 && ny >= 2 && ny <= w - 1 {
                    nbr = nbr.insert((nx, ny));
                }
            }
        }
    }
    let mut a = [0i64; 10];
    for (cx, cy) in nbr.into_vec() {
        let mut cnt = 0;
        for dx in -1 .. 2 {
            for dy in -1 .. 2 {
                if pts.find_index(&(cx + dx, cy + dy)).1 {
                    cnt += 1;
                }
            }
        }
        a[cnt] += 1;
    }
    a[0] = (h as i64 - 2) * (w as i64 - 2);
    for i in 1 .. 10 {
        a[0] -= a[i];
    }
    for i in 0 .. 10 {
        println!("{}", a[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
