#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
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


int main(void){
  int n, m, k;
  cin >> n >> m >> k;
  VI gov(k);
  REP(i, 0, k) {
    cin >> gov[i];
    gov[i]--;
  }
  UnionFind uf(n);
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    uf.unite(u, v);
  }
  int tot_v = 0;
  int tot_e = 0;
  VI vs;
  VI qf(n, 0);
  REP(i, 0, n) {
    qf[uf.root(i)]++;
  }
  REP(i, 0, k) {
    int ri = uf.root(gov[i]);
    tot_v += qf[ri];
    vs.push_back(qf[ri]);
  }
  sort(vs.rbegin(), vs.rend());
  vs[0] += n - tot_v;
  REP(i, 0, vs.size()) {
    int r = vs[i];
    tot_e += r * (r - 1) / 2;
  }
  cout << tot_e - m << endl;
}
