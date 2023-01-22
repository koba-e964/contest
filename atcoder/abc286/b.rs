fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let a: Vec<_> = getline().trim().chars().collect();
    let mut b = "".to_string();
    let mut pos = 0;
    while pos < a.len() {
        if pos + 2 <= a.len() && a[pos..pos + 2] == ['n', 'a'] {
            b += "nya";
            pos += 2;
            continue;
        }
        b.push(a[pos]);
        pos += 1;
    }
    println!("{}", b);
}
