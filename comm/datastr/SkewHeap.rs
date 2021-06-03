// skew heap: http://hos.ac/blog/#blog0001
// Verified by: https://atcoder.jp/contests/atc002/submissions/23142200
#[derive(Clone)]
struct SkewHeap<T> {
    l: Option<Box<Self>>,
    r: Option<Box<Self>>,
    val: T,
}

impl<T: PartialOrd> SkewHeap<T> {
    fn new() -> Option<Box<Self>> {
        None
    }
    fn with_one(a: T) -> Option<Box<Self>> {
        Some(Box::new(SkewHeap {
            l: None,
            r: None,
            val: a,
        }))
    }
    fn peek(a: &Option<Box<Self>>) -> Option<&T> {
        if let Some(a) = a.as_ref() {
            Some(&a.val)
        } else {
            None
        }
    }
    // Complexity: amortized O(log (|a| + |b|))
    fn meld(a: Option<Box<Self>>, b: Option<Box<Self>>) -> Option<Box<Self>> {
        let (mut a, mut b) = match (a, b) {
            (None, b) => return b,
            (a, None) => return a,
            (Some(a), Some(b)) => (a, b)
        };
        if a.val > b.val {
            std::mem::swap(&mut a, &mut b);
        }
        a.r = Self::meld(a.r, Some(b));
        std::mem::swap(&mut a.l, &mut a.r);
        Some(a)
    }
    fn pop(h: &mut Option<Box<Self>>) -> Option<T> {
        if let Some(inner) = std::mem::replace(h, None) {
            let new = Self::meld(inner.l, inner.r);
            let ret = Some(inner.val);
            *h = new;
            return ret;
        }
        None
    }
    fn push(h: &mut Option<Box<Self>>, t: T) {
        let sing = Self::with_one(t);
        *h = Self::meld(std::mem::replace(h, None), sing);
    }
}
