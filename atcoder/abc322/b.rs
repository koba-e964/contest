fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().to_string();
    let t = getline().trim().to_string();
    let mut a = 0;
    if t.strip_prefix(&s).is_none() {
        a += 2;
    }
    if t.strip_suffix(&s).is_none() {
        a += 1;
    }
    println!("{}", a);
}
