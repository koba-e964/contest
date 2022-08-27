use std::cmp::*;
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

struct HaskSegTree {
    lv: usize,
    ch: [[Option<Box<Self>>; 2]; 2],
    ma: i64,
}

impl HaskSegTree {
    fn new(lv: usize) -> Self {
        HaskSegTree {
            lv: lv,
            ch: [[None, None], [None, None]],
            ma: 0,
        }
    }
    fn get(&self, x: usize, y: usize) -> i64 {
        if self.lv == 0 {
            return self.ma;
        }
        let lv = self.lv;
        if x >= 1 << lv && y >= 1 << lv {
            return self.ma;
        }
        let half = 1 << (lv - 1);
        let mut ma = 0;
        if let Some(ch) = &self.ch[0][0] {
            ma = max(ma, ch.get(x, y));
        }
        if x >= half {
            if let Some(ch) = &self.ch[1][0] {
                ma = max(ma, ch.get(x - half, y));
            }
            if y >= half {
                if let Some(ch) = &self.ch[1][1] {
                    ma = max(ma, ch.get(x - half, y - half));
                }
            }
        }
        if y >= half {
            if let Some(ch) = &self.ch[0][1] {
                ma = max(ma, ch.get(x, y - half));
            }
        }
        ma
    }
    fn create(&mut self, x: usize, y: usize) -> &mut Self {
        if self.ch[x][y].is_none() {
            self.ch[x][y] = Some(Box::new(Self::new(self.lv - 1)));
        }
        self.ch[x][y].as_mut().unwrap()
    }
    fn update(&mut self, x: usize, y: usize, val: i64) {
        if self.lv == 0 {
            self.ma = max(self.ma, val);
            return;
        }
        let lv = self.lv;
        if x >= 1 << lv && y >= 1 << lv {
            return;
        }
        let half = 1 << (lv - 1);
        self.ma = max(self.ma, val);
        if x >= half {
            if y >= half {
                let r = self.create(1, 1);
                return r.update(x - half, y - half, val);
            }
            let r = self.create(1, 0);
            return r.update(x - half, y, val);
        }
        if y >= half {
            let r = self.create(0, 1);
            return r.update(x, y - half, val);
        }
        let r = self.create(0, 0);
        r.update(x, y, val);
    }
}

// Tags: weighted-2d-lis
fn main() {
    input! {
        n: usize,
        txya: [(i64, i64, i64, i64); n],
    }
    let mut pts = vec![];
    let mut coo1 = vec![0];
    let mut coo2 = vec![0];
    pts.push((0, 0, 0, 0));
    for (t, x, y, a) in txya {
        pts.push((y, t - x - y, t + x - y, a));
        coo1.push(t - x - y);
        coo2.push(t + x - y);
    }
    pts.sort();
    coo1.sort(); coo1.dedup();
    coo2.sort(); coo2.dedup();
    pts.reverse();
    let pts: Vec<_> = pts.into_iter().map(|(t, x, y, a)| (t, coo1.binary_search(&x).unwrap(), coo2.binary_search(&y).unwrap(), a)).collect();
    let mut st = HaskSegTree::new(17);
    let mut ans = 0;
    let full = (1 << 17) - 1;
    for (_t, x, y, a) in pts {
        let val = st.get(full - x, full - y);
        st.update(full - x, full - y, val + a);
        if a == 0 {
            ans = val;
        }
    }
    println!("{}", ans);
}
