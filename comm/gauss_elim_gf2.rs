/*
 * Find an assignment (result) s.t. xor_i a[i] * result[i] = b (in GF(2))
 * Returns Some if such an assignment was found, None otherwise.
 */
fn gauss_elim_gf2_i64(basis: &[i64], mut b: i64) -> Option<Vec<bool>> {
    let n = basis.len();
    let mut a = basis.to_vec();
    let mut c = 0;
    let mut revmap = Vec::new();
    let w = 64; // i64's size
    for r in 0 .. w {
        if c >= n {
            break;
        }
        let mut c2 = None;
        for i in c .. n {
            if (a[i] & 1 << r) != 0 {
                c2 = Some(i);
                break;
            }
        }
        if c2 == None {
            revmap.push(None);
            continue;
        }
        a.swap(c, c2.unwrap());
        let rm = a[c] & -(1 << r) << 1;
        a[c] ^= rm;
        for k in c + 1 .. n {
            if (a[k] & 1 << r) != 0 {
                a[k] ^= rm;
            }
        }
        if (b & 1 << r) != 0 {
            b ^= rm;
        }
        revmap.push(Some(c));
        c += 1;
    }
    // recover
    let rank = revmap.len();
    let mut result = vec![false; n];
    for i in (0 .. rank).rev() {
        if (b & 1 << i) != 0 {
            match revmap[i] {
                None => return None,
                Some(c) => {
                    b ^= a[c];
                    result[c] = true;
                },
            }
        }
    }
    if b == 0 {
        Some(result)
    } else {
        None
    }
}
