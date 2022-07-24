mod sol {
    include!("f.rs");
}

fn main() {
    let b = 5;
    let mut pts = vec![];
    for i in 0..b {
        pts.push((0, i));
    }
    for i in 0..b {
        pts.push((i, b));
    }
    for i in 0..b {
        pts.push((b, b - i));
    }
    for i in 0..b {
        pts.push((b - i, 0));
    }
    for (px, py) in pts {
        sol::dist(px, py, b, 3, b);
    }
}
