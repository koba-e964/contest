#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
private:
  std::vector<int> disj;
  std::vector<int> rank;
public:
  UnionFind(int n) : disj(n), rank(n) {
    for (int i = 0; i < n; ++i) {
      disj[i] = i;
      rank[i] = 0;
    }
  }
  int root(int x) {
    if (disj[x] == x) {
      return x;
    }
    return disj[x] = root(disj[x]);
  }
  void unite(int x, int y) {
    x = root(x);
    y = root(y);
    if (x == y) {
      return;
    }
    if (rank[x] < rank[y]) {
      disj[x] = y;
    } else {
      disj[y] = x;
      if (rank[x] == rank[y]) {
	++rank[x];
      }
    }
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};

typedef pair<ll, ll> PL;
typedef pair<ll, int> PLI;

const int DEBUG = 0;

ll solve(UnionFind &uf, int n, const vector<PL> &pts, ll d, int mode, int root = -1) {
  ll tot = 0;
  map<ll, vector<PLI> > tt;
  REP(i, 0, n) {
    ll x = pts[i].first;
    if (tt.count(x) == 0) {
      tt[x] = vector<PLI>();
    }
    tt[x].push_back(PLI(pts[i].second, i));
  }
  int m = tt.size();
  vector<pair<ll, vector<PLI> > > tt_v;
  tt_v.reserve(m);
  for (map<ll, vector<PLI> >::iterator it = tt.begin(); it != tt.end(); ++it) {
    tt_v.push_back(*it);
  }
  assert (tt_v.size() == m);
  if (DEBUG) {
    REP(i, 0, m) {
      cerr << "tt_v[" << i << "]" << "(" << tt_v[i].first << "):";
      REP(j, 0, tt_v[i].second.size()) {
	cerr << " " << tt_v[i].second[j].first
	     << "(" << tt_v[i].second[j].second << ")";
      }
      cerr << endl;
    }
  }
  REP(i, 0, m) {
    vector<PLI> irnbru = tt_v[i].second;
    sort(irnbru.begin(), irnbru.end());
    ll dest = tt_v[i].first + d;
    if (tt.count(dest) == 0) {
      continue;
    }
    vector<PLI> udon = tt[dest];
    sort(udon.begin(), udon.end());
    int merged = -1;
    REP(j, 0, irnbru.size()) {
      ll cur = irnbru[j].first; // y-coord
      int pos1 = lower_bound(udon.begin(), udon.end(), PLI(cur - d, -1))
	- udon.begin();
      int pos2 = lower_bound(udon.begin(), udon.end(), PLI(cur + d + 1, -1))
	- udon.begin();
      if (mode == 0) {
	if (pos1 < pos2) {
	  uf.unite(irnbru[j].second, udon[pos1].second);
	  REP(k, max(pos1, merged), pos2 - 1) {
	    uf.unite(udon[k].second, udon[k + 1].second);
	  }
	  merged = pos2 - 1;
	}
      }
      if (mode == 1 && uf.root(irnbru[j].second) == root) {
	tot += (pos2 - pos1) * 2;
	if (pos1 < pos2) {
	  if (udon[pos1].first == cur - d) {
	    tot--;
	  }
	  if (udon[pos2 - 1].first == cur + d) {
	    tot--;
	  }
	}
	if (DEBUG) { cerr << "tot = " << tot << endl; }
      }
      if (DEBUG) { cerr << "cur = " << cur << ", pos1=" << pos1 << ", pos2=" << pos2 << endl; }
    }
  }
  return tot;
}

int main(void){
  int n, a, b;
  cin >> n >> a >> b;
  a--, b--;
  VL x(n), y(n);
  REP(i, 0, n) {
    ll u, v;
    cin >> u >> v;
    x[i] = u + v;
    y[i] = u - v;
  }
  ll dist = max(max(x[a] - x[b], x[b] - x[a]), max(y[a] - y[b], y[b] - y[a]));
  UnionFind uf(n);
  vector<PL> pts(n);
  REP(i, 0, n) {
    pts[i] = PL(x[i], y[i]);
  }
  solve(uf, n, pts, dist, 0);
  REP(i, 0, n) {
    pts[i] = PL(y[i], x[i]);
  }
  solve(uf, n, pts, dist, 0);
  REP(i, 0, n) {
    pts[i] = PL(x[i], y[i]);
  }
  ll tot = 0;
  assert (uf.root(a) == uf.root(b));
  int r = uf.root(a);
  tot += solve(uf, n, pts, dist, 1, r);
  REP(i, 0, n) {
    pts[i] = PL(y[i], x[i]);
  }
  tot += solve(uf, n, pts, dist, 1, r);
  assert (tot % 2 == 0);
  cout << tot / 2 << endl;
}
