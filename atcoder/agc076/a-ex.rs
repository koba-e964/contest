fn ok(targ: &[usize], n: usize, mut path: Vec<usize>) -> Option<()> {
    if path.len() >= n * n {
        for i in 0..n {
            let ide = (0..n * n).filter(|&j| targ[n * n * i + j] == path[j]).count();
            if ide <= n {
                return Some(());
            }
        }
        eprintln!("{targ:?} path = {path:?}");
        return None;
    }
    for i in 0..n {
        path.push(i);
        ok(targ, n, path.clone())?;
        path.pop();
    }
    Some(())
}

fn rec(n: usize, mut path: Vec<usize>) {
    if path.len() >= n * n * n {
        if ok(&path, n, vec![]).is_some() {
            println!("{path:?}");
            if n == 2 {
                let mut xor = vec![0; n * n];
                for i in 0..n * n {
                    xor[i] = path[i] ^ path[i + n * n];
                }
                eprintln!("xor = {xor:?}");
            }
        }
        return;
    }
    for i in 0..n {
        path.push(i);
        rec(n, path.clone());
        path.pop();
    }
}
fn main() {
    for n in 1..=0 {
        rec(n, vec![]);
    }
    let mut ans = vec![0; 27];
    for i in 12..21 {
        ans[i] = 1;
    }
    for i in 21..27 {
        ans[i] = 2;
    }
    ok(&ans, 3, vec![]);
}
