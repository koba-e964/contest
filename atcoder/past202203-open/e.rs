fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_owned();
    let mut ans = "3000/03/03".to_string();
    for bits in 0..8 {
        let mut y = 2;
        for i in 0..3 {
            y = 10 * y + if (bits & 1 << i) != 0 { 2 } else { 0 };
        }
        for i in &["02", "20", "22"] {
            let tmp = format!("{}/02/{}", y, i);
            if s <= tmp && tmp < ans {
                ans = tmp;
            }
        }
    }
    for bits in 0..8 {
        let mut y = 2;
        for i in 0..3 {
            y = 10 * y + if (bits & 1 << i) != 0 { 2 } else { 1 };
        }
        for m in &["11", "12"] {
            for i in &["11", "12", "21", "22"] {
                let tmp = format!("{}/{}/{}", y, m, i);
                if s <= tmp && tmp < ans {
                    ans = tmp;
                }
            }
        }
    }
    println!("{}", ans);
}
