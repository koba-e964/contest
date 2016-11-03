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

const int N = 110;
int tbl[N][N];

int main(void){
  int n;
  for (;scanf("%d",&n) >= 1;) {
    vector<PI> edges;
    REP(i, 0, n) {
      REP(j, 0, n) {
	scanf("%d",&tbl[i][j]);
	if (i < j) {
	  edges.push_back(PI(tbl[i][j], i * n + j));
	}
      }
    }
    sort(edges.begin(), edges.end());
    UnionFind uf(n);
    ll tot = 0;
    REP(i, 0, edges.size()) {
      PI e = edges[i];
      int uv = e.second;
      int u = uv/n, v = uv%n;
      if (not uf.is_same_set(u, v)) {
	uf.unite(u, v);
	tot += e.first;
      }
    }
    printf("%lld\n", tot);
  }
}
