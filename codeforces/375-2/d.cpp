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

const int N = 52;
string s[N];
int pond[N][N];

int main(void){
  int n, m, k;
  cin >> n >> m >> k;
  REP(i, 0, n) {
    cin >> s[i];
  }
  UnionFind uf((n + 2) * (m + 2));
  REP(i, 0, m + 1) {
    uf.unite(i, i + 1);
    uf.unite((n + 1) * (m + 2) + i, (n + 1) * (m + 2) + i + 1);
  }
  REP(i, 0, n + 1) {
    uf.unite(i * (m + 2), (i + 1) * (m + 2));
    uf.unite(i * (m + 2) + m + 1, (i + 1) * (m + 2) + m + 1);
  }
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (s[i][j] != '.') {
	continue;
      }
      int dxy[5] = {0, 1, 0, -1, 0};
      REP(d, 0, 4) {
	int nx = i + dxy[d];
	int ny = j + dxy[d + 1];
	bool ok;
	if (nx < 0 || nx >= n || ny < 0 || ny >= m) {
	  ok = true;
	} else {
	  ok = s[nx][ny] == '.';
	}
	if (ok) {
	  uf.unite((i + 1) * (m + 2) + j + 1, (nx + 1) * (m + 2) + (ny + 1));
	}
      }
    }
  }
  vector<PI> acc((n + 2) * (m + 2));
  REP(i, 0, (n + 2) * (m + 2)) {
    acc[i].second = i;
  }
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (s[i][j] == '.') {
	acc[uf.root((i + 1) * (m + 2) + (j + 1))].first++;
      }
    }
  }
  acc[uf.root(0)].first = 0;
  sort(acc.begin(), acc.end());
  assert(k < 1 || acc[(n + 2) * (m + 2) - k].first > 0);
  int sum = 0;
  VI burial((n + 2) * (m + 2));
  REP(i, 0, (n + 2) * (m + 2) - k) {
    sum += acc[i].first;
    if (acc[i].first > 0) {
      burial[acc[i].second] = 1;
    }
  }
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (burial[uf.root((i + 1) * (m + 2) + j + 1)]) {
	s[i][j] = '*';
      }
    }
  }
  cout << sum << endl;
  REP(i, 0, n) {
    cout << s[i] << endl;
  }
}
