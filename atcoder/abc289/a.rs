fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    for c in getline().trim().chars() {
        print!("{}", if c == '1' { '0' } else { '1' });
    }
    println!();
}
