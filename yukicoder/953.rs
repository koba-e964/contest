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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

// 0: nearest, 1: the second nearest
fn pri(v: usize, k1: usize, k2: usize) -> usize {
    if v == k1 {
        return 0;
    }
    if (v < k1) != (k2 < k1) {
        return ((v as i32 - k1 as i32).abs() * 2) as usize;
    }
    ((v as i32 - k1 as i32).abs() * 2 - 1) as usize
}

fn assign(r: &mut BTreeSet<(usize, usize)>,
          r2: &mut BTreeSet<(usize, usize)>,
          k1: usize, k2: usize,
          n: usize) -> usize {
    if r2.is_empty() {
        let &mi = r.iter().next().unwrap();
        r.remove(&mi);
        return mi.1;
    }
    let &mi = r2.iter().next().unwrap();
    r2.remove(&mi);
    r.remove(&mi);
    // neighbors
    let mut indices = vec![];
    if mi.1 + 1 < n {
        indices.push(mi.1 + 1);
    }
    if mi.1 > 0 {
        indices.push(mi.1 - 1);
    }
    for idx in indices {
        let e = (pri(idx, k1, k2), idx);
        r2.remove(&e);
    }
    mi.1
}

fn fix(r: &BTreeSet<(usize, usize)>,
       r2: &mut BTreeSet<(usize, usize)>,
       v: usize, n: usize, k1: usize, k2: usize) {
    let mut ok = true;
    if v >= 1 {
        ok &= r.contains(&(pri(v - 1, k1, k2), v - 1));
    }
    if v + 1 < n {
        ok &= r.contains(&(pri(v + 1, k1, k2), v + 1));
    }
    ok &= r.contains(&(pri(v, k1, k2), v));
    let e = (pri(v, k1, k2), v);
    if ok && !r2.contains(&e) {
        r2.insert(e);
    }
    if !ok && r2.contains(&e) {
        r2.remove(&e);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, k1: usize1, k2: usize1,
        q: usize,
        ab: [(i64, i64); q],
    }
    let mut ev: BTreeMap<_, (Vec<usize>, Vec<usize>)> = BTreeMap::new();
    for i in 0..q {
        let (a, _b) = ab[i];
        let e = ev.entry(a).or_insert((vec![], vec![]));
        e.0.push(i);
        /*
        let e = ev.entry(a + b).or_insert((vec![], vec![]));
        e.1.push(i);
         */
    }
    let mut r2 = BTreeSet::new();
    let mut r = BTreeSet::new();
    for i in 0..n {
        r2.insert((pri(i, k1, k2), i));
        r.insert((pri(i, k1, k2), i));
    }
    let mut ans = vec![n + 1; q];
    let mut waiting = VecDeque::new();
    let mut cur = -1;
    loop {
        let x = ev.range(cur + 1..).next();
        if x.is_none() {
            break;
        }
        let (&time, (come, leave)) = x.unwrap();
        cur = time;
        let come = come.clone();
        let leave = leave.clone();
        /*
        eprintln!("time = {}, ans = {:?}", time, ans);
        eprintln!("r = {:?}, r2 = {:?}", r, r2);
         */
        for c in come {
            waiting.push_back(c);
        }
        for idx in leave {
            let pos = ans[idx % q];
            assert!(pos < n);
            // eprintln!("leaving: {} {}", idx, pos);
            r.insert((pri(pos, k1, k2), pos));
            let mut indices = vec![pos];
            if pos + 1 < n {
                indices.push(pos + 1);
            }
            if pos > 0 {
                indices.push(pos - 1);
            }
            for idx in indices {
                fix(&r, &mut r2, idx, n, k1, k2);
            }
            //eprintln!("r,r2={:?}, {:?}", r, r2);
        }
        while !r.is_empty() && !waiting.is_empty() {
            if let Some(v) = waiting.pop_front() {
                //eprintln!("assigning {}, {:?}, {:?}", v, r, r2);
                let pos = assign(&mut r, &mut r2, k1, k2, n);
                ans[v] = pos;
                let ent = ev.entry(cur + ab[v].1).or_insert((vec![], vec![]));
                ent.1.push(v);
                //eprintln!("assigning {} -> {}, {:?}, {:?}", v, pos, r, r2);
            }
        }
    }
    for i in 0..q {
        puts!("{}\n", ans[i] + 1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
