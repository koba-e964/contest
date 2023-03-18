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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn sa_doubling(s: &[i32]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rnk: Vec<i32> = s.to_vec();
    let mut tmp = vec![0; n];
    let mut k = 1;
    while k < n {
        let cmp = |&x: &usize, &y: &usize| {
            if rnk[x] != rnk[y] {
                return rnk[x].cmp(&rnk[y]);
            }
            let rx = if x + k < n { rnk[x + k] } else { -1 };
            let ry = if y + k < n { rnk[y + k] } else { -1 };
            rx.cmp(&ry)
        };
        sa.sort_by(cmp);
        tmp[sa[0]] = 0;
        for i in 1..n {
            tmp[sa[i]] = tmp[sa[i - 1]]
                + if cmp(&sa[i - 1], &sa[i]) == std::cmp::Ordering::Less {
                    1
                } else {
                    0
                };
        }
        std::mem::swap(&mut tmp, &mut rnk);
        k *= 2;
    }
    sa
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, k: usize,
        p: [i32; n],
    }
    let fst = ((k - 1) / n) as i32;
    let mut diff = vec![0; 2 * n];
    for i in 0..2 * n {
        diff[i] = (-p[i % n] + n as i32 + p[(i + 1) % n]) % n as i32;
    }
    let sa = sa_doubling(&diff);
    let mut sa_comp = vec![];
    for s in sa {
        if s < n {
            sa_comp.push(s);
        }
    }
    let mut l = 0;
    let mut r = n;
    let mut cur = fst;
    let mut rem = (k - 1) % n;
    for pos in 1..n {
        // search n - cur..n
        let mut pass = r + 1;
        let mut fail = l;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            let targ = diff[(sa_comp[mid - 1] + pos - 1) % n];
            if targ >= n as i32 - cur {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let pass = pass - 1;
        let now = if r - pass > rem {
            l = pass;
            diff[(sa_comp[pass + rem] + pos - 1) % n]
        } else {
            rem -= r - pass;
            r = pass;
            diff[(sa_comp[l + rem] + pos - 1) % n]
        };
        let mut pass = l;
        let mut fail = r;
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            let targ = diff[(sa_comp[mid] + pos - 1) % n];
            if targ <= now {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let hi = pass + 1;
        let mut pass = r;
        let mut fail = l;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            let targ = diff[(sa_comp[mid - 1] + pos - 1) % n];
            if targ >= now {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let lo = pass - 1;
        rem -= lo - l;
        l = lo;
        r = hi;
        cur = (cur + now) % n as i32;
    }
    let mut ans = vec![];
    let st = sa_comp[l];
    for i in 0..n {
        ans.push((p[(st + i) % n] + n as i32 + fst - p[st]) % n as i32);
    }
    putvec!(ans);
}
