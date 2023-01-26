use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
// T is a commutative monoid. Indices are in 0..n.
#[derive(Clone, Debug)]
struct BIT<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> BIT<T> {
    fn new(n: usize, e: T) -> Self {
        BIT { n: n, ary: vec![e.clone(); n + 1], e: e }
    }
    // Usage: self.accum(..idx)
    fn accum(&self, idx: std::ops::RangeTo<usize>) -> T {
        let mut idx = idx.end;
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx].clone();
            idx &= idx - 1;
        }
        sum
    }
    // Usage: self.accum(l..r)
    #[inline(always)]
    fn range(&self, rng: std::ops::Range<usize>) -> T
        where T: std::ops::Sub<Output = T> {
        self.accum(..rng.end) - self.accum(..rng.start)
    }
    // performs data[idx] += val;
    // 0 <= idx, idx < n
    fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where T: std::ops::AddAssign<U> {
        debug_assert!(idx < self.n);
        idx += 1;
        let n = self.n;
        while idx <= n {
            self.ary[idx] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    // Make sure that 0 <= idx < n.
    #[allow(unused)]
    #[inline(always)]
    fn get(&self, idx: usize) -> T
        where T: std::ops::Sub<Output = T> {
        debug_assert!(idx < self.n);
        self.accum(..idx + 1) - self.accum(..idx)
    }
}
/// This implementation of AddAssign is useful when you want to make a 2D BIT.
impl<T: Clone, U: Clone> std::ops::AddAssign<(usize, U)> for BIT<T>
    where T: std::ops::AddAssign<U>,
          T: std::ops::AddAssign<T> {
    fn add_assign(&mut self, (idx, val): (usize, U)) {
        self.add(idx, val);
    }
}

fn main() {
    let n: usize = get();
    let mut s: Vec<u8> = get_word().bytes().collect();
    let q: usize = get();
    let mut freq = [0; 26];
    let mut bits = vec![BIT::new(n, 0i32); 26];
    for i in 0..n {
        let b = (s[i] - b'a') as usize;
        freq[b] += 1;
        bits[b].add(i, 1);
    }
    let mut decr = BIT::new(n - 1, 0i32);
    for i in 0..n - 1 {
        if s[i] > s[i + 1] {
            decr.add(i, 1);
        }
    }
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let x = get::<usize>() - 1;
            let c: char = get();
            let c = c as u8;
            freq[(s[x] - b'a') as usize] -= 1;
            freq[(c - b'a') as usize] += 1;
            bits[(s[x] - b'a') as usize].add(x, -1);
            bits[(c - b'a') as usize].add(x, 1);
            s[x] = c;
            if x > 0 {
                let old = decr.get(x - 1);
                let new = if s[x - 1] > s[x] { 1 } else { 0 };
                decr.add(x - 1, new - old);
            }
            if x < n - 1 {
                let old = decr.get(x);
                let new = if s[x] > s[x + 1] { 1 } else { 0 };
                decr.add(x, new - old);
            }
        } else {
            let l = get::<usize>() - 1;
            let r: usize = get();
            if decr.range(l..r - 1) > 0 {
                println!("No");
                continue;
            }
            let st = (s[l] - b'a') as usize;
            let en = (s[r - 1] - b'a') as usize;
            let ok = bits[st].range(l..r) <= freq[st] && bits[en].range(l..r) <= freq[en]
                && (st + 1..en).all(|i| bits[i].range(l..r) == freq[i]);
            println!("{}", if ok { "Yes" } else { "No" });
        }
    }
}
