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


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  int w, h;
  while ((cin >> w >> h) && (w || h)) {
    vector<VI> mp(h, VI(w, 0));
    REP(i, 0, h) {
      REP(j, 0, w) {
	cin >> mp[i][j];
      }
    }
    UnionFind uf(w * h);
    REP(i, 0, h) {
      REP(j, 0, w) {
	if (mp[i][j] == 0) { continue; }
	REP(dx, -1, 2) {
	  REP(dy, -1, 2) {
	    if (dx == 0 && dy == 0) { continue; }
	    int nx = i + dx;
	    int ny = j + dy;
	    if (0 <= nx && nx < h && 0 <= ny && ny < w && mp[nx][ny]) {
	      // unite
	      uf.unite(i * w + j, nx * w + ny);
	    }
	  }
	}
      }
    }
    set<int> roots;
    REP(i, 0, h * w) {
      int x = i / w;
      int y = i % w;
      if (mp[x][y]) {
	roots.insert(uf.root(i));
      }
    }
    cout << roots.size() << endl;
  }
}
