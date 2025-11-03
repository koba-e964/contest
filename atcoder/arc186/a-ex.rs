use std::collections::*;

fn ex(n: usize) {
    let mut hm = HashMap::new();
    for bits in 0..1 << (n * n) {
        let mut row = vec![0; n];
        let mut col = vec![0; n];
        for i in 0..n * n {
            if (bits & 1 << i) != 0 {
                row[i / n] += 1;
                col[i % n] += 1;
            }
        }
        hm.entry((row, col)).or_insert(vec![]).push(bits);
    }
    let mut seen = HashSet::new();
    for (_, v) in hm {
        let mut mi = (1 << (n * n)) - 1;
        let mut ma = 0u64;
        for &v in &v {
            mi &= v;
            ma |= v;
        }
        let unfixed = (mi ^ ma).count_ones();
        if !seen.contains(&unfixed) {
            seen.insert(unfixed);
            eprintln!("{n} {unfixed} =>");
            for i in 0..n {
                for j in 0..n {
                    eprint!("{}", if (v[0] & 1 << (i * n + j)) != 0 {
                        1
                    } else {
                        0
                    });
                }
                eprintln!();
            }
            eprintln!("result:");
            for i in 0..n {
                for j in 0..n {
                    eprint!("{}", if ((mi ^ ma) & 1 << (i * n + j)) != 0 {
                        '?'
                    } else if (ma & 1 << (i * n + j)) != 0 {
                        '1'
                    } else {
                        '0'
                    });
                }
                eprintln!();
            }
        }
    }
}

fn main() {
    for n in 5..=5 {
        ex(n);
    }
}
