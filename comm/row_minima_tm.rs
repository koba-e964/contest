// Ported and modified from https://topcoder-g-hatena-ne-jp.jag-icpc.org/spaghetti_source/20120923/1348327542.html
// Verified by: ABC289-G (https://atcoder.jp/contests/abc289/submissions/38855915)
type Val = i64;
const INF: Val = 1 << 62;

fn smawk_min<F: Fn(usize, usize) -> Val>(js: Vec<usize>, ib: usize, ie: usize, lv: usize, f: &F, minima: &mut [usize]) {
    if ib >= ie { return; }

    let id = 1 << lv;
    let len = (ie - 1 - ib) >> lv;
    let mut js2 = vec![];
    let mut i = ib;
    for &jsq in &js {
        while !js2.is_empty() && f(i, js2[js2.len() - 1]) >= f(i, jsq) {
            js2.pop(); i -= id;
        }
        if js2.len() != len {
            js2.push(jsq); i += id;
        }
    }
  
    smawk_min(js2, ib + id, ie, lv + 1, f, minima);

    let mut i = ib;
    let mut q = 0;
    while i < ie {
        let jt = if i + id < ie {
            minima[i + id]
        } else {
            js[js.len() - 1]
        };
        let mut fm = INF;
        while q < js.len() {
            let fq = f(i, js[q]);
            if fm > fq {
                fm = fq;
                minima[i] = js[q];
            }
            if js[q] == jt { break; }
            q += 1;
        }
        i += id * 2;
    }
}

// f: totally monotone
fn row_minima_tm<F: Fn(usize, usize) -> i64>(
    rows: std::ops::Range<usize>,
    cols: std::ops::Range<usize>,
    f: F, minima: &mut [usize],
) {
    smawk_min(cols.collect(), rows.start, rows.end, 0, &f, minima);
}
