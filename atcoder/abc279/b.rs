fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let a = getline().trim().to_owned();
    let b = getline().trim().to_owned();
    for i in b.len()..a.len() + 1 {
        if a[i - b.len()..i] == b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
