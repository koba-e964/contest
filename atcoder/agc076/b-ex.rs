fn op(v: Vec<i32>, bits: u64) -> Vec<i32> {
    let mut a = vec![];
    let mut b = vec![];
    let n = v.len();
    for i in 0..n {
        if (bits & 1 << i) != 0 {
            a.push(v[i]);
        } else {
            b.push(v[i]);
        }
    }
    let mut u = vec![];
    for i in 0..n {
        if (bits & 1 << i) != 0 {
            u.push(a.pop().unwrap());
        } else {
            u.push(b.pop().unwrap());
        }
    }
    u
}

fn calc(zero: usize, one: usize) -> usize {
    let mut a = vec![0; zero];
    a.extend_from_slice(&vec![1; one]);
    let n = a.len();
    if n == 0 {
        return 1;
    }
    let mut seen = std::collections::HashSet::new();
    for bits in 0..1 << (n - 1) {
        let b = op(a.clone(), bits);
        if !seen.contains(&b) {
            seen.insert(b);
        }
    }
    seen.len()
}

fn calc2(zero: usize, one: usize) -> usize {
    let mut a = vec![0; zero];
    a.extend_from_slice(&vec![1; one]);
    let n = a.len();
    if n == 0 {
        return 1;
    }
    let mut seen = std::collections::HashSet::new();
    for bits in 0..1 << (n - 1) {
        let b = op(a.clone(), bits);
        if !seen.contains(&b) {
            seen.insert(b);
        }
    }
    let mut seen2 = std::collections::HashSet::new();
    for a in seen {
        for bits in 0..1 << (n - 1) {
            let b = op(a.clone(), bits);
            if !seen2.contains(&b) {
                seen2.insert(b);
            }
        }
    }
    seen2.len()
}

fn calc2a(v: Vec<i32>) -> usize {
    let mut a = vec![0; 7];
    a.extend_from_slice(&vec![1; 6]);
    let n = a.len();
    if n == 0 {
        return 1;
    }
    let mut seen = std::collections::HashSet::new();
    for bits in 0..1 << (n - 1) {
        let b = op(a.clone(), bits);
        if !seen.contains(&b) {
            seen.insert(b);
        }
    }
    for a in seen {
        for bits in 0..1 << (n - 1) {
            let b = op(a.clone(), bits);
            if b == v {
                eprintln!("a = {a:?}");
                break;
            }
        }
    }
    0
}


fn main() {
    let a = vec![0, 0, 0, 0, 1, 1, 1];
    let n = a.len();
    let mut seen = std::collections::HashSet::new();
    for bits in 0..1 << (n - 1) {
        let b = op(a.clone(), bits);
        if !seen.contains(&b) {
            eprintln!("b = {b:?}, bits = {bits}");
            seen.insert(b);
        }
    }
    for zero in 0..10 {
        eprint!("{zero}:");
        for one in 0..15 - zero {
            eprint!(" {}", calc(zero, one));
        }
        eprintln!();
    }

    let mut v = vec![0; 13];
    for i in 0..13 {
        if i % 2 == 1 {
            v[i] = 1;
        }
    }
    calc2a(v);

    eprintln!("rem:");
    for zero in 0..0 {
        eprint!("{zero}:");
        for one in zero..20 - zero {
            let mut comb = 1i64;
            for i in 0..one {
                comb *= (zero + i + 1) as i64;
                comb /= (i + 1) as i64;
            }
            eprint!(" {}", comb - calc2(zero, one) as i64);
        }
        eprintln!();
    }

}
