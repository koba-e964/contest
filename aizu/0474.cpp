#include <algorithm>
#include <iostream>
#include <cassert>
#include <functional>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

// https://github.com/atcoder/ac-library/blob/v1.6/atcoder/internal_bit.hpp
unsigned int bit_ceil(unsigned int n) {
    unsigned int x = 1;
    while (x < (unsigned int)(n)) x *= 2;
    return x;
}

int countr_zero(unsigned int n) {
#ifdef _MSC_VER
    unsigned long index;
    _BitScanForward(&index, n);
    return index;
#else
    return __builtin_ctz(n);
#endif
}

// https://github.com/atcoder/ac-library/blob/v1.6/atcoder/lazysegtree.hpp
template <class S,
          S (*op)(S, S),
          S (*e)(),
          class F,
          S (*mapping)(F, S),
          F (*composition)(F, F),
          F (*id)()>
struct lazy_segtree {
  public:
    lazy_segtree() : lazy_segtree(0) {}
    explicit lazy_segtree(int n) : lazy_segtree(std::vector<S>(n, e())) {}
    explicit lazy_segtree(const std::vector<S>& v) : _n(int(v.size())) {
        size = (int)bit_ceil((unsigned int)(_n));
        log = countr_zero((unsigned int)size);
        d = std::vector<S>(2 * size, e());
        lz = std::vector<F>(size, id());
        for (int i = 0; i < _n; i++) d[size + i] = v[i];
        for (int i = size - 1; i >= 1; i--) {
            update(i);
        }
    }

    void set(int p, S x) {
        assert(0 <= p && p < _n);
        p += size;
        for (int i = log; i >= 1; i--) push(p >> i);
        d[p] = x;
        for (int i = 1; i <= log; i++) update(p >> i);
    }

    S get(int p) {
        assert(0 <= p && p < _n);
        p += size;
        for (int i = log; i >= 1; i--) push(p >> i);
        return d[p];
    }

    S prod(int l, int r) {
        assert(0 <= l && l <= r && r <= _n);
        if (l == r) return e();

        l += size;
        r += size;

        for (int i = log; i >= 1; i--) {
            if (((l >> i) << i) != l) push(l >> i);
            if (((r >> i) << i) != r) push((r - 1) >> i);
        }

        S sml = e(), smr = e();
        while (l < r) {
            if (l & 1) sml = op(sml, d[l++]);
            if (r & 1) smr = op(d[--r], smr);
            l >>= 1;
            r >>= 1;
        }

        return op(sml, smr);
    }

    S all_prod() { return d[1]; }

    void apply(int p, F f) {
        assert(0 <= p && p < _n);
        p += size;
        for (int i = log; i >= 1; i--) push(p >> i);
        d[p] = mapping(f, d[p]);
        for (int i = 1; i <= log; i++) update(p >> i);
    }
    void apply(int l, int r, F f) {
        assert(0 <= l && l <= r && r <= _n);
        if (l == r) return;

        l += size;
        r += size;

        for (int i = log; i >= 1; i--) {
            if (((l >> i) << i) != l) push(l >> i);
            if (((r >> i) << i) != r) push((r - 1) >> i);
        }

        {
            int l2 = l, r2 = r;
            while (l < r) {
                if (l & 1) all_apply(l++, f);
                if (r & 1) all_apply(--r, f);
                l >>= 1;
                r >>= 1;
            }
            l = l2;
            r = r2;
        }

        for (int i = 1; i <= log; i++) {
            if (((l >> i) << i) != l) update(l >> i);
            if (((r >> i) << i) != r) update((r - 1) >> i);
        }
    }

    template <bool (*g)(S)> int max_right(int l) {
        return max_right(l, [](S x) { return g(x); });
    }
    template <class G> int max_right(int l, G g) {
        assert(0 <= l && l <= _n);
        assert(g(e()));
        if (l == _n) return _n;
        l += size;
        for (int i = log; i >= 1; i--) push(l >> i);
        S sm = e();
        do {
            while (l % 2 == 0) l >>= 1;
            if (!g(op(sm, d[l]))) {
                while (l < size) {
                    push(l);
                    l = (2 * l);
                    if (g(op(sm, d[l]))) {
                        sm = op(sm, d[l]);
                        l++;
                    }
                }
                return l - size;
            }
            sm = op(sm, d[l]);
            l++;
        } while ((l & -l) != l);
        return _n;
    }

