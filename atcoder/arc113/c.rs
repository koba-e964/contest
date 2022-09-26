fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut st = vec![];
    let mut tot = 0i64;
    for c in getline().trim().chars().rev() {
        if let Some((d, e)) = st.pop() {
            if d == c {
                st.push((d, e + 1));
            } else {
                st.push((d, e));
                st.push((c, 1));
            }
        } else {
            st.push((c, 1));
        }
        if let Some((d, mut e)) = st.pop() {
            if e < 2 {
                st.push((d, e));
                continue;
            }
            while let Some((x, y)) = st.pop() {
                e += y;
                if x != d {
                    tot += y;
                }
            }
            st.push((d, e));
        }
    }
    println!("{}", tot);
}
