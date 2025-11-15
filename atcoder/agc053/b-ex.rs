use std::collections::*;

fn dfs(n: usize, me: u64, rem: u64, seen: &mut HashSet<u64>) {
    if rem == 0 {
        seen.insert(me);
        return;
    }
    for i in 0..2 * n {
        if (rem & 1 << i) == 0 {
            continue;
        }
        let mut ids = vec![];
        for j in 0..2 * n {
            if i != j && (rem & 1 << j) != 0 {
                ids.push(j);
            }
        }
        let mid = ids[ids.len() / 2];
        dfs(n, me | 1 << i, rem ^ 1 << i ^ 1 << mid, seen);
    }
}

fn main() {
    for n in 2..=4 {
        let mut seen = HashSet::new();
        dfs(n, 0, (1 << (2 * n)) - 1, &mut seen);
        for (c, s) in seen.into_iter().enumerate() {
            eprint!("{n} {c}: ");
            for i in 0..2 * n {
                if (s & 1 << i) != 0 {
                    eprint!(" {}", i);
                }
            }
            eprintln!();
        }
    }
}
