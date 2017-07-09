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


const int H = 210;

int in[H], out[H], inc[H], outc[H];

int main(void){
  int n, h;
  cin >> n >> h;
  set<int> app;
  UnionFind uf(2 * H);
  REP(i, 0, n) {
    int a, b, c, d;
    cin >> a >> b >> c >> d;
    int u, v;
    if (c == 0) {
      in[a] += 1;
      app.insert(u = a);
    } else {
      inc[c] += 1;
      app.insert(u = c + H);
    }
    if (d == 0) {
      out[b] += 1;
      app.insert(v = b + H);
    } else {
      outc[d] += 1;
      app.insert(v = d);
    }
    uf.unite(u, v);
  }
  set<int> roots;
  for (set<int>::iterator it = app.begin(); it != app.end(); ++it) {
    roots.insert(uf.root(*it));
  }
  bool ok = true;
  int ncc = roots.size();
  int cnt = 0;
  REP(i, 0, h + 1) {
    if (0) {
      cerr << i << ":";
      cerr << "in=" << in[i] << ", out=" << out[i];
      cerr << "inc=" << inc[i] << ", outc=" << outc[i];
      cerr << endl;
    }
    if (in[i] != outc[i]) {
      cnt += in[i] - outc[i];
      if (in[i] < outc[i]) {
	ok = false;
      }
    }
    if (out[i] != inc[i]) {
      cnt += out[i] - inc[i];
      if (out[i] < inc[i]) {
	ok = false;
      }
    }
  }
  assert (cnt % 2 == 0);
  ok &= cnt % 2 == 0 && cnt >= 2 * ncc;
  cout << (ok ? "YES" : "NO") << endl;
}
