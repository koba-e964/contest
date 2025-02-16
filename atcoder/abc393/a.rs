fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let l = getline().trim().to_string();
    let s = l.split(' ').collect::<Vec<_>>();
    let mut ans = 1;
    if s[0] == "fine" {
        ans += 2;
    }
    if s[1] == "fine" {
        ans += 1;
    }
    println!("{ans}");
}
