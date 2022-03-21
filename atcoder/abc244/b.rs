fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;
    for c in getline().trim().chars() {
        if c == 'S' {
            x += dx;
            y += dy;
        } else {
            let (nx, ny) = (dy, -dx);
            dx = nx;
            dy = ny;
        }
    }
    println!("{} {}", x, y);
}
