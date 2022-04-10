#[allow(unused)]
mod orig {
    include!("j.rs");
    pub fn calc2(k: i64) -> usize {
        calc(k)
    }
    pub fn extract2(n: i64) -> Option<(i64, i64)> { extract(n) }
}

fn naive(k: i64) -> usize {
    let mut ans = 0;
    for n in 2..10000i64 {
        let diff = n * n - 1;
        if diff % k != 0 { continue; }
        let pm = diff / k;
        if let Some((_p, m)) = orig::extract2(pm) {
            eprintln!("k = {}, p = {}", k, _p);
            if m >= 1 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    for k in 1..20 {
        let ex = naive(k);
        let act = orig::calc2(k);
        if ex != act {
            eprintln!("{}: ex = {}, act = {}", k, ex, act);
        }
    }
}
