fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_string();
    let mut ans = "".to_string();
    let mut ok = true;
    for c in s.chars() {
        if c == '.' {
            if ok {
                ans.push('o');
                ok = false;
            } else {
                ans.push('.');
            }
        } else {
            ans.push(c);
            ok = true;
        }
    }
    println!("{ans}");
}