    template <bool (*g)(S)> int min_left(int r) {
        return min_left(r, [](S x) { return g(x); });
    }
    template <class G> int min_left(int r, G g) {
        assert(0 <= r && r <= _n);
        assert(g(e()));
        if (r == 0) return 0;
        r += size;
        for (int i = log; i >= 1; i--) push((r - 1) >> i);
        S sm = e();
        do {
            r--;
            while (r > 1 && (r % 2)) r >>= 1;
            if (!g(op(d[r], sm))) {
                while (r < size) {
                    push(r);
                    r = (2 * r + 1);
                    if (g(op(d[r], sm))) {
                        sm = op(d[r], sm);
                        r--;
                    }
                }
                return r + 1 - size;
            }
            sm = op(d[r], sm);
        } while ((r & -r) != r);
        return 0;
    }

  private:
    int _n, size, log;
    std::vector<S> d;
    std::vector<F> lz;

    void update(int k) { d[k] = op(d[2 * k], d[2 * k + 1]); }
    void all_apply(int k, F f) {
        d[k] = mapping(f, d[k]);
        if (k < size) lz[k] = composition(f, lz[k]);
    }
    void push(int k) {
        all_apply(2 * k, lz[k]);
        all_apply(2 * k + 1, lz[k]);
        lz[k] = id();
    }
};

// add-sum
struct S1 {
    int a;
    int size;
};

struct F1 {
    int a, b;
};

S1 op1(S1 l, S1 r) { return S1{l.a + r.a, l.size + r.size}; }

S1 e1() { return S1{0, 0}; }

S1 mapping1(F1 l, S1 r) { return S1{r.a * l.a + r.size * l.b, r.size}; }

F1 composition1(F1 l, F1 r) { return F1{r.a * l.a, r.b * l.a + l.b}; }

F1 id1() { return F1{1, 0}; }

// add-max
struct S2 {
    int a;
};

struct F2 {
    int coef; // 0 or 1
    int a;
};

S2 op2(S2 l, S2 r) { return S2{max(l.a, r.a)}; }

S2 e2() { return S2{-1}; }

S2 mapping2(F2 l, S2 r) { return S2{l.a + l.coef * r.a}; }

F2 composition2(F2 l, F2 r) { return F2{l.coef * r.coef, l.a + l.coef * r.a}; }

F2 id2() { return F2{1, 0}; }

int main() {
    int n, m;
    cin >> n >> m;
    vector<int> x(m);
    REP(i, 0, m) cin >> x[i];
    sort(x.begin(), x.end());
    vector<S2> t(m);
    REP(i, 0, m) t[i] = S2{x[i]};
    vector<S1> up(m);
    REP(i, 0, m) up[i] = S1{1, 1};
    lazy_segtree<S1, op1, e1, F1, mapping1, composition1, id1> seg1(up);
    lazy_segtree<S2, op2, e2, F2, mapping2, composition2, id2> seg2(t);
    int q;
    cin >> q;
    REP(_, 0, q) {
        int ty, a, b;
        cin >> ty >> a >> b;
        b++;
        if (ty == 1) {
            int c;
            cin >> c;
            if (c > 0) {
                int idx0 = seg2.max_right(0, [&](S2 val) { return val.a < a; });
                int idx1 = seg2.max_right(0, [&](S2 val) { return val.a < max(0, b - c); });
                idx1 = max(idx1, idx0);
                int idx2 = seg2.max_right(0, [&](S2 val) { return val.a < b; });
                seg2.apply(idx0, idx1, F2{1, c});
                seg2.apply(idx1, idx2, F2{0, b - 1});
                seg1.apply(idx0, idx2, F1{-1, 0});
            } else {
                int idx0 = seg2.max_right(0, [&](S2 val) { return val.a < a; });
                int idx1 = seg2.max_right(0, [&](S2 val) { return val.a < a - c; });
                int idx2 = seg2.max_right(0, [&](S2 val) { return val.a < b; });
                idx1 = min(idx1, idx2);
                seg2.apply(idx1, idx2, F2{1, c});
                seg2.apply(idx0, idx1, F2{0, a});
                seg1.apply(idx0, idx2, F1{-1, 0});
            }
        } else {
            int idx0 = seg2.max_right(0, [&](S2 val) { return val.a < a; });
            int idx1 = seg2.max_right(0, [&](S2 val) { return val.a < b; });
            auto val = seg1.prod(idx0, idx1);
            cout << val.a << "\n";
        }
    }
}
