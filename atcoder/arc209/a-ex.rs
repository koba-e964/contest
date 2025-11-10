fn is_valid(s: &[i32]) -> bool {
    let n = s.len();
    let mut delta = 0;
    let mut mi = 0;
    for i in 0..n {
        delta += s[i];
        mi = mi.min(delta);
    }
    delta == 0 && mi >= 0
}

fn win(s: &[i32], first: bool, k: usize) -> bool {
    if s.len() <= k {
        return first ^ is_valid(s);
    }
    if first {
        if !is_valid(s) {
            return true;
        }
    } else {
        if is_valid(s) {
            return true;
        }
    }
    for i in [0, s.len() - 1] {
        let mut t = s.to_vec();
        t.remove(i);
        if !win(&t, !first, k) {
            return true;
        }
    }
    false
}


fn main() {
    for n in 2..=10 {
        if n % 2 != 0 { continue; }
        for bits in 0..1 << n {
            let mut s = vec![];
            for i in 0..n {
                s.push(if (bits & 1 << i) != 0 { 1 } else { -1 });
            }
            if !is_valid(&s) {
                continue;
            }
            let mut seq = vec![];
            for k in 2..=n {
                if k % 2 != 0 { continue; }
                if win(&s, true, k) {
                    seq.push(k);
                }
            }
            if !seq.is_empty() {
                for i in 0..n {
                    print!("{}", if (bits & 1 << i) != 0 { '(' } else { ')' });
                }
                print!("k = {seq:?}");
                println!();
            }
        }
    }
}
