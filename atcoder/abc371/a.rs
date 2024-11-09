fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_string();
    if s == "< < <" || s == "> > >" {
        println!("B");
    }
    if s == "> < <" || s == "< > >" {
        println!("A");
    }
    if s == "< < >" || s == "> > <" {
        println!("C");
    }
}
