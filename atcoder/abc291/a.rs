fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline();
    for (i, c) in s.chars().enumerate() {
        if c <= 'Z' {
            println!("{}", i + 1);
            return;
        }
    }
}
