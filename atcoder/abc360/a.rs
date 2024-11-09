fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline();
    if s.contains("RM") || s.contains("RSM") {
        println!("Yes");
    } else {
        println!("No");
    }
}
