#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>

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

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, PI> PLPI;

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<PLPI> e;
  REP(i, 0, m) {
    int u, v, c;
    cin >> u >> v >> c;
    u--, v--;
    e.push_back(PLPI(c, PI(u, v)));
  }
  if (n == 1) {
    cout << "0 0" << endl;
    return 0;
  }
  sort(e.begin(), e.end());
  UnionFind uf(n);
  VL uu;
  ll tot = 0;
  REP(i, 0, m) {
    PLPI ee = e[i];
    int u = ee.second.first;
    int v = ee.second.second;
    ll c = ee.first;
    if (uf.is_same_set(u, v)) continue;
    uu.push_back(c);
    tot += c;
    uf.unite(u, v);
  }
  ll ma = uu[uu.size() - 1];
  cout << tot - ma;
  UnionFind uf2(n);
  int cnt = 0;
  REP(i, 0, m) {
    PLPI ee = e[i];
    int u = ee.second.first;
    int v = ee.second.second;
    ll c = ee.first;
    if (uf2.is_same_set(u, v)) continue;
    if (c >= ma) {
      cnt += 1;
    } else {
      uu.push_back(c);
      tot += c;
      uf2.unite(u, v);
    }
  }
  cout << " " << cnt << endl;
}
