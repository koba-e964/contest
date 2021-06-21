// Ref: http://algoogle.hadrori.jp/algorithm/aho-corasick.html
// Verified by: https://atcoder.jp/contests/jsc2019-final/submissions/23661893
struct PMA {
    len: usize,
    next: Vec<Option<std::rc::Rc<std::cell::RefCell<PMA>>>>,
    dp: i64,
    back: std::rc::Weak<std::cell::RefCell<PMA>>
}

impl PMA {
    fn new(len: usize) -> std::rc::Rc<std::cell::RefCell<PMA>> {
        use std::rc::{Rc, Weak};
        use std::cell::RefCell;
        Rc::new(RefCell::new(PMA {
            len: len,
            next: vec![None; len],
            dp: 1 << 60,
            back: Weak::new(),
        }))
    }
    pub fn with_lower_strings<F: Fn(&[usize], usize) -> i64>(pat: &[Vec<char>], f: F) -> std::rc::Rc<std::cell::RefCell<PMA>> {
        use std::rc::{Rc, Weak};
        let len = 26;
        let root = Self::new(len);
        let root_cp = Rc::clone(&root);
        let root_weak = Rc::downgrade(&root);
        root.borrow_mut().back = Weak::clone(&root_weak);
        for pat in pat {
            let pat: Vec<_> = pat.iter().map(|&x| (x as u8 - b'a') as _).collect();
            Self::add_pattern(root.clone(), &pat, &f);
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
                    let newdp = std::cmp::min(tmp.borrow().dp, to.borrow().dp);
                    tmp.borrow_mut().dp = newdp;
                    que.push_back(tmp);
                }
            }
        }
        root
    }
    fn add_pattern<F: Fn(&[usize], usize) -> i64>(root: std::rc::Rc<std::cell::RefCell<PMA>>, pat: &[usize], f: &F) {
        let len = root.borrow().len;
        let mut now = root;
        for i in 0..pat.len() {
            let c = pat[i];
            if now.borrow().next[c].is_none() {
                now.borrow_mut().next[c] = Some(Self::new(len));
            }
            let val = now.borrow().next[c].clone().unwrap();
            now = val;
            let newdp = std::cmp::min(now.borrow().dp, f(&pat, i + 1));
            now.borrow_mut().dp = newdp;
        }
    }
    fn progress(mut pma: std::rc::Rc<std::cell::RefCell<PMA>>, idx: usize)
                -> std::rc::Rc<std::cell::RefCell<PMA>> {
        while pma.borrow().next[idx].is_none() {
            let val = std::rc::Weak::upgrade(&pma.borrow().back).unwrap();
            pma = val;
        }
        pma.borrow().next[idx].clone().unwrap()
    }
}
