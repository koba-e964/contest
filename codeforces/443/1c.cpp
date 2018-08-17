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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int par;

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

  int size(void) { return root->size; }
  static bool gt_min(const T &key, const T &tkey) {
    REP(i, 0, par) {
      if (key[i] > tkey[i]) return true;
    }
    return false;
  }
  static bool lt_max(const T &key, const T &tkey) {
    REP(i, par, 2 * par) {
      if (key[i] < tkey[i]) return true;
    }
    return false;
  }
  pointer find(const T &key) { return find(root, key); }
  node *find(node *t, const T &key) {
    if (t == NULL) return NULL;
    if (key == t->key) return t;
    else if (key < t->key) return find(t->child[0], key);
    else                   return find(t->child[1], key);
  }
  pointer find_next(const T &key) { return find_next(root, key); }
  node *find_next(node *t, const T &key) {
    if (t == NULL) return NULL;
    if (key == t->key) return t;
    else if (key < t->key) {
      node *tmp = find_next(t->child[0], key);
      return tmp ? tmp : t;
    }
    else                   return find_next(t->child[1], key);
  }
  // the strongest that key can defeat (win)
  pointer find_max(const T &key) { return find_max(root, key); }
  node *find_max(node *t, const T &key) {
    if (t == NULL) return NULL;
    if (key == t->key) assert (0);
    else if (gt_min(key, t->key)) {
      node *tmp = find_max(t->child[1], key);
      return tmp ? tmp : t;
    }
    else                   return find_max(t->child[0], key);
  }
  pointer find_min(const T &key) { return find_min(root, key); }
  node *find_min(node *t, const T &key) {
    if (t == NULL) return NULL;
    if (key == t->key) assert (0);
    else if (lt_max(key, t->key)) {
      node *tmp = find_min(t->child[0], key);
      return tmp ? tmp : t;
    }
    else                   return find_min(t->child[1], key);
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
};

void debug_entry(VI t) {
  REP(i, 0, 2 *par) cerr << t[i] << " ";
  cerr << "[" << t[2 * par] << "]" << endl;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  par = k;
  vector<VL> s(n, VL(k));
  REP(i, 0, n) REP(j, 0, k) cin >> s[i][j];
  // [0, k): min, [k, 2 * k): max, [2 * k]: size
  avl_tree<VI> bin;
  REP(i, 0, n) {
    if (DEBUG) {
      DEBUGP(i);
    }
    VI dat(2 * k + 1);
    REP(j, 0, k) {
      dat[j] = dat[j + k] = s[i][j];
    }
    dat[2 * k] = 1;
    avl_tree<VI>::node *lo = bin.find_min(dat);
    avl_tree<VI>::node *hi = bin.find_max(dat);
    bool single = false;
    VI xlo, xhi;
    if (lo && hi) {
      xlo = lo->key;
      xhi = hi->key;
      if (DEBUG) {
	cerr << "xlo = "; debug_entry(xlo);
	cerr << "xhi = "; debug_entry(xhi);
      }
      if (xlo[0] > xhi[0]) {
	single = true;
      }
    } else {
      single = true;
    }
    if (not single) {
      assert (lo && hi);
      while (1) {
	if (DEBUG) {
	  cerr << "dat = "; debug_entry(dat);
	  cerr << "xlo = "; debug_entry(xlo);
	}
	REP(i, 0, k) dat[i] = min(dat[i], xlo[i]);
	REP(i, k, 2 * k) dat[i] = max(dat[i], xlo[i]);
	dat[2 * k] += xlo[2 * k];
	if (DEBUG) {
	  cerr << "newdat = "; debug_entry(dat);
	}
	bin.erase(xlo);
	if (xlo == xhi) break;
	lo = bin.find_next(xlo);
	xlo = lo->key;
      }
    }
    if (DEBUG) {
      cerr << "final_dat = "; debug_entry(dat);
    }
    bin.insert(dat);
    avl_tree<VI>::node *ma_ent = bin.find_max(VI(2 * k + 1, (1 << 31) - 1));
    VI ma = ma_ent->key;
    if (DEBUG) {
      cerr << "ma = "; debug_entry(ma);
      cerr << "mi = "; debug_entry(bin.find_min(VI(2 * k + 1, -1))->key);
    }
    cout << ma[2 * k] << "\n";
  }
}
