fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut x = '+';
    let mut y = 0;
    for i in 0..8 {
        let s: Vec<_> = getline().chars().collect();
        for j in 0..8 {
            if s[j] == '*' {
                x = (b'a' + j as u8) as char;
                y = 8 - i;
            }
        }
    }
    println!("{}{}", x, y);
}
