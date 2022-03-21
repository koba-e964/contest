#include <algorithm>
#include <cassert>
#include <iostream>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

// Copied and modified from: https://atcoder.jp/contests/geocon2013/submissions/20558265
struct P {
  ll x, y;
  P(){};
  P(ll a, ll b) {
    x = a;
    y = b;
  }
  P operator-(P a) { return P(x - a.x, y - a.y); }
  P operator+(P a) { return P(x + a.x, y + a.y); }
  bool operator<(P b) const {
    if (x == b.x) return this->y < b.y;
    return x < b.x;
  }
  bool operator==(P b) const {
    return x == b.x && y == b.y;
  }
};

ll dot(P a, P b) { return a.x * b.x + a.y * b.y; }
ll det(P a, P b) { return a.x * b.y - b.x * a.y; }
 
int ccw(P a, P b, P c) {
  c = c - a;
  b = b - a;
  if (det(b, c) > 0) return 1;
  if (det(b, c) < 0) return -1;
  return 0;
}

const P inf(0, -1e11);

struct pairlike {
  P x;
  P y;
  pairlike(P x, P y): x(x), y(y) {}
  bool operator<(const pairlike &other) const {
    if (!(x == other.x)) return x < other.x;
    return y < other.y;
  }
  bool operator<(const P &dir) const {
    if (y == inf) {
      return false;
    }
    ll dif = dot(x, dir) - dot(y, dir);
    return dif < 0;
  }
};
pairlike mk_pair(P x, P y) {
  return pairlike(x, y);
}

struct Upper {
  set<P> upper;
  set<pairlike, less<>> nxt;
  Upper(){}
  set<P>::iterator erase_it(set<P>::iterator it) {
    auto it2 = nxt.lower_bound(mk_pair(*it, P(-1e15, -1e15)));
    assert (it2 != nxt.end());
    P to = it2->y;
    it2 = nxt.erase(it2);
    if (it2 != nxt.begin()) {
      it2--;
      P from = it2->x;
      nxt.erase(it2);
      nxt.insert(mk_pair(from, to));
    }
    it = upper.erase(it);
    return it;
  }
  void insert(P p) {
    auto it = upper.lower_bound(p);
    if (it != upper.end() && *it == p) return;
    P to = it != upper.end() ? *it : inf;
    nxt.insert(mk_pair(p, to));
    if (it != upper.begin()) {
      int res = nxt.erase(mk_pair(*prev(it), to));
      if (res != 1) {
        cerr << "res = " << res << endl;
        cerr << "p = " << p.x << "," << p.y << endl;
        cerr << "to = " << to.x << " " << to.y << endl;
        debug();
      }
      assert (res == 1);
      nxt.insert(mk_pair(*prev(it), p));
    }
    upper.insert(p);
  }
  void add_upper(P p) {
    if (upper.size() == 0) {
      insert(p);
      return;
    }
    if (upper.size() == 1) {
      insert(p);
      return;
    }
    auto it = upper.lower_bound(p);
    if (it != upper.begin() && it != upper.end() && ccw(*prev(it), *it, p) <= 0)
      return;
    // left
    if (it != upper.begin()) it = prev(it);
    while (it != upper.begin() && ccw(*it, *prev(it), p) <= 0) {
      it = erase_it(it);
      it--;
    }
    // right
    it = upper.lower_bound(p);
    if (it != upper.end()) {
      while (next(it) != upper.end() && ccw(*it, *next(it), p) >= 0) {
        it = erase_it(it);
      }
    }
    insert(p);
  }
  P find_dir(P dir) const {
    assert (nxt.size());
    auto it = nxt.lower_bound(dir);
    if (it != nxt.end()) return it->x;
    it--;
    return it->x;
  }
  void debug() const {
    cerr << "upper:";
    for (auto x: upper) cerr << x.x << "," << x.y << " ";
    cerr << endl;
    cerr << "nxt:";
    for (auto x: nxt) cerr << x.x.x << "," << x.x.y << "==="
                           << x.y.x << "," << x.y.y << " ";
    cerr << endl;
  }
};
 
class AddableConvexHull {
 public:
  Upper upper, lower;
  AddableConvexHull() {}
 
  void add_upper(P p) {
    upper.add_upper(p);
  }
 
  void add_lower(P p) {
    lower.add_upper(P(-p.x, -p.y));
  }
 
  void add(P p) {
    add_upper(p);
    add_lower(p);
  }

  ll find_dir(P dir) const {
    if (dir.y == 0) {
      if (dir.x == 0) return 0;
      if (dir.x > 0) {
        P p = *upper.upper.rbegin();
        return p.x * dir.x;
      }
      P p = *upper.upper.begin();
      return p.x * dir.x;
    }
    if (dir.y > 0) {
      P u = upper.find_dir(dir);
      return dot(u, dir);
    }
    P v = lower.find_dir(P(-dir.x, -dir.y));
    return -dot(v, dir);
  }

  void debug() const {
    upper.debug();
    lower.debug();
  }
};

// End https://atcoder.jp/contests/geocon2013/submissions/20558265

// Tags: geometry, dynamic-convex-hull
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q; cin >> q;
  VI x(q), y(q), a(q), b(q);
  REP(i, 0, q) {
    cin >> x[i] >> y[i] >> a[i] >> b[i];
  }
  AddableConvexHull c;
  REP(i, 0, q) {
    c.add(P(x[i], y[i]));
    cout << c.find_dir(P(a[i], b[i])) << "\n";
  }
}
