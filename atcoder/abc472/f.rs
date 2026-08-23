fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, q] = ints[..] else { panic!() };
    let mut pts = vec![];
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [a, b] = ints[..] else { panic!() };
        pts.push((a, b));
    }
    let mut qs = vec![];
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>();
        let [a, b] = ints[..] else { panic!() };
        let b = if a > b { b + n } else { b };
        qs.push((a, b));
    }
    const B: usize = 250;
    let mut lri: Vec<_> = (0..q).map(|i| {
        let (l, r) = qs[i];
        (l, r, i)
    }).collect();
    lri.sort_by_key(|&(l, r, _idx)| {
        let q = l / B;
        if q % 2 == 1 {
            (q, n - r)
        } else {
            (q, r)
        }
    });

    let get = |a: usize, b: usize, c: usize| {
        let a = a % n;
        let b = b % n;
        let c = c % n;
        let (xa, ya) = pts[a];
        let (xb, yb) = pts[b];
        let (xc, yc) = pts[c];

        let x = xa + xb + xc;
        let y = ya + yb + yc;

        let (xb, yb) = (xb - xa, yb - ya);
        let (xc, yc) = (xc - xa, yc - ya);

        let area = xb * yc - xc * yb;
        (x, y, area)
    };

    let mut ans = vec![(0.0, 0.0); q];
    
    // pointer
    let mut cl = 0;
    let mut cr = 0;
    
    // state
    // 座標*(2*面積)の3倍
    let mut gx = 0i64;
    let mut gy = 0i64;
    // 2*面積
    let mut area = 0i64;
    
    for &(l, r, idx) in &lri {
        while cr < r {
            // add cr
            let (x, y, a) = get(cl, cr, cr + 1);
            gx += x * a;
            gy += y * a;
            area += a;
            cr += 1;
        }
        while cl > l {
            cl -= 1;
            // add cl
            let (x, y, a) = get(cl, cl + 1, cr);
            gx += x * a;
            gy += y * a;
            area += a;
        }
        while cr > r {
            cr -= 1;
            // del cr
            let (x, y, a) = get(cl, cr, cr + 1);
            gx -= x * a;
            gy -= y * a;
            area -= a;
        }
        while cl < l {
            // del cl
            let (x, y, a) = get(cl, cl + 1, cr);
            gx -= x * a;
            gy -= y * a;
            area -= a;
            cl += 1;
        }
        ans[idx] = (gx as f64 / area as f64 / 3.0, gy as f64 / area as f64 / 3.0);
    }
    for (x, y) in ans {
        println!("{x} {y}");
    }
}
