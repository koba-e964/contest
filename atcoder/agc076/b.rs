use std::io::{Write, BufWriter};
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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn perform(mut y: Vec<i32>, v: &[bool]) -> Vec<i32> {
    let n = y.len();
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        if v[i] {
            a.push(y[i]);
        } else {
            b.push(y[i]);
        }
    }
    y.clear();
    for i in 0..n {
        if v[i] {
            y.push(a.pop().unwrap());
        } else {
            y.push(b.pop().unwrap());
        }
    }
    y
}


fn calc1(x: Vec<i32>) -> Option<Vec<bool>> {
    let mut zero = 0;
    let mut one = 0;
    for &c in &x {
        if c == 0 {
            zero += 1;
        } else {
            one += 1;
        }
    }
    let mut t = vec![0; zero];
    t.extend_from_slice(&vec![1; one]);
    if x == t {
        return None;
    }
    let n = x.len();
    let mut s1 = n;
    for i in zero..n {
        if x[i] == 1 {
            s1 = i;
        }
    }
    if s1 == n {
        // ok
        let mut v = vec![false; n];
        for i in 0..zero {
            if x[i] == 1 {
                v[i] = true;
            }
        }
        for i in zero..n {
            v[i] = true;
        }
        return Some(v);
    }
    let mut mand = 0;
    for i in zero..s1 {
        if x[i] == 0 {
            mand += 1;
        }
    }
    assert!(mand <= zero);
    if x[..mand] != vec![1; mand] {
        return None;
    }
    let mut v = vec![false; n];
    for i in mand..zero {
        if x[i] == 1 {
            v[i] = true;
        }
    }
    for i in zero..s1 + 1 {
        if x[i] == 1 {
            v[i] = true;
        }
    }
    for i in s1 + 1..n {
        v[i] = true;
    }
    Some(v)
}

fn facil(x: Vec<i32>) -> (Vec<i32>, Vec<bool>) {
    let n = x.len();
    let mut zero = 0;
    let mut one = 0;
    for &c in &x {
        if c == 0 {
            zero += 1;
        } else {
            one += 1;
        }
    }
    assert!(zero >= one);
    // editorial
    let mut p = 0;
    for i in 0..zero {
        if x[i] == 0 {
            p += 1;
        }
    }
    let mut op = vec![false; n];
    for i in 0..zero {
        op[i] = x[i] == 0;
    }
    for i in n - p..n {
        op[i] = true;
    }
    let y = perform(x, &op);
    (y, op)
}

pub fn calc(mut x: Vec<i32>) -> Vec<Vec<bool>> {
    let mut zero = 0;
    let mut one = 0;
    for &c in &x {
        if c == 0 {
            zero += 1;
        } else {
            one += 1;
        }
    }
    let mut t = vec![0; zero];
    t.extend_from_slice(&vec![1; one]);
    if x == t {
        return vec![];
    }
    let n = x.len();
    if let Some(ans) = calc1(x.clone()) {
        return vec![ans];
    }
    let flip =  zero < one ;
    if flip {
        for i in 0..n {
            x[i] = 1 - x[i];
        }
        x.reverse();
    }
    if let Some(mut ans) = calc1(x.clone()) {
        if flip {
            ans.reverse();
        }
        return vec![ans];
    }
    let (y, mut op1) = facil(x);
    let mut op2 = calc1(y.clone()).unwrap();
    if flip {
        op1.reverse();
        op2.reverse();
    }

    vec![op1, op2]
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: i32,
        x: [[i32]; t],
    }
    for x in x {
        let n = x.len();
        let res = calc(x);
        puts!("{}\n", res.len());
        for r in res {
            for i in 0..n {
                puts!("{}", if r[i] { 'A' } else { 'B' });
            }
            puts!("\n");
        }
    }
}
