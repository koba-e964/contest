fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    println!("{}", getline().trim().chars().filter(|&x| x.is_numeric()).collect::<String>());
}
