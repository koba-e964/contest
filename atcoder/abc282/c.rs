fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut quo = false;
    for c in getline().trim().chars() {
        if !quo && c == ',' {
            print!(".");
        } else {
            print!("{}", c);
        }
        if c == '"' {
            quo = !quo;
        }
    }
    println!();
}
