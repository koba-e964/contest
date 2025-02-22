fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut ans = "".to_string();
    let mut w = 0;
    for c in getline().trim().chars() {
        if c == 'W' {
            w += 1;
        } else if w > 0 && c == 'A' {
            ans.push('A');
            for _ in 0..w {
                ans.push('C');
            }
            w = 0;
        } else {
            for _ in 0..w {
                ans.push('W');
            }
            w = 0;
            ans.push(c);
        }
    }
    for _ in 0..w {
        ans.push('W');
    }
    println!("{ans}");
}
