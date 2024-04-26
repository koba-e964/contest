fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().chars().filter(|&x| !b"aeiuo".contains(&(x as u8))).collect::<String>();
    print!("{}", s);
}
