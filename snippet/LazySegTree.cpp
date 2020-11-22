// Not verified
template<class T, class U>
struct LazySegTree {
  int n;
  vector<T> dat;
  vector<U> lazy;
  T e;
  U upe; // upe: identity for upop
  static T biop(const T &a, const T &b) {
    // TODO modify here
    return min(a, b);
  }
  static void upop(T &a, const U &b) {
    // TODO modify here
    a += b;
  }
  static U upbiop(const U &a, const U &b) {
    // TODO modify here
    return a + b;
  }
  LazySegTree(int n_, T e, U upe): e(e), upe(upe) {
    n = 1;
    while (n < n_) n *= 2;
    dat = vector<T>(2 * n - 1, e);
    lazy = vector<U>(2 * n - 1, upe);
  }
  void lazy_evaluate_node(int k) {
    upop(dat[k], lazy[k]);
    if (k < n - 1) {
      lazy[2 * k + 1] = upbiop(lazy[2 * k + 1], lazy[k]);
      lazy[2 * k + 2] = upbiop(lazy[2 * k + 2], lazy[k]);
    }
    lazy[k] = upe;
  }
  void update_node(int k) {
    dat[k] = biop(dat[2 * k + 1], dat[2 * k + 2]);
  }
  void update_sub(int a, int b, U v, int k, int l, int r) {
    lazy_evaluate_node(k);
    if (r <= a || b <= l) return;
    if (a <= l && r <= b) {
      lazy[k] = upbiop(lazy[k], v);
      lazy_evaluate_node(k);
      return;
    }
    int mid = (l + r) / 2;
    update_sub(a, b, v, 2 * k + 1, l, mid);
    update_sub(a, b, v, 2 * k + 2, mid, r);
    update_node(k);
  }
  // Updates [a, b)
  void update(int a, int b, U v) {
    update_sub(a, b, v, 0, 0, n);
  }
  void update_single(int a, T val, int k, int l, int r) {
    lazy_evaluate_node(k);
    if (r <= a || a + 1 <= l) return;
    if (a <= l && r <= a + 1) {
      lazy_evaluate_node(k);
      dat[k] = val;
      return;
    }
    int mid = (l + r) / 2;
    update_single(a, val, 2 * k + 1, l, mid);
    update_single(a, val, 2 * k + 2, mid, r);
    update_node(k);
  }
  void update_single(int a, T val) {
    update_single(a, val, 0, 0, n);
  }
  T query_sub(int a, int b, int k, int l, int r) {
    lazy_evaluate_node(k);
    if (r <= a || b <= l) return e;
    if (a <= l && r <= b) return dat[k];
    int mid = (l + r) / 2;
    T vl = query_sub(a, b, 2 * k + 1, l, mid);
    T vr = query_sub(a, b, 2 * k+ 2, mid, r);
    update_node(k);
    biop(vl, vr);
  }
  // [a, b)
  T query(int a, int b) {
    return query_sub(a, b, 0, 0, n);
  }
};

