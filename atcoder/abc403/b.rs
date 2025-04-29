fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let t: Vec<char> = getline().trim().chars().collect();
    let u: Vec<char> = getline().trim().chars().collect();
    for i in 0..t.len() - u.len() + 1 {
        let view = t[i..i + u.len()].to_vec();
        if (0..u.len()).all(|i| view[i] == '?' || view[i] == u[i]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
