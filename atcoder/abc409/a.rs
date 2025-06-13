fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let t = getline().trim().to_string();
    let a = getline().trim().to_string();
    for (a, b) in t.chars().zip(a.chars()) {
        if a == b && a == 'o' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
