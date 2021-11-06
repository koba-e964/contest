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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// E = A|AorE
// A = P|PandA
// P = YES|NO|(E)|random(EE)

fn ee(s: &[char], p: f64, q: f64, r: f64) -> (usize, f64) {
    let (l, mut v) = ee_sub(s, p, q, r);
    let mut x = v.pop().unwrap();
    while let Some(y) = v.pop() {
        x = (1.0 - r) * (x + y - x * y) + r * (1.0 - x - y + x * y);
    }
    (l, x)
}

fn ee_sub(s: &[char], p: f64, q: f64, r: f64) -> (usize, Vec<f64>) {
    let len = s.len();
    let (l0, x0) = aa(s, p, q, r);
    if l0 + 2 <= len && s[l0..l0 + 2] == ['o', 'r'] {
        let (l1, mut x1) = ee_sub(&s[l0 + 2..], p, q, r);
        x1.push(x0);
        return (l0 + 2 + l1, x1);
    }
    (l0, vec![x0])
}

fn aa(s: &[char], p: f64, q: f64, r: f64) -> (usize, f64) {
    let (l, mut v) = aa_sub(s, p, q, r);
    let mut x = v.pop().unwrap();
    while let Some(y) = v.pop() {
        x = (1.0 - r) * x * y + r * (1.0 - x * y);
    }
    (l, x)
}

fn aa_sub(s: &[char], p: f64, q: f64, r: f64) -> (usize, Vec<f64>) {
    let len = s.len();
    let (l0, x0) = pp(s, p, q, r);
    if l0 + 3 <= len && s[l0..l0 + 3] == ['a', 'n', 'd'] {
        let (l1, mut x1) = ee_sub(&s[l0 + 3..], p, q, r);
        x1.push(x0);
        return (l0 + 3 + l1, x1);
    }
    (l0, vec![x0])
}

fn pp(s: &[char], p: f64, q: f64, r: f64) -> (usize, f64) {
    let len = s.len();
    if len >= 3 && s[..3] == ['Y', 'E', 'S'] {
        return (3, 1.0);
    }
    if len >= 2 && s[..2] == ['N', 'O'] {
        return (2, 0.0);
    }
    if len >= 7 && s[..7] == ['r', 'a', 'n', 'd', 'o', 'm', '('] {
        let (l0, x0) = ee(&s[7..], p, q, r);
        let (l1, x1) = ee(&s[7 + l0..], p, q, r);
        assert_eq!(s[7 + l0 + l1], ')');
        return (7 + l0 + l1 + 1, p * x0 * x1 + q * (1.0 - x0 * x1));
    }
    assert_eq!(s[0], '(');
    let (l, x) = ee(&s[1..], p, q, r);
    assert_eq!(s[1 + l], ')');
    (l + 2, x)
}

// Tags: parsing
fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 3 * 104_857_600; // 300 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    input! {
        _n: usize,
        p: f64, q: f64, r: f64,
        s: chars,
    }
    let (_, ans) = ee(&s, p, q, r);
    println!("{}", (ans * 100.0 + 0.5).floor() as i64);
}
