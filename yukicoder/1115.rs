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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn inversion_by_merge(a: &mut [usize]) -> i64 {
    if a.len() <= 1 {
        return 0;
    }
    let mut ans = 0;
    let mut tmp = Vec::with_capacity(a.len());
    {
        let mid = a.len() / 2;
        let (x, y) = a.split_at_mut(mid);
        ans += inversion_by_merge(x);
        ans += inversion_by_merge(y);
        let mut p0 = 0;
        let mut p1 = 0;
        while p0 < x.len() && p1 < y.len() {
            if x[p0] <= y[p1] {
                tmp.push(x[p0]);
                p0 += 1;
            } else {
                tmp.push(y[p1]);
                p1 += 1;
                ans += (x.len() - p0) as i64;
            }
        }
        while p0 < x.len() {
            tmp.push(x[p0]);
            p0 += 1;
        }
        while p1 < y.len() {
            tmp.push(y[p1]);
            p1 += 1;
        }
    }
    for i in 0..a.len() {
        a[i] = tmp[i];
    }
    ans
}

// Tags: inversion-count
fn main() {
    input! {
        n: usize,
        a: [usize1; n],
        b: [usize1; n],
    }
    let mut inva = vec![0; n];
    for i in 0..n {
        inva[a[i]] = i;
    }
    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = inva[b[i]];
    }
    println!("{}", inversion_by_merge(&mut p));
}
