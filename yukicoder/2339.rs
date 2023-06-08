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

fn main() {
    let n: usize = get();
    if n == 1 {
        println!("1 1\n.");
        return;
    }
    let mut blocks = vec![];
    let mut st = vec![n];
    let mut h = 0;
    let mut w = 0;
    while let Some(v) = st.pop() {
        if v <= 1 { continue; }
        let a = v / 2;
        let b = v - a;
        blocks.push((a, b));
        h += a;
        w += b;
        st.push(a);
        st.push(b);
    }
    println!("{} {}", h + 1, w + 1);
    let mut s = vec![vec!['#'; w + 1]; h + 1];
    let mut x = 0;
    let mut y = 0;
    for (a, b) in blocks {
        for i in x..x + a + 1 {
            for j in y..y + b + 1 {
                s[i][j] = '.';
            }
        }
        x += a;
        y += b;
    }
    for i in 0..h + 1 {
        println!("{}", s[i].iter().cloned().collect::<String>());
    }
}
