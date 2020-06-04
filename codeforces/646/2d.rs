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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

trait Interactor {
    fn ask(&self, v: &[usize]) -> usize;
    fn test(&self, pass: &[usize]);
}

struct Real;

impl Interactor for Real {
    fn ask(&self, v: &[usize]) -> usize {
        print!("? {}", v.len());
        for &v in v {
            print!(" {}", v + 1);
        }
        println!();
        get::<usize>() - 1
    }
    fn test(&self, a: &[usize]) {
        print!("!");
        for &a in a {
            print!(" {}", a + 1);
        }
        println!();
        let w = get_word();
        assert_eq!(w, "Correct");
    }
}

struct Mock {
    a: Vec<usize>,
    s: Vec<Vec<usize>>,
}

impl Interactor for Mock {
    fn ask(&self, v: &[usize]) -> usize {
        let mut ma = 0;
        for &v in v {
            ma = max(ma, self.a[v]);
        }
        ma
    }
    fn test(&self, pass: &[usize]) {
        let n = self.a.len();
        let k = self.s.len();
        assert_eq!(pass.len(), k);
        let mut realp = vec![0; k];
        for i in 0..k {
            let mut seen = HashSet::new();
            for &s in &self.s[i] {
                seen.insert(s);
            }
            let mut ma = 0;
            for i in 0..n {
                if !seen.contains(&i) {
                    ma = max(ma, self.a[i]);
                }
            }
            realp[i] = ma;
        }
        assert_eq!(pass, &realp[..]);
    }
}

fn calc(int: &dyn Interactor, n: usize, s: Vec<Vec<usize>>) {
    // First find what the max is
    let v: Vec<_> = (0..n).collect();
    let max = int.ask(&v);
    // First find where ma is
    let mut lo = 0;
    let mut hi = n;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        let v: Vec<_> = (lo..mid).collect();
        let ma = int.ask(&v);
        if ma == max {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    // a[lo] == max; for s containing lo, let's find p for s.
    let k = s.len();
    let mut pass = vec![max; k];
    for i in 0..k {
        if s[i].iter().any(|&x| x == lo) {
            let mut rem: HashSet<_> = (0..n).collect();
            for &s in &s[i] {
                rem.remove(&s);
            }
            let vec: Vec<_> = rem.into_iter().collect();
            pass[i] = int.ask(&vec);
        }
    }
    int.test(&pass);
}

fn stress() {
    let n = 4;
    let s = vec![vec![0, 2], vec![1, 3]];
    let mut a = vec![0; n];
    for i in 0..n { a[i] = i; }
    let int = Mock {
        a: a.clone(), s: s.clone(),
    };
    calc(&int, n, s);
    let s = vec![vec![1, 2], vec![0, 3]];
    let int = Mock {
        a, s: s.clone(),
    };
    calc(&int, n, s);
    let mut x: usize = 1333;
    let mut nxt = || {
        x = x.wrapping_mul(0x45555551).wrapping_add(1);
        x
    };
    for _ in 0..20000 {
        let n = 159;
        let mut a: Vec<_> = vec![0; n];
        for i in 0..n {
            a[i] = nxt() % n;
        }
        let k = nxt() % 98 + 2;
        let mut s = vec![vec![]; k];
        for i in 0..n {
            let idx = nxt() % (k + 1);
            if idx < k {
                s[idx].push(i);
            }
        }
        let int = Mock {
            a, s: s.clone(),
        };
        calc(&int, n, s);
    }
}

fn solve() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "stress" {
        stress();
        return;
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let k: usize = get();
        let mut s = vec![vec![]; k];
        for i in 0..k {
            let c: usize = get();
            let mut v = vec![];
            for _ in 0..c {
                let x = get::<usize>() - 1;
                v.push(x);
            }
            s[i] = v;
        }
        calc(&Real, n, s);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
