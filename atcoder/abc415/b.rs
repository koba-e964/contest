fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline();
    let mut inds = Vec::new();
    for (i, c) in s.trim().chars().enumerate() {
        if c == '#' {
            inds.push(i + 1);
        }
    }
    for i in 0..inds.len() / 2 {
        println!("{},{}", inds[i * 2], inds[i * 2 + 1]);
    }
}
