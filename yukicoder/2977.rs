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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const D: usize = 30;

struct Trie {
    l: Option<Box<Trie>>,
    r: Option<Box<Trie>>,
    car: i32,
}

fn new() -> Trie {
    Trie {
        l: None,
        r: None,
        car: 0,
    }
}

fn add(t: &mut Trie, dep: usize, a: i32) {
    if dep == D {
        t.car += 1;
        return;
    }
    let b = (a >> (D - 1 - dep)) & 1;
    if b == 0 {
        if t.l.is_none() {
            t.l = Some(Box::new(new()));
        }
        add(t.l.as_mut().unwrap(), dep + 1, a);
    } else {
        if t.r.is_none() {
            t.r = Some(Box::new(new()));
        }
        add(t.r.as_mut().unwrap(), dep + 1, a);
    }
    t.car += 1;
}

// #{x | x xor a < b}
fn xor_lt(t: &Trie, dep: usize, a: i32, b: i32) -> i32 {
    if dep == D {
        return 0;
    }
    let bi = (b >> (D - 1 - dep)) & 1;
    let ai = (a >> (D - 1 - dep)) & 1;
    let mut res = 0;
    let (l, r) = (t.l.as_ref(), t.r.as_ref());
    let (l, r) = if ai == 0 {
        (l, r)
    } else {
        (r, l)
    };
    if let Some(l) = l {
        if bi == 0 {
            res += xor_lt(l, dep + 1, a, b);
        } else {
            res += l.car;
        }
    }
    if let Some(r) = r {
        if bi == 1 {
            res += xor_lt(r, dep + 1, a, b);
        }
    }
    res
}

// https://yukicoder.me/problems/no/2977 (3)
// A_i xor A_j (i, j に制限なし) の下から 2K + N 番目を求めれば良い。
// 二分探索して Trie でできる。
// 下から y 番目は、 #{x | x < k} < y となる最大の k である。
// Tags: trie, nth-element-with-binary-search
fn main() {
    input! {
        n: usize, k: i64,
        a: [i32; n],
    }
    let mut trie = new();
    for &a in &a {
        add(&mut trie, 0, a);
    }
    let mut pass = -1;
    let mut fail = 1 << D;
    while fail - pass > 1 {
        let mid = (pass - fail) / 2 + fail;
        let mut cnt = 0i64;
        for &a in &a {
            cnt += xor_lt(&trie, 0, a, mid) as i64;
        }
        if cnt < 2 * k + n as i64 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{pass}");
}
