fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut x = -1;
    for (i, c) in getline().chars().enumerate() {
        if c == 'a' {
            x = i as i32 + 1;
        }
    }
    println!("{}", x);
}
