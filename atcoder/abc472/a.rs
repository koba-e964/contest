fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_string();
    println!("{}", s.chars().map(|x| if x == 'A' { x } else { '.' }).collect::<String>());
}
