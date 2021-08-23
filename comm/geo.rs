type Coord = i64; // the type of coordinates
type P = (Coord, Coord);

fn inn((ax, ay): P, (bx, by): P) -> Coord {
    ax * bx + ay * by
}

fn det((ax, ay): P, (bx, by): P) -> Coord {
    ax * by - ay * bx
}

fn sub((ax, ay): P, (bx, by): P) -> P {
    (ax - bx, ay - by)
}
