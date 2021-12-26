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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let n: usize = get();
    let q: usize = get();
    let mut pre = vec![n; n];
    let mut nxt = vec![n; n];
    for _ in 0..q {
        let ty: i32 = get();
        let x = get::<usize>() - 1;
        if ty == 3 {
            let mut v = x;
            while pre[v] != n {
                v = pre[v];
            }
            let mut ans = vec![];
            while v != n {
                ans.push(v);
                v = nxt[v];
            }
            print!("{}", ans.len());
            for i in 0..ans.len() {
                print!(" {}", ans[i] + 1);
            }
            println!();
        } else if ty == 1 {
            let y = get::<usize>() - 1;
            pre[y] = x;
            nxt[x] = y;
        } else {
            let y = get::<usize>() - 1;
            pre[y] = n;
            nxt[x] = n;
        }
    }
}
