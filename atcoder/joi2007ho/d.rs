use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Tags: topological-sort
fn main() {
    let n: usize = get();
    let m: usize = get();
    let mut g = vec![vec![]; n];
    let mut indeg = vec![0; n];
    for _ in 0..m {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        g[a].push(b);
        indeg[b] += 1;
    }
    let mut st = vec![];
    for i in 0..n {
        if indeg[i] == 0 {
            st.push(i);
        }
    }
    let mut mul = false;
    let mut ord = vec![];
    while let Some(v) = st.pop() {
        if !st.is_empty() {
            mul = true;
        }
        ord.push(v);
        for &w in &g[v] {
            indeg[w] -= 1;
            if indeg[w] == 0 {
                st.push(w);
            }
        }
    }
    for i in 0..n {
        println!("{}", ord[i] + 1);
    }
    println!("{}", if mul { 1 } else { 0 });
}
