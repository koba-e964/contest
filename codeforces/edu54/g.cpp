#include <algorithm>
#include <cassert>
#include <cstdio>
#include <vector>


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;

template<class T, class U>
struct LazySegTree {
  int n;
  vector<T> dat;
  vector<U> lazy;
  T e;
  U upe; // upe: identity for upop
  static void biop_assign(T &u, const T &a, const T &b) {
    int m = a[0].size();
    REP(i, 0, 2) {
      REP(j, 0, m) {
        u[i][j] = a[i][b[i][j]];
      }
    }
  }
  static void upop(T &a, U b) {
    if (b == 1) {
      swap(a[0], a[1]);
    }
  }
  static U upbiop(U a, U b) {
    return a ^ b;
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
    assert (0 <= k && k < n - 1);
    biop_assign(dat[k], dat[2 * k + 1], dat[2 * k + 2]);
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
  int query_sub(int a, int b, int k, int l, int r, int x) {
    lazy_evaluate_node(k);
    if (r <= a || b <= l) return x;
    if (a <= l && r <= b) return dat[k][0][x];
    int mid = (l + r) / 2;
    int vr = query_sub(a, b, 2 * k + 2, mid, r, x);
    int vl = query_sub(a, b, 2 * k + 1, l, mid, vr);
    update_node(k);
    return vl;
  }
  // [a, b)
  int query(int a, int b, int x) {
    return query_sub(a, b, 0, 0, n, x);
  }
};



int main(void) {
  int n, m, q;
  scanf("%d%d%d", &n, &m, &q);
  VL a(n);
  REP(i, 0, n) {
    scanf("%lld", &a[i]);
    a[i] %= 2;
  }
  typedef vector<vector<int> > T;
  T id(2, VI(m + 1, 0));
  REP(i, 0, m + 1) {
    id[0][i] = id[1][i] = i;
  }
  LazySegTree<T, int> lst(n, id, 0);
  REP(i, 0, n) {
    T val(2, VI(m + 1, 0));
    REP(j, 0, m) {
      val[0][j] = val[1][j] = j + 1;
    }
    val[0][m] = m;
    val[1][m] = 0;
    if (a[i] == 1) {
      swap(val[0], val[1]);
    }
    lst.dat[lst.n - 1 + i] = val;
  }
  for (int i = lst.n - 2; i >= 0; --i) {
    lst.update_node(i);
  }
  REP(i, 0, q) {
    int ty;
    scanf("%d", &ty);
    if (ty == 1) {
      int l, r;
      ll d;
      scanf("%d%d%lld", &l ,&r, &d);
      l--;
      d %= 2;
      lst.update(l, r, d);
    } else {
      int l, r;
      scanf("%d%d", &l, &r);
      l--;
      int res = lst.query(l, r, m);
      printf("%d\n", res ? 1 : 2);
    }
  }
}
