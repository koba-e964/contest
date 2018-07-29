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

const int N = 100100;
vector<PI> rel[N];

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
    if (rel[x].size() < rel[y].size()) swap(x, y);
    disj[y] = x;
    REP(i, 0, rel[y].size()) {
      PI v = rel[y][i];
      rel[x].push_back(v);
    }
    rel[y].clear();
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  UnionFind uf(n);
  set<PI> dir_edges;
  REP(i, 0, q) {
    int ty, u, v;
    cin >> ty >> u >> v;
    u--, v--;
    if (ty == 1) {
      if (v > u) swap(u, v);
      dir_edges.insert(PI(u, v));
      if (not uf.is_same_set(u, v)) {
	int ru = uf.root(u);
	int rv = uf.root(v);
	rel[ru].push_back(PI(u, v));
	rel[rv].push_back(PI(v, u));
      }
    } else if (ty == 2) {
      while (1) {
	int ru = uf.root(u);
	if (rel[ru].empty()) break;
	PI p = rel[ru].back(); rel[ru].pop_back();
	uf.unite(p.first, p.second);
      }
    } else {
      if (v > u) swap(u, v);
      if (uf.is_same_set(u, v) || dir_edges.count(PI(u, v))) {
	cout << "Yes" << endl;
      } else {
	cout << "No" << endl;
      }
    }
    if (DEBUG) {
      REP(i, 0, n) {
	cerr << "r = " << uf.root(i) << " ";
	cerr << "rel[" << i << "]:";
	REP(j, 0, rel[i].size()) {
	  cerr << " " << rel[i][j].first << "," << rel[i][j].second;
	}
	cerr << endl;
      }
    }
  }
}
