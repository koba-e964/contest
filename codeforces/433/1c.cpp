#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

// http://www.prefield.com/algorithm/container/avl_tree.html
template <class T>
struct avl_tree {
  struct node {
    T key;
    int size, height;
    node *child[2];
    node(const T &key) : key(key), size(1), height(1) {
      child[0] = child[1] = 0; }
  } *root;
  typedef node *pointer;
  avl_tree() { root = NULL; }
  int size() const { return root == NULL ? 0 : root->size; }

  pointer find(const T &key) { return find(root, key); }
  node *find(node *t, const T &key) {
    if (t == NULL) return NULL;
    if (key == t->key) return t;
    else if (key < t->key) return find(t->child[0], key);
    else                   return find(t->child[1], key);
  }
  void insert(const T &key) { root = insert(root, new node(key)); }
  node *insert(node *t, node *x) {
    if (t == NULL) return x;
    if (x->key <= t->key) t->child[0] = insert(t->child[0], x);
    else                  t->child[1] = insert(t->child[1], x);
    t->size += 1;
    return balance(t);
  }
  void erase(const T &key) { root = erase(root, key); }
  node *erase(node *t, const T &x) {
    if (t == NULL) return NULL;
    if (x == t->key) {
      return move_down(t->child[0], t->child[1]);
    } else {
      if (x < t->key) t->child[0] = erase(t->child[0], x);
      else            t->child[1] = erase(t->child[1], x);
      t->size -= 1;
      return balance(t);
    }
  }
  node *move_down(node *t, node *rhs) {
    if (t == NULL) return rhs;
    t->child[1] = move_down(t->child[1], rhs);
    return balance(t);
  }
#define sz(t) (t ? t->size : 0)
#define ht(t) (t ? t->height : 0)
  node *rotate(node *t, int l, int r) {
    node *s = t->child[r];
    t->child[r] = s->child[l];
    s->child[l] = balance(t);
    if (t) t->size = sz(t->child[0]) + sz(t->child[1]) + 1;
    if (s) s->size = sz(s->child[0]) + sz(s->child[1]) + 1;
    return balance(s);
  }
  node *balance(node *t) {
    for (int i = 0; i < 2; ++i) {
      if (ht(t->child[!i]) - ht(t->child[i]) < -1) {
        if (ht(t->child[i]->child[!i]) - ht(t->child[i]->child[i]) > 0)
          t->child[i] = rotate(t->child[i], i, !i);
        return rotate(t, !i, i);
      }
    }
    if (t) t->height = max(ht(t->child[0]), ht(t->child[1])) + 1;
    if (t) t->size = sz(t->child[0]) + sz(t->child[1]) + 1;
    return t;
  }
  pointer rank(int k) const { return rank(root, k); }
  pointer rank(node *t, int k) const {
    if (!t) return NULL;
    int m = sz(t->child[0]);
    if (k  < m) return rank(t->child[0], k);
    if (k == m) return t;
    if (k  > m) return rank(t->child[1], k - m - 1);
  }
  int lower_count(T key) const { return lower_count(root, key); }
  int lower_count(node *t, T x) const {
    if (t == NULL) return 0;
    if (x == t->key) {
      return sz(t->child[0]);
    }
    if (x < t->key) {
      return lower_count(t->child[0], x);
    }
    int tmp = sz(t->child[0]) + 1;
    return tmp + lower_count(t->child[1], x);
  }
};

const int N = 1 << 18;
avl_tree<int> seg[N];


void update(int k, int v) {
  k += 1;
  while (k < N) {
    seg[k].insert(v);
    k += k & -k;
  }
}

int get_once(int idx, int d, int u) {
  if (d <= 0 && u >= N) {
    return seg[idx].size();
  }
  return seg[idx].lower_count(u) - seg[idx].lower_count(d);
}

int get(int r, int d, int u) {
  int tot = 0;
  while (r > 0) {
    tot += get_once(r, d, u);
    r &= r - 1;
  }
  return tot;
}

int get(int l, int r, int d, int u) {
  return get(r, d, u) - get(l, d, u);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  VI p(n);
  REP(i, 0, n) {
    cin >> p[i];
    p[i]--;
    update(i, p[i]);
  }
  REP(i, 0, q) {
    int l, d, r, u;
    cin >> l >> d >> r >> u;
    l--, d--;
    ll tot = 0;
    ll zl0d = get(0, l, 0, d);
    ll tmp = zl0d;
    ll lndn = get(l, n, d, n);
    tmp *= lndn;
    tot += tmp;
    ll zldu = get(0, l, d, u);
    tmp = zldu;
    ll ln0n = get(l, n, 0, N);
    tmp *= ln0n;
    tot += tmp;
    ll ln0u = get(l, n, 0, u);
    tmp = get(0, l, 0, N) - zl0d - zldu;
    tmp *= ln0u;
    tot += tmp;
    ll lr0d = get(l, r, 0, d);
    tmp = lr0d;
    tmp *= lndn;
    tot += tmp;
    ll ln0d = ln0n - lndn;
    ll rn0d = ln0d - lr0d;
    ll rndu = get(r, n, d, u);
    ll lnun = ln0n - ln0u;
    ll lrun = get(l, r, u, n);
    ll rnun = lnun - lrun;
    ll lndu = ln0u + lndn - ln0n;
    ll lrdu = lndu - rndu;
    tmp = lrdu;
    tmp *= (rn0d + rndu + rnun + lrun); // get(r, n, 0, n) + get(l, r, u, n));
    tot += tmp;
    tmp = lrun;
    tmp *= rn0d + rndu;
    tot += tmp;
    tmp = lrdu;
    tot += tmp * (tmp - 1) / 2;
    if (0) {
      cerr << get(0, l, 0, d) << " " << lr0d << " " << rn0d << endl;
      cerr << get(0, l, d, u) << " " << lrdu << " " << rndu << endl;
      cerr << get(0, l, u, n) << " " << lrun << " " << rnun << endl;
    }
    cout << tot << "\n";
  }
}
