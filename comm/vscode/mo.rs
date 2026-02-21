const B: usize = 500;
let mut lri: Vec<_> = (0..q).map(|i| {
    let (l, r) = lr[i];
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
let mut ans = vec![0; q];

// pointer
let mut cl = 0;
let mut cr = 0;

// state

for &(l, r, idx) in &lri {
    while cr < r {
        // add cr
        cr += 1;
    }
    while cl > l {
        cl -= 1;
        // add cl
    }
    while cr > r {
        cr -= 1;
        // del cr
    }
    while cl < l {
        // del cl
        cl += 1;
    }
}
