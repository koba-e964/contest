use std::io::Write;
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn calc(a: &[usize], k: usize) -> usize {
    let mut hm = std::collections::HashSet::new();
    let mut ans = 0;
    for i in 0..a.len() {
        hm.insert(a[i]);
        if hm.len() > k {
            hm.clear();
            hm.insert(a[i]);
            ans += 1;
        }
    }
    ans + 1
}

fn main() {
    let out = std::io::stdout();
    #[allow(unused)]
    let mut out = std::io::BufWriter::new(out.lock());
    #[allow(unused)]
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    getline();
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();
    let n = a.len();
    a.dedup();
    let mut ans = vec![0; n + 1];
    let mut sq = 1;
    while sq * sq <= a.len() {
        sq += 1;
    }
    sq -= 1;
    for i in 1..=sq {
        ans[i] = calc(&a, i);
    }
    let mut hi = n + 1;
    for q in 1..=sq + 1 {
        let mut pass = hi;
        let mut fail = sq;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if calc(&a, mid) <= q {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        for i in pass..hi {
            ans[i] = q;
        }
        hi = pass;
    }

    for i in 1..=n {
        puts!("{}\n", ans[i]);
    }
}
