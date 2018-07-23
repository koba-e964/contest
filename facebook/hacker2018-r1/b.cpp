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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
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

void dfs(const VI &a, const VI &b, int v, VI &seq, int is_post) {
  if (not is_post) {
    seq.push_back(v);
  }
  if (a[v] >= 0) {
    dfs(a, b, a[v], seq, is_post);
  }
  if (b[v] >= 0) {
    dfs(a, b, b[v], seq, is_post);
  }
  if (is_post) {
    seq.push_back(v);
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    cout << "Case #" << case_nr << ":";
    int k, n;
    cin >> n >> k;
    VI a(n), b(n);
    REP(i, 0, n) {
      cin >> a[i] >> b[i];
      a[i]--, b[i]--;
    }
    VI pre, post;
    dfs(a, b, 0, pre, 0);
    dfs(a, b, 0, post, 1);
    UnionFind uf(n);
    REP(i, 0, n) {
      uf.unite(pre[i], post[i]);
    }
    set<int> rts;
    REP(i, 0, n) rts.insert(uf.root(i));
    if ((int) rts.size() < k) {
      cout << " Impossible";
    } else {
      map<int, int> tbl;
      int cnt = 1;
      for (int v: rts) {
	tbl[v] = cnt;
	cnt = min(cnt + 1, k);
      }
      VI ans(n);
      REP(i, 0, n) ans[i] = tbl[uf.root(i)];
      REP(i, 0, n) cout << " " << ans[i];
    }
    cout << "\n";
  }

}
