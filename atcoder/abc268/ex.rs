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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Ref: http://algoogle.hadrori.jp/algorithm/aho-corasick.html
// Verified by: https://atcoder.jp/contests/jsc2019-final/submissions/23661893
// Verified by: https://atcoder.jp/contests/joisc2010/submissions/23693164
// Verified by: https://atcoder.jp/contests/jag2017autumn/submissions/23887937
// If no reference to the root remains, it does not work correctly.
struct PMA<T> {
    len: usize,
    next: Vec<Option<std::rc::Rc<std::cell::RefCell<Self>>>>,
    dp: T,
    back: std::rc::Weak<std::cell::RefCell<Self>>
}

impl<T: Clone> PMA<T> {
    fn new(len: usize, e: T) -> std::rc::Rc<std::cell::RefCell<Self>> {
        use std::rc::{Rc, Weak};
        use std::cell::RefCell;
        Rc::new(RefCell::new(PMA {
            len: len,
            next: vec![None; len],
            dp: e,
            back: Weak::new(),
        }))
    }
    #[allow(unused)]
    pub fn with_lower_strings<F: Fn(&[usize], usize) -> T, M: Fn(T, T) -> T>(pat: &[Vec<char>], f: F, m: M, e: T) -> std::rc::Rc<std::cell::RefCell<Self>> {
        let len = 26;
        let pat: Vec<Vec<_>> = pat.iter().map(|pat| pat.iter().map(|&x| (x as u8 - b'a') as _).collect()).collect();
        Self::with_arrays(len, &pat, f, m, e)
    }
    pub fn with_arrays<F: Fn(&[usize], usize) -> T, M: Fn(T, T) -> T>(len: usize, pat: &[Vec<usize>], f: F, m: M, e: T) -> std::rc::Rc<std::cell::RefCell<Self>> {
        use std::rc::{Rc, Weak};
        let root = Self::new(len, e.clone());
        let root_cp = Rc::clone(&root);
        let root_weak = Rc::downgrade(&root);
        root.borrow_mut().back = Weak::clone(&root_weak);
        for pat in pat {
            Self::add_pattern(root.clone(), &pat, &f, &m, e.clone());
        }
        let mut que = std::collections::VecDeque::new();
        for i in 0..len {
            if root.borrow().next[i].is_none() {
                root.borrow_mut().next[i] = Some(Rc::clone(&root_cp));
            } else {
                let tmp = root.borrow().next[i].clone().unwrap();
                tmp.borrow_mut().back = Weak::clone(&root_weak);
                que.push_back(tmp);
            }
        }
        while let Some(now) = que.pop_front() {
            for i in 0..len {
                if let Some(tmp) = now.borrow().next[i].clone() {
                    let mut nxt = Weak::upgrade(&now.borrow().back).unwrap();
                    while nxt.borrow().next[i].is_none() {
                        let val = Weak::upgrade(&nxt.borrow().back).unwrap();
                        nxt = val;
                    }
                    let to = nxt.borrow().next[i].clone().unwrap();
                    tmp.borrow_mut().back = Rc::downgrade(&to);
                    let newdp = m(tmp.borrow().dp.clone(), to.borrow().dp.clone());
                    tmp.borrow_mut().dp = newdp;
                    que.push_back(tmp);
                }
            }
        }
        root
    }
    fn add_pattern<F: Fn(&[usize], usize) -> T, M: Fn(T, T) -> T>(root: std::rc::Rc<std::cell::RefCell<Self>>, pat: &[usize], f: &F, m: &M, e: T) {
        let len = root.borrow().len;
        let mut now = root;
        for i in 0..pat.len() {
            let c = pat[i];
            if now.borrow().next[c].is_none() {
                now.borrow_mut().next[c] = Some(Self::new(len, e.clone()));
            }
            let val = now.borrow().next[c].clone().unwrap();
            now = val;
            let newdp = m(now.borrow().dp.clone(), f(&pat, i + 1));
            now.borrow_mut().dp = newdp;
        }
    }
    fn progress(mut pma: std::rc::Rc<std::cell::RefCell<Self>>, idx: usize)
                -> std::rc::Rc<std::cell::RefCell<Self>> {
        while pma.borrow().next[idx].is_none() {
            let val = std::rc::Weak::upgrade(&pma.borrow().back).unwrap();
            pma = val;
        }
        pma.borrow().next[idx].clone().unwrap()
    }
}

// Tags: aho-corasick
fn main() {
    input! {
        s: chars,
        n: usize,
        t: [chars; n],
    }
    let root = PMA::with_lower_strings(&t, |s, idx| if s.len() == idx { 1 } else { 0 }, std::cmp::max, 0);
    let mut cur = root.clone();
    let mut ans = 0;
    for c in s {
        let idx = (c as u8 - b'a') as usize;
        cur = PMA::progress(cur, idx);
        if cur.borrow().dp == 1 {
            ans += 1;
            cur = root.clone();
        }
    }
    println!("{}", ans);
}
