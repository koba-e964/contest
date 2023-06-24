fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut depth = 0;
    let mut ans = vec![];
    for ch in getline().trim().chars() {
        if ch == '(' {
            depth += 1;
            ans.push(ch);
        } else if ch == ')' {
            if depth > 0 {
                depth -= 1;
                while let Some(p) = ans.pop() {
                    if p == '(' {
                        break;
                    }
                }
            } else {
                ans.push(ch);
            }
        } else {
            ans.push(ch);
        }
    }
    println!("{}", ans.into_iter().collect::<String>());
}
