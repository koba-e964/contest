fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    for i in 0..n {
        print!("{}", s[i]);
        if i + 1 < n {
            print!("o");
        }
    }
    println!();
}
